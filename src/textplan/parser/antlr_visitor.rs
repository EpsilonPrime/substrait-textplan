// SPDX-License-Identifier: Apache-2.0

//! ANTLR visitor implementation for Substrait textplans.
//!
//! This module contains visitor implementations that process
//! the ANTLR parse tree and build a symbol table following the
//! multiphase approach used in the C++ implementation.

use std::rc::Rc;
use std::sync::Arc;

use antlr_rust::parser_rule_context::ParserRuleContext;
use antlr_rust::rule_context::RuleContext;
use antlr_rust::token::{GenericToken, Token};
use antlr_rust::tree::{ParseTree, ParseTreeVisitor};
use antlr_rust::TidExt;

use crate::textplan::common::structured_symbol_data::RelationData;
use crate::textplan::common::text_location::TextLocation;
use crate::textplan::parser::antlr::substraitplanparser::RelationContextAttrs;
use crate::textplan::parser::antlr::substraitplanparser::*;
use crate::textplan::parser::antlr::substraitplanparservisitor::SubstraitPlanParserVisitor;
use crate::textplan::parser::error_listener::ErrorListener;
use crate::textplan::symbol_table::{RelationType, SymbolInfo, SymbolTable, SymbolType};
use ::substrait::proto::r#type::{
    Binary, Boolean, Date, Decimal, FixedBinary, FixedChar, Fp32, Fp64, IntervalDay, IntervalYear,
    Kind, List, Map, Nullability, String as StringType, Struct, Time, Timestamp, TimestampTz, Uuid,
    VarChar, I16, I32, I64, I8,
};
use ::substrait::proto::{rel::RelType, Rel, Type};
use std::any::Any;
use std::sync::Mutex;

/// Helper function to convert ANTLR token to TextLocation
pub fn token_to_location<'a>(
    token: &impl std::ops::Deref<Target = GenericToken<std::borrow::Cow<'a, str>>>,
) -> TextLocation {
    // Convert token position to an absolute position
    // Use both line and column to create a unique position
    // Position = (line * 10000) + column to ensure different lines have different positions
    let position = (token.line as i32 * 10000) + (token.column as i32);
    let length = token.get_text().len() as i32;
    TextLocation::new(position, length)
}

/// Helper function to extract string content by removing quotes.
/// Removes leading and trailing quotation marks from a string.
pub fn extract_from_string(s: &str) -> String {
    if s.len() < 2 {
        return s.to_string();
    }

    let mut result = s.to_string();

    // Remove trailing quote if present
    if result.ends_with('"') {
        result.pop();
    }

    // Remove leading quote if present
    if result.starts_with('"') {
        result.remove(0);
    }

    result
}

/// Helper function to safely apply a visitor to a parse tree node.
///
/// This function handles the common pattern of applying a visitor to a parse tree node,
/// properly managing the lifetimes and ownership.
pub fn visit_parse_tree<'input, V>(visitor: &mut V, context: &dyn antlr_rust::tree::Visitable<V>)
where
    V: SubstraitPlanParserVisitor<'input>,
{
    // Call the accept method on the context to apply the visitor
    context.accept(visitor);
}

/// Helper function to safely apply a visitor to a plan context.
///
/// This specializes the visit_parse_tree function for the plan rule.
pub fn visit_plan<'input, V>(
    visitor: &mut V,
    context: &crate::textplan::parser::antlr::substraitplanparser::PlanContext<'input>,
) where
    V: SubstraitPlanParserVisitor<'input>,
{
    println!("Visiting plan node");

    // Use antlr_rust::tree::Visitable trait to access the accept method
    use antlr_rust::tree::Visitable;
    context.accept(visitor);
}

/// Pre-scans the parse tree to mark all subqueries before building expressions.
///
/// This function creates a specialized visitor that only looks for SUBQUERY keywords
/// and marks those relations, without building full expressions. This ensures that
/// parent_query_info is set before expressions are built, enabling proper outer
/// reference detection.
pub fn prescan_subqueries<'input>(
    relation_visitor: &mut RelationVisitor<'input>,
    context: &crate::textplan::parser::antlr::substraitplanparser::PlanContext<'input>,
) {
    // Set a flag to indicate we're in prescan mode
    relation_visitor.set_prescan_mode(true);

    // Visit the tree to find and mark subqueries
    use antlr_rust::tree::Visitable;
    context.accept(relation_visitor);

    // Clear prescan mode
    relation_visitor.set_prescan_mode(false);

    println!("  Prescan complete - subqueries marked");
}

/// Base trait for all ANTLR-based Substrait plan visitors.
pub trait PlanVisitor<'input> {
    /// Gets the error listener for this visitor.
    fn error_listener(&self) -> Arc<ErrorListener>;

    /// Gets the symbol table for this visitor.
    fn symbol_table(&self) -> SymbolTable;
}

/// Base implementation for ANTLR-based Substrait plan visitors.
///
/// This provides common functionality for all visitor implementations.
pub struct BasePlanVisitor {
    symbol_table: SymbolTable,
    error_listener: Arc<ErrorListener>,
}

impl BasePlanVisitor {
    /// Creates a new base plan visitor.
    pub fn new(symbol_table: SymbolTable, error_listener: Arc<ErrorListener>) -> Self {
        Self {
            symbol_table,
            error_listener,
        }
    }

    /// Gets a mutable reference to the symbol table for modifications.
    pub fn symbol_table_mut(&mut self) -> &mut SymbolTable {
        &mut self.symbol_table
    }
}

impl<'input> PlanVisitor<'input> for BasePlanVisitor {
    fn error_listener(&self) -> Arc<ErrorListener> {
        self.error_listener.clone()
    }

    fn symbol_table(&self) -> SymbolTable {
        self.symbol_table.clone()
    }
}

/// The TypeVisitor processes and validates types in the parse tree.
///
/// This visitor is the first phase in the multiphase parsing approach, focusing on
/// basic and complex types. It builds on the BasePlanVisitor to provide common
/// symbol table and error handling functionality.
pub struct TypeVisitor<'input> {
    base: BasePlanVisitor,
    _phantom: std::marker::PhantomData<&'input ()>,
}

impl<'input> TypeVisitor<'input> {
    /// Creates a new TypeVisitor.
    pub fn new(symbol_table: SymbolTable, error_listener: Arc<ErrorListener>) -> Self {
        Self {
            base: BasePlanVisitor::new(symbol_table, error_listener),
            _phantom: std::marker::PhantomData,
        }
    }

    /// Converts a text representation of a type to a Substrait protobuf Type.
    /// This follows the C++ substrait type library logic for handling nullable markers.
    pub fn text_to_type_proto(
        &self,
        ctx: &dyn SubstraitPlanParserContext<'input>,
        type_text: &str,
    ) -> Type {
        let mut proto_type = Type::default();

        // Find positions of special characters
        let question_pos = type_text.find('?');
        let left_angle_pos = type_text.find('<');

        // Determine nullability following C++ logic:
        // - For parameterized types: check if ? appears immediately before <
        // - For simple types: check if last character is ?
        let nullable = if let Some(angle_pos) = left_angle_pos {
            // Parameterized type case: "decimal?<19,0>"
            angle_pos > 0 && question_pos == Some(angle_pos - 1)
        } else {
            // Simple type case: "i32?"
            question_pos == Some(type_text.len() - 1)
        };

        // Extract base type name (without the ? marker)
        let base_type_name = if nullable {
            if let Some(q_pos) = question_pos {
                &type_text[..q_pos]
            } else {
                type_text
            }
        } else {
            type_text
        };

        // Reconstruct the type string for parsing (base name + parameters)
        // For "decimal?<19,0>" this becomes "decimal<19,0>"
        // For "i32?" this becomes "i32"
        let base_type_str = if let Some(angle_pos) = left_angle_pos {
            if nullable {
                // Concatenate base name + everything from < onwards
                format!("{}{}", base_type_name, &type_text[angle_pos..])
            } else {
                type_text.to_string()
            }
        } else {
            base_type_name.to_string()
        };

        let nullability = if nullable {
            Nullability::Nullable
        } else {
            Nullability::Required
        };

        // Parse complex types
        if let Some(list_content) = base_type_str
            .strip_prefix("list<")
            .and_then(|s| s.strip_suffix(">"))
        {
            // List type - format: list<element_type>
            let element_type = self.text_to_type_proto(ctx, list_content);
            let mut list_type = List::default();
            list_type.nullability = nullability.into();
            list_type.r#type = Some(Box::new(element_type));
            proto_type.kind = Some(Kind::List(Box::new(list_type)));
            return proto_type;
        } else if let Some(struct_content) = base_type_str
            .strip_prefix("struct<")
            .and_then(|s| s.strip_suffix(">"))
        {
            // Struct type - format: struct<field1_type, field2_type, ...>
            let mut struct_type = Struct::default();
            struct_type.nullability = nullability.into();

            // Split the struct content by commas, respecting nesting
            let field_types = self.split_struct_fields(struct_content);

            for field_type_str in field_types {
                let field_type = self.text_to_type_proto(ctx, field_type_str.trim());
                struct_type.types.push(field_type);
            }

            proto_type.kind = Some(Kind::Struct(struct_type));
            return proto_type;
        } else if let Some(map_content) = base_type_str
            .strip_prefix("map<")
            .and_then(|s| s.strip_suffix(">"))
        {
            // Map type - format: map<key_type, value_type>
            if let Some((key_type_str, value_type_str)) = map_content.split_once(',') {
                let key_type = self.text_to_type_proto(ctx, key_type_str.trim());
                let value_type = self.text_to_type_proto(ctx, value_type_str.trim());

                let mut map_type = Map::default();
                map_type.nullability = nullability.into();
                map_type.key = Some(Box::new(key_type));
                map_type.value = Some(Box::new(value_type));

                proto_type.kind = Some(Kind::Map(Box::new(map_type)));
                return proto_type;
            } else {
                // Use the start() method properly
                // Get the start token directly - it's not an Option
                let token = ctx.start();
                self.add_error(&token, &format!("Invalid map type format: {}", type_text));
            }
        } else if let Some(decimal_content) = base_type_str
            .strip_prefix("decimal<")
            .and_then(|s| s.strip_suffix(">"))
        {
            // Decimal type - format: decimal<precision, scale>
            if let Some((precision_str, scale_str)) = decimal_content.split_once(',') {
                if let (Ok(precision), Ok(scale)) = (
                    precision_str.trim().parse::<i32>(),
                    scale_str.trim().parse::<i32>(),
                ) {
                    let mut decimal_type = Decimal::default();
                    decimal_type.nullability = nullability.into();
                    decimal_type.precision = precision;
                    decimal_type.scale = scale;

                    proto_type.kind = Some(Kind::Decimal(decimal_type));
                    return proto_type;
                } else {
                    // Get the start token directly - it's not an Option
                    let token = ctx.start();
                    self.add_error(
                        &token,
                        &format!("Invalid decimal parameters: {}", decimal_content),
                    );
                }
            } else {
                // Get the start token directly - it's not an Option
                let token = ctx.start();
                self.add_error(
                    &token,
                    &format!("Invalid decimal type format: {}", type_text),
                );
            }
        } else if let Some(fixed_char_content) = base_type_str
            .strip_prefix("fixedchar<")
            .and_then(|s| s.strip_suffix(">"))
        {
            // Fixed char type - format: fixedchar<length>
            if let Ok(length) = fixed_char_content.trim().parse::<i32>() {
                let mut fixed_char_type = FixedChar::default();
                fixed_char_type.nullability = nullability.into();
                fixed_char_type.length = length;

                proto_type.kind = Some(Kind::FixedChar(fixed_char_type));
                return proto_type;
            } else {
                // Get the start token directly - it's not an Option
                let token = ctx.start();
                self.add_error(
                    &token,
                    &format!("Invalid fixedchar length: {}", fixed_char_content),
                );
            }
        } else if let Some(varchar_content) = base_type_str
            .strip_prefix("varchar<")
            .and_then(|s| s.strip_suffix(">"))
        {
            // Varchar type - format: varchar<length>
            if let Ok(length) = varchar_content.trim().parse::<i32>() {
                let mut varchar_type = VarChar::default();
                varchar_type.nullability = nullability.into();
                varchar_type.length = length;

                proto_type.kind = Some(Kind::Varchar(varchar_type));
                return proto_type;
            } else {
                // Get the start token directly - it's not an Option
                let token = ctx.start();
                self.add_error(
                    &token,
                    &format!("Invalid varchar length: {}", varchar_content),
                );
            }
        } else if let Some(fixed_binary_content) = base_type_str
            .strip_prefix("fixedbinary<")
            .and_then(|s| s.strip_suffix(">"))
        {
            // Fixed binary type - format: fixedbinary<length>
            if let Ok(length) = fixed_binary_content.trim().parse::<i32>() {
                let mut fixed_binary_type = FixedBinary::default();
                fixed_binary_type.nullability = nullability.into();
                fixed_binary_type.length = length;

                proto_type.kind = Some(Kind::FixedBinary(fixed_binary_type));
                return proto_type;
            } else {
                // Get the start token directly - it's not an Option
                let token = ctx.start();
                self.add_error(
                    &token,
                    &format!("Invalid fixedbinary length: {}", fixed_binary_content),
                );
            }
        }

        // Handle basic types
        match base_type_str.as_str() {
            "boolean" | "bool" => {
                let mut boolean = Boolean::default();
                boolean.nullability = nullability.into();
                proto_type.kind = Some(Kind::Bool(boolean));
            }
            "i8" | "int8" | "tinyint" => {
                let mut i8_type = I8::default();
                i8_type.nullability = nullability.into();
                proto_type.kind = Some(Kind::I8(i8_type));
            }
            "i16" | "int16" | "smallint" => {
                let mut i16_type = I16::default();
                i16_type.nullability = nullability.into();
                proto_type.kind = Some(Kind::I16(i16_type));
            }
            "i32" | "int32" | "int" => {
                let mut i32_type = I32::default();
                i32_type.nullability = nullability.into();
                proto_type.kind = Some(Kind::I32(i32_type));
            }
            "i64" | "int64" | "bigint" => {
                let mut i64_type = I64::default();
                i64_type.nullability = nullability.into();
                proto_type.kind = Some(Kind::I64(i64_type));
            }
            "fp32" | "float" => {
                let mut fp32_type = Fp32::default();
                fp32_type.nullability = nullability.into();
                proto_type.kind = Some(Kind::Fp32(fp32_type));
            }
            "fp64" | "double" => {
                let mut fp64_type = Fp64::default();
                fp64_type.nullability = nullability.into();
                proto_type.kind = Some(Kind::Fp64(fp64_type));
            }
            "string" => {
                let mut string_type = StringType::default();
                string_type.nullability = nullability.into();
                proto_type.kind = Some(Kind::String(string_type));
            }
            "binary" => {
                let mut binary_type = Binary::default();
                binary_type.nullability = nullability.into();
                proto_type.kind = Some(Kind::Binary(binary_type));
            }
            "timestamp" => {
                let mut timestamp_type = Timestamp::default();
                timestamp_type.nullability = nullability.into();
                proto_type.kind = Some(Kind::Timestamp(timestamp_type));
            }
            "timestamp_tz" => {
                let mut timestamp_tz_type = TimestampTz::default();
                timestamp_tz_type.nullability = nullability.into();
                proto_type.kind = Some(Kind::TimestampTz(timestamp_tz_type));
            }
            "date" => {
                let mut date_type = Date::default();
                date_type.nullability = nullability.into();
                proto_type.kind = Some(Kind::Date(date_type));
            }
            "time" => {
                let mut time_type = Time::default();
                time_type.nullability = nullability.into();
                proto_type.kind = Some(Kind::Time(time_type));
            }
            "interval_year" | "interval_year_to_month" | "interval_year_month" => {
                let mut interval_year_type = IntervalYear::default();
                interval_year_type.nullability = nullability.into();
                proto_type.kind = Some(Kind::IntervalYear(interval_year_type));
            }
            "interval_day" | "interval_day_to_second" | "interval_day_second" => {
                let mut interval_day_type = IntervalDay::default();
                interval_day_type.nullability = nullability.into();
                proto_type.kind = Some(Kind::IntervalDay(interval_day_type));
            }
            "uuid" => {
                let mut uuid_type = Uuid::default();
                uuid_type.nullability = nullability.into();
                proto_type.kind = Some(Kind::Uuid(uuid_type));
            }
            "fixedchar" => {
                // Bare fixedchar without length parameter (used in literal suffixes like "text"_fixedchar)
                // We don't know the length from the literal alone, so use length 0 as a placeholder
                let mut fixed_char_type = FixedChar::default();
                fixed_char_type.nullability = nullability.into();
                fixed_char_type.length = 0;
                proto_type.kind = Some(Kind::FixedChar(fixed_char_type));
            }
            "varchar" => {
                // Bare varchar without length parameter (used in literal suffixes like "text"_varchar)
                let mut varchar_type = VarChar::default();
                varchar_type.nullability = nullability.into();
                varchar_type.length = 0;
                proto_type.kind = Some(Kind::Varchar(varchar_type));
            }
            // For unknown types, log an error and use i32 as a fallback
            _ => {
                // Get the start token directly - it's not an Option
                let token = ctx.start();
                self.add_error(
                    &token,
                    &format!("Unsupported or unknown type: {}", type_text),
                );
                // Set a default type for now
                let mut unknown_type = I32::default();
                unknown_type.nullability = nullability.into();
                proto_type.kind = Some(Kind::I32(unknown_type));
            }
        }

        proto_type
    }

    /// Helper function to split struct fields, respecting nested angle brackets.
    fn split_struct_fields(&self, struct_content: &str) -> Vec<String> {
        let mut result = Vec::new();
        let mut current_field = String::new();
        let mut angle_bracket_depth = 0;

        for c in struct_content.chars() {
            match c {
                '<' => {
                    angle_bracket_depth += 1;
                    current_field.push(c);
                }
                '>' => {
                    angle_bracket_depth -= 1;
                    current_field.push(c);
                }
                ',' if angle_bracket_depth == 0 => {
                    // Only split at commas that are not inside angle brackets
                    result.push(current_field.trim().to_string());
                    current_field.clear();
                }
                _ => {
                    current_field.push(c);
                }
            }
        }

        // Add the last field if there is one
        if !current_field.trim().is_empty() {
            result.push(current_field.trim().to_string());
        }

        result
    }

    /// Determines if the context is inside a struct literal with an external type.
    pub fn inside_struct_literal_with_external_type(
        &self,
        _ctx: &dyn SubstraitPlanParserContext<'input>,
    ) -> bool {
        // This would check up the parse tree to determine context
        // For now, default to false
        false
    }

    /// Adds an error message to the error listener.
    pub fn add_error<'a>(
        &self,
        token: &impl std::ops::Deref<Target = GenericToken<std::borrow::Cow<'a, str>>>,
        message: &str,
    ) {
        let location = token_to_location(token);
        self.base
            .error_listener()
            .add_error(message.to_string(), location);
    }
}

impl<'input> PlanVisitor<'input> for TypeVisitor<'input> {
    fn error_listener(&self) -> Arc<ErrorListener> {
        self.base.error_listener()
    }

    fn symbol_table(&self) -> SymbolTable {
        self.base.symbol_table()
    }
}

// ANTLR visitor implementation for TypeVisitor
impl<'input> ParseTreeVisitor<'input, SubstraitPlanParserContextType> for TypeVisitor<'input> {}

impl<'input> SubstraitPlanParserVisitor<'input> for TypeVisitor<'input> {
    // Override specific visitor methods for literal types
    fn visit_literal_basic_type(&mut self, ctx: &Literal_basic_typeContext<'input>) {
        // In a real implementation, we would process this and store the result
        // For now, we'll just process the type text - using Token trait's get_text method
        let type_text = ctx.get_text();
        let _type_proto = self.text_to_type_proto(ctx, &type_text);

        // Visit children nodes in case there are nested types
        self.visit_children(ctx);
    }

    fn visit_literal_complex_type(&mut self, ctx: &Literal_complex_typeContext<'input>) {
        // In a real implementation, we would process this and store the result
        // For now, we'll just process the type text
        let type_text = ctx.get_text();
        let _type_proto = self.text_to_type_proto(ctx, &type_text);

        // Visit children nodes in case there are nested types
        self.visit_children(ctx);
    }

    // We use the default implementation for other visitor methods,
    // which will call visit_children to traverse the entire tree
}

// Forward declarations of the visitor implementations that will be added
// after the TypeVisitor is fully implemented.

/// The PlanVisitor processes the top-level plan structure.
///
/// This visitor is the second phase in the multiphase parsing approach,
/// building on the TypeVisitor to handle plan-level structures.
pub struct MainPlanVisitor<'input> {
    type_visitor: TypeVisitor<'input>,
    current_relation_scope: Option<Arc<SymbolInfo>>, // Use actual SymbolInfo
    current_source_scope: Option<Arc<SymbolInfo>>,   // Track current source being processed
    current_extension_space: Option<Arc<SymbolInfo>>, // Track current extension space
    num_spaces_seen: i32,
    num_functions_seen: i32,
}

impl<'input> MainPlanVisitor<'input> {
    /// Creates a new MainPlanVisitor.
    pub fn new(symbol_table: SymbolTable, error_listener: Arc<ErrorListener>) -> Self {
        Self {
            type_visitor: TypeVisitor::new(symbol_table, error_listener.clone()),
            current_relation_scope: None,
            current_source_scope: None,
            current_extension_space: None,
            num_spaces_seen: 0,
            num_functions_seen: 0,
        }
    }

    /// Gets the current relation scope, if any.
    pub fn current_relation_scope(&self) -> Option<&Arc<SymbolInfo>> {
        self.current_relation_scope.as_ref()
    }

    /// Sets the current relation scope.
    pub fn set_current_relation_scope(&mut self, scope: Option<Arc<SymbolInfo>>) {
        self.current_relation_scope = scope;
    }

    /// Gets the current source scope, if any.
    pub fn current_source_scope(&self) -> Option<&Arc<SymbolInfo>> {
        self.current_source_scope.as_ref()
    }

    /// Sets the current source scope.
    pub fn set_current_source_scope(&mut self, scope: Option<Arc<SymbolInfo>>) {
        self.current_source_scope = scope;
    }

    /// Gets the error listener for this visitor.
    pub fn get_error_listener(&self) -> Arc<ErrorListener> {
        self.type_visitor.error_listener()
    }

    /// Gets the symbol table for this visitor.
    pub fn get_symbol_table(&self) -> SymbolTable {
        self.type_visitor.symbol_table()
    }

    /// Adds an error message to the error listener.
    pub fn add_error<'a>(
        &self,
        token: &impl std::ops::Deref<Target = GenericToken<std::borrow::Cow<'a, str>>>,
        message: &str,
    ) {
        self.type_visitor.add_error(token, message);
    }

    /// Process an extension space and add it to the symbol table.
    fn process_extension_space(
        &mut self,
        ctx: &ExtensionspaceContext<'input>,
    ) -> Option<Arc<SymbolInfo>> {
        // Get the name of the extension space from the URI token if present
        // If no URI, create an unnamed extension space
        let (name, location) = if let Some(uri_token) = ctx.URI() {
            let name = uri_token.get_text().trim().to_string();
            let location = token_to_location(&uri_token.symbol);
            (name, location)
        } else {
            // No URI provided, use a default name and location from the context
            let token = ctx.start();
            let location = token_to_location(&token);
            ("unnamed_extension_space".to_string(), location)
        };

        // Assign an anchor for this extension space (incrementing counter)
        let anchor = self.num_spaces_seen as u32;

        // Create ExtensionSpaceData blob
        let extension_space_data =
            crate::textplan::common::structured_symbol_data::ExtensionSpaceData::new(anchor);
        let blob = Some(Arc::new(std::sync::Mutex::new(extension_space_data))
            as Arc<std::sync::Mutex<dyn std::any::Any + Send + Sync>>);

        // Define the extension space in the symbol table
        let symbol = self.type_visitor.base.symbol_table_mut().define_symbol(
            name,
            location,
            SymbolType::ExtensionSpace,
            None, // subtype
            blob, // blob
        );

        println!(
            "  Defined extension space '{}' with anchor {}",
            symbol.name(),
            anchor
        );

        Some(symbol)
    }

    /// Process a function definition and add it to the symbol table.
    fn process_function(&mut self, ctx: &FunctionContext<'input>) -> Option<Arc<SymbolInfo>> {
        // Get the function signature (e.g., "multiply:dec_dec")
        let full_name = ctx.name()?.get_text();

        // Get the alias (the name after "AS", e.g., "multiply")
        // If no alias, use the function name before the colon
        let alias = if let Some(id_ctx) = ctx.id() {
            id_ctx.get_text()
        } else {
            // No alias - use function name before colon
            full_name
                .split(':')
                .next()
                .unwrap_or(&full_name)
                .to_string()
        };

        // Create a location from the context's start token
        let token = ctx.start();
        let location = token_to_location(&token);

        // Assign an anchor for this function (incrementing counter)
        let anchor = self.num_functions_seen as u32;

        // Get extension_uri_reference from current extension space
        let extension_uri_reference = if let Some(ext_space) = &self.current_extension_space {
            // Get the anchor from the extension space blob
            if let Some(blob_lock) = &ext_space.blob {
                if let Ok(blob_data) = blob_lock.lock() {
                    if let Some(ext_data) = blob_data.downcast_ref::<crate::textplan::common::structured_symbol_data::ExtensionSpaceData>() {
                        Some(ext_data.anchor_reference())
                    } else {
                        None
                    }
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
        };

        // Create FunctionData blob
        let function_data = crate::textplan::common::structured_symbol_data::FunctionData::new(
            full_name.clone(),
            extension_uri_reference,
            anchor,
        );
        let blob = Some(Arc::new(std::sync::Mutex::new(function_data))
            as Arc<std::sync::Mutex<dyn std::any::Any + Send + Sync>>);

        // Define the function in the symbol table with the alias as the name
        let symbol = self.type_visitor.base.symbol_table_mut().define_symbol(
            alias,
            location,
            SymbolType::Function,
            None, // subtype
            blob, // blob
        );

        println!(
            "  Defined function '{}' (alias '{}') with anchor {}, extension_uri_ref {:?}",
            full_name,
            symbol.name(),
            anchor,
            extension_uri_reference
        );

        Some(symbol)
    }

    /// Process a source definition and add it to the symbol table.
    fn process_source_definition(
        &mut self,
        ctx: &Source_definitionContext<'input>,
    ) -> Option<Arc<SymbolInfo>> {
        use crate::textplan::parser::antlr::substraitplanparser::Read_typeContextAll;

        // Get the read_type which can be NamedTable, LocalFiles, VirtualTable, or ExtensionTable
        let read_type_ctx = ctx.read_type()?;

        // Match on the specific read type variant
        match read_type_ctx.as_ref() {
            Read_typeContextAll::NamedTableContext(named_table_ctx) => {
                // Get the id (source name) from the named table context
                let name = named_table_ctx.id()?.get_text();

                // Create a location from the context's start token
                let token = ctx.start();
                let location = token_to_location(&token);

                // Define the source in the symbol table
                let symbol = self.type_visitor.base.symbol_table_mut().define_symbol(
                    name,
                    location,
                    SymbolType::Source,
                    None,
                    None,
                );

                Some(symbol)
            }
            _ => {
                // For other source types, we don't process them yet
                None
            }
        }
    }

    /// Process named table details and add string symbols to the symbol table.
    fn process_named_table_detail(
        &mut self,
        ctx: &Named_table_detailContext<'input>,
        parent_source: &Arc<SymbolInfo>,
    ) -> Option<()> {
        // Get all STRING tokens from the context
        let strings = ctx.STRING_all();

        for string_token in strings {
            // Get the text of the string token
            let text = string_token.get_text();

            // Extract the string content (remove quotes)
            let name = extract_from_string(&text);

            // Create a location from the string token
            let location = token_to_location(&string_token.symbol);

            // Define the source detail in the symbol table
            let symbol = self.type_visitor.base.symbol_table_mut().define_symbol(
                name,
                location,
                SymbolType::SourceDetail,
                None,
                None,
            );

            // Set the source as the parent (similar to SchemaColumn→Schema)
            symbol.set_source(parent_source.clone());
        }

        Some(())
    }

    /// Add input fields from continuing pipeline and new pipelines to the current relation's field_references.
    /// For READ relations, populates from the schema. For other relations, populates from input pipelines.
    /// This recursively populates upstream relations first to ensure fields are available.
    fn add_input_fields_to_schema(&mut self, relation_symbol: &Arc<SymbolInfo>) {
        println!(
            "    add_input_fields_to_schema called for '{}'",
            relation_symbol.name()
        );

        // Check if already populated (early return to avoid unnecessary work)
        if let Some(blob_lock) = &relation_symbol.blob {
            if let Ok(blob_data) = blob_lock.lock() {
                if let Some(relation_data) = blob_data.downcast_ref::<RelationData>() {
                    if !relation_data.field_references.is_empty() {
                        println!(
                            "      '{}' already has {} field_references, skipping",
                            relation_symbol.name(),
                            relation_data.field_references.len()
                        );
                        return; // Already populated
                    }
                }
            }
        }

        // Recursively populate upstream relations first
        if let Some(blob_lock) = &relation_symbol.blob {
            if let Ok(blob_data) = blob_lock.lock() {
                if let Some(relation_data) = blob_data.downcast_ref::<RelationData>() {
                    // Collect upstream relations to populate (need to drop lock before recursing)
                    let mut upstreams = Vec::new();
                    if let Some(cont) = &relation_data.continuing_pipeline {
                        upstreams.push(cont.clone());
                    }
                    for pipe in &relation_data.new_pipelines {
                        upstreams.push(pipe.clone());
                    }
                    drop(blob_data);
                    drop(blob_lock);

                    // Recursively populate upstreams
                    for upstream in upstreams {
                        self.add_input_fields_to_schema(&upstream);
                    }
                }
            }
        }

        // Now populate this relation's field_references from its (now-populated) upstreams
        if let Some(blob_lock) = &relation_symbol.blob {
            if let Ok(mut blob_data) = blob_lock.lock() {
                if let Some(relation_data) = blob_data.downcast_mut::<RelationData>() {
                    // Check if this is a READ relation
                    if let Some(RelType::Read(_)) = &relation_data.relation.rel_type {
                        println!(
                            "      '{}' is READ, populating from schema",
                            relation_symbol.name()
                        );
                        // For READ relations, populate field_references from the schema
                        if let Some(schema_arc) = &relation_data.schema {
                            println!("        Schema: '{}'", schema_arc.name());
                            for symbol in self.symbol_table().symbols() {
                                if symbol.symbol_type() == SymbolType::SchemaColumn {
                                    if let Some(symbol_schema) = symbol.schema() {
                                        if Arc::ptr_eq(&symbol_schema, schema_arc) {
                                            println!("          Adding field: '{}'", symbol.name());
                                            relation_data.field_references.push(symbol.clone());
                                        }
                                    }
                                }
                            }
                            println!(
                                "        Total fields added: {}",
                                relation_data.field_references.len()
                            );
                        } else {
                            println!("        No schema found!");
                        }
                        return;
                    }

                    // For non-READ relations, add fields from continuing pipeline
                    if let Some(continuing_pipeline) = &relation_data.continuing_pipeline {
                        if let Some(cont_blob_lock) = &continuing_pipeline.blob {
                            if let Ok(cont_blob_data) = cont_blob_lock.lock() {
                                if let Some(cont_relation_data) =
                                    cont_blob_data.downcast_ref::<RelationData>()
                                {
                                    println!("      Upstream '{}': output_field_refs={}, field_refs={}, generated={}",
                                        continuing_pipeline.name(),
                                        cont_relation_data.output_field_references.len(),
                                        cont_relation_data.field_references.len(),
                                        cont_relation_data.generated_field_references.len());

                                    if !cont_relation_data.output_field_references.is_empty() {
                                        println!(
                                            "        Using output_field_references ({} fields)",
                                            cont_relation_data.output_field_references.len()
                                        );
                                        for field in &cont_relation_data.output_field_references {
                                            relation_data.field_references.push(field.clone());
                                        }
                                    } else {
                                        println!("        Using field_references + generated ({} + {} fields)",
                                            cont_relation_data.field_references.len(),
                                            cont_relation_data.generated_field_references.len());
                                        for field in &cont_relation_data.field_references {
                                            relation_data.field_references.push(field.clone());
                                        }
                                        for field in &cont_relation_data.generated_field_references
                                        {
                                            relation_data.field_references.push(field.clone());
                                        }
                                    }
                                }
                            }
                        }
                    }

                    // Add fields from new pipelines (e.g., for joins)
                    for pipeline in &relation_data.new_pipelines.clone() {
                        if let Some(pipe_blob_lock) = &pipeline.blob {
                            if let Ok(pipe_blob_data) = pipe_blob_lock.lock() {
                                if let Some(pipe_relation_data) =
                                    pipe_blob_data.downcast_ref::<RelationData>()
                                {
                                    if !pipe_relation_data.output_field_references.is_empty() {
                                        for field in &pipe_relation_data.output_field_references {
                                            relation_data.field_references.push(field.clone());
                                        }
                                    } else {
                                        for field in &pipe_relation_data.field_references {
                                            relation_data.field_references.push(field.clone());
                                        }
                                        for field in &pipe_relation_data.generated_field_references
                                        {
                                            relation_data.field_references.push(field.clone());
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    /// Process a relation and add it to the symbol table.
    fn process_relation(&mut self, ctx: &RelationContext<'input>) -> Option<Arc<SymbolInfo>> {
        // Get the name from the relation_ref's first id
        let relation_ref = ctx.relation_ref()?;
        let name = relation_ref.id(0)?.get_text();

        // Create a location from the context's start token
        // Get the start token directly - it's not an Option
        let token = ctx.start();
        let location = token_to_location(&token);

        // Determine the relation type and create corresponding Rel protobuf
        let (relation_type, rel_protobuf) = if let Some(relation_type_ctx) = ctx.relation_type() {
            let type_text = relation_type_ctx.get_text().to_lowercase();
            match type_text.as_str() {
                "read" => (
                    RelationType::Read,
                    Rel {
                        rel_type: Some(RelType::Read(Box::new(
                            ::substrait::proto::ReadRel::default(),
                        ))),
                    },
                ),
                "project" => (
                    RelationType::Project,
                    Rel {
                        rel_type: Some(RelType::Project(Box::new(
                            ::substrait::proto::ProjectRel::default(),
                        ))),
                    },
                ),
                "join" => (
                    RelationType::Join,
                    Rel {
                        rel_type: Some(RelType::Join(Box::new(::substrait::proto::JoinRel {
                            common: Some(::substrait::proto::RelCommon {
                                emit_kind: Some(::substrait::proto::rel_common::EmitKind::Direct(
                                    ::substrait::proto::rel_common::Direct {},
                                )),
                                ..Default::default()
                            }),
                            ..Default::default()
                        }))),
                    },
                ),
                "cross" => (
                    RelationType::Cross,
                    Rel {
                        rel_type: Some(RelType::Cross(Box::new(::substrait::proto::CrossRel {
                            common: Some(::substrait::proto::RelCommon {
                                emit_kind: Some(::substrait::proto::rel_common::EmitKind::Direct(
                                    ::substrait::proto::rel_common::Direct {},
                                )),
                                ..Default::default()
                            }),
                            ..Default::default()
                        }))),
                    },
                ),
                "fetch" => (
                    RelationType::Fetch,
                    Rel {
                        rel_type: Some(RelType::Fetch(Box::new(
                            ::substrait::proto::FetchRel::default(),
                        ))),
                    },
                ),
                "aggregate" => {
                    let mut agg_rel = ::substrait::proto::AggregateRel::default();
                    // Add empty grouping (required if no measures)
                    #[allow(deprecated)]
                    {
                        agg_rel
                            .groupings
                            .push(::substrait::proto::aggregate_rel::Grouping {
                                grouping_expressions: Vec::new(),
                                expression_references: Vec::new(),
                            });
                    }
                    (
                        RelationType::Aggregate,
                        Rel {
                            rel_type: Some(RelType::Aggregate(Box::new(agg_rel))),
                        },
                    )
                }
                "sort" => (
                    RelationType::Sort,
                    Rel {
                        rel_type: Some(RelType::Sort(Box::new(
                            ::substrait::proto::SortRel::default(),
                        ))),
                    },
                ),
                "filter" => (
                    RelationType::Filter,
                    Rel {
                        rel_type: Some(RelType::Filter(Box::new(
                            ::substrait::proto::FilterRel::default(),
                        ))),
                    },
                ),
                "set" => (
                    RelationType::Set,
                    Rel {
                        rel_type: Some(RelType::Set(::substrait::proto::SetRel::default())),
                    },
                ),
                "hash_join" => (
                    RelationType::HashJoin,
                    Rel {
                        rel_type: Some(RelType::HashJoin(Box::new(::substrait::proto::HashJoinRel {
                            common: Some(::substrait::proto::RelCommon {
                                emit_kind: Some(::substrait::proto::rel_common::EmitKind::Direct(
                                    ::substrait::proto::rel_common::Direct {},
                                )),
                                ..Default::default()
                            }),
                            ..Default::default()
                        }))),
                    },
                ),
                "merge_join" => (
                    RelationType::MergeJoin,
                    Rel {
                        rel_type: Some(RelType::MergeJoin(Box::new(::substrait::proto::MergeJoinRel {
                            common: Some(::substrait::proto::RelCommon {
                                emit_kind: Some(::substrait::proto::rel_common::EmitKind::Direct(
                                    ::substrait::proto::rel_common::Direct {},
                                )),
                                ..Default::default()
                            }),
                            ..Default::default()
                        }))),
                    },
                ),
                "exchange" => (
                    RelationType::Exchange,
                    Rel {
                        rel_type: Some(RelType::Exchange(Box::new(
                            ::substrait::proto::ExchangeRel::default(),
                        ))),
                    },
                ),
                "ddl" => (
                    RelationType::Ddl,
                    Rel {
                        rel_type: Some(RelType::Ddl(Box::new(
                            ::substrait::proto::DdlRel::default(),
                        ))),
                    },
                ),
                "write" => (
                    RelationType::Write,
                    Rel {
                        rel_type: Some(RelType::Write(Box::new(
                            ::substrait::proto::WriteRel::default(),
                        ))),
                    },
                ),
                "extension_leaf" => (
                    RelationType::ExtensionLeaf,
                    Rel {
                        rel_type: Some(RelType::ExtensionLeaf(
                            ::substrait::proto::ExtensionLeafRel::default(),
                        )),
                    },
                ),
                "extension_single" => (
                    RelationType::ExtensionSingle,
                    Rel {
                        rel_type: Some(RelType::ExtensionSingle(Box::new(
                            ::substrait::proto::ExtensionSingleRel::default(),
                        ))),
                    },
                ),
                "extension_multi" => (
                    RelationType::ExtensionMulti,
                    Rel {
                        rel_type: Some(RelType::ExtensionMulti(
                            ::substrait::proto::ExtensionMultiRel::default(),
                        )),
                    },
                ),
                _ => (RelationType::Unknown, Rel::default()),
            }
        } else {
            (RelationType::Unknown, Rel::default())
        };

        // Create RelationData with the Rel protobuf
        let relation_data = RelationData::new(rel_protobuf);
        let blob = Some(Arc::new(Mutex::new(relation_data)) as Arc<Mutex<dyn Any + Send + Sync>>);

        // Define the relation in the symbol table with the blob
        let symbol = self.type_visitor.base.symbol_table_mut().define_symbol(
            name,
            location,
            SymbolType::Relation,
            Some(Box::new(relation_type)),
            blob,
        );

        eprintln!(
            "Created relation symbol '{}' with type {:?} and blob",
            symbol.name(),
            relation_type
        );

        // Set this as the current relation scope
        self.set_current_relation_scope(Some(symbol.clone()));

        Some(symbol)
    }

    /// Process a root relation and add it to the symbol table.
    fn process_root_relation(
        &mut self,
        ctx: &Root_relationContext<'input>,
    ) -> Option<Arc<SymbolInfo>> {
        // Create a location from the context's start token
        let token = ctx.start();
        let location = token_to_location(&token);

        // Root is a Relation symbol with RelationType::Root subtype (matching C++: kRelation + kRoot)
        // It indicates that the pipeline output should be wrapped in a RelRoot at the plan level
        // We need RelationData with an empty Rel for pipeline connectivity
        let mut relation_data = RelationData::new_empty();

        // Extract root output names from the grammar: ROOT { NAMES = [id, id, ...] }
        let root_names: Vec<String> = ctx
            .id_all()
            .iter()
            .map(|id_ctx| id_ctx.get_text())
            .collect();

        eprintln!(
            "Root relation has {} output names: {:?}",
            root_names.len(),
            root_names
        );

        // Store the root names in the RelationData
        relation_data.root_names = root_names;

        let blob = Some(Arc::new(Mutex::new(relation_data)) as Arc<Mutex<dyn Any + Send + Sync>>);

        // Define the root symbol as a Relation with Root subtype (matching C++)
        let symbol = self.type_visitor.base.symbol_table_mut().define_symbol(
            "root".to_string(),
            location,
            SymbolType::Relation,
            Some(Box::new(RelationType::Root)),
            blob,
        );

        eprintln!("Created root relation symbol 'root' with RelationType::Root and blob");

        // Set this as the current relation scope
        self.set_current_relation_scope(Some(symbol.clone()));

        Some(symbol)
    }

    /// Process a pipeline and add it to the symbol table.
    fn process_pipeline(&mut self, ctx: &PipelineContext<'input>) -> Option<Arc<SymbolInfo>> {
        // Pipeline symbols are created by PipelineVisitor via update_relation_symbol
        // This method is kept for potential future use but does not create symbols
        None
    }

    /// Process a schema definition and add it to the symbol table.
    fn process_schema(
        &mut self,
        ctx: &Schema_definitionContext<'input>,
    ) -> Option<Arc<SymbolInfo>> {
        // Extract schema name from the id token
        let schema_name = ctx.id()?.get_text();

        // Create a location from the context's start token
        // Get the start token directly - it's not an Option
        let token = ctx.start();
        let location = token_to_location(&token);

        // Define the schema in the symbol table
        // Symbol table accessed directly via base
        let symbol = self.type_visitor.base.symbol_table_mut().define_symbol(
            schema_name,
            location,
            SymbolType::Schema,
            None,
            None,
        );

        // Process schema items
        for schema_item in ctx.schema_item_all() {
            // We need to borrow each schema_item as a reference
            self.process_schema_item(&schema_item, &symbol);
        }

        Some(symbol)
    }

    /// Process a schema item and add it to the symbol table.
    fn process_schema_item(
        &mut self,
        ctx: &Schema_itemContext<'input>,
        parent_schema: &Arc<SymbolInfo>,
    ) -> Option<Arc<SymbolInfo>> {
        // Get the name of the column using schema item context trait
        let name = ctx.id(0)?.get_text();

        // Get the type from the literal_complex_type
        let type_text = if let Some(type_ctx) = ctx.literal_complex_type() {
            type_ctx.get_text()
        } else {
            "i64".to_string() // Default fallback
        };

        // Convert the type text to a Substrait Type protobuf
        let proto_type = self.type_visitor.text_to_type_proto(ctx, &type_text);

        // Store the Type protobuf in the blob
        let blob = Some(Arc::new(std::sync::Mutex::new(proto_type))
            as Arc<std::sync::Mutex<dyn std::any::Any + Send + Sync>>);

        // Create a location from the context's start token
        let token = ctx.start();
        let location = token_to_location(&token);

        // Define the column in the symbol table with the type blob
        let symbol = self.type_visitor.base.symbol_table_mut().define_symbol(
            name,
            location,
            SymbolType::SchemaColumn,
            None, // subtype
            blob, // blob contains the Type protobuf
        );

        // Set the schema as the parent
        symbol.set_schema(parent_schema.clone());
        Some(symbol)
    }
}

impl<'input> PlanVisitor<'input> for MainPlanVisitor<'input> {
    fn error_listener(&self) -> Arc<ErrorListener> {
        self.type_visitor.error_listener()
    }

    fn symbol_table(&self) -> SymbolTable {
        self.type_visitor.symbol_table()
    }
}

// ANTLR visitor implementation for MainPlanVisitor
impl<'input> ParseTreeVisitor<'input, SubstraitPlanParserContextType> for MainPlanVisitor<'input> {}

impl<'input> SubstraitPlanParserVisitor<'input> for MainPlanVisitor<'input> {
    // Override specific visitor methods for plan parsing

    fn visit_plan(&mut self, ctx: &PlanContext<'input>) {
        // Process the top-level plan structure
        println!("Visiting plan: {}", ctx.get_text());

        // Visit all children to process the entire plan
        self.visit_children(ctx);
    }

    fn visit_plan_detail(&mut self, ctx: &Plan_detailContext<'input>) {
        // Process plan details
        println!("Visiting plan detail: {}", ctx.get_text());

        // Visit children to process nested elements
        self.visit_children(ctx);
    }

    fn visit_pipelines(&mut self, ctx: &PipelinesContext<'input>) {
        // Process pipeline section
        println!("Visiting pipelines: {}", ctx.get_text());

        // Visit children to process individual pipelines
        self.visit_children(ctx);
    }

    fn visit_pipeline(&mut self, ctx: &PipelineContext<'input>) {
        // Process a single pipeline
        println!("Visiting pipeline: {}", ctx.get_text());

        // Process the pipeline and add it to the symbol table
        self.process_pipeline(ctx);

        // Visit children to process pipeline details
        self.visit_children(ctx);
    }

    fn visit_relation(&mut self, ctx: &RelationContext<'input>) {
        // Process a relation definition
        println!("Visiting relation: {}", ctx.get_text());

        // Process the relation and add it to the symbol table
        let relation_symbol = self.process_relation(ctx);

        // Visit children to process relation details (this sets schema, base_schema, etc.)
        self.visit_children(ctx);

        // NOTE: We DON'T call add_input_fields_to_schema here anymore.
        // Instead, it's called lazily from lookup_field_index when first needed.
        // This ensures all relations are fully processed (schemas linked, etc.) before
        // we try to populate field_references.

        // Clear the current relation scope when done with this relation
        self.set_current_relation_scope(None);
    }

    fn visit_root_relation(&mut self, ctx: &Root_relationContext<'input>) {
        // Process a root relation definition
        println!("Visiting root relation: {}", ctx.get_text());

        // Process the root relation and add it to the symbol table
        let relation_symbol = self.process_root_relation(ctx);

        // Visit children to process relation details (this sets schema, base_schema, etc.)
        self.visit_children(ctx);

        // NOTE: We DON'T call add_input_fields_to_schema here anymore.
        // Instead, it's called lazily from lookup_field_index when first needed.
        // This ensures all relations are fully processed (schemas linked, etc.) before
        // we try to populate field_references.

        // Clear the current relation scope when done with this relation
        self.set_current_relation_scope(None);
    }

    fn visit_schema_definition(&mut self, ctx: &Schema_definitionContext<'input>) {
        // Process a schema definition
        println!("Visiting schema definition: {}", ctx.get_text());

        // Process the schema and add it to the symbol table
        self.process_schema(ctx);

        // No need to visit children as process_schema already handles them
    }

    fn visit_extensionspace(&mut self, ctx: &ExtensionspaceContext<'input>) {
        // Process extension space definition
        println!("Visiting extension space: {}", ctx.get_text());
        self.num_spaces_seen += 1;

        // Process the extension space and add it to the symbol table
        let extension_space_symbol = self.process_extension_space(ctx);

        // Set as current extension space before visiting children (functions)
        let old_extension_space = self.current_extension_space.clone();
        self.current_extension_space = extension_space_symbol;

        // Visit children to process extension details (functions)
        self.visit_children(ctx);

        // Restore old extension space
        self.current_extension_space = old_extension_space;
    }

    fn visit_function(&mut self, ctx: &FunctionContext<'input>) {
        // Process function definition
        println!("Visiting function: {}", ctx.get_text());
        self.num_functions_seen += 1;

        // Process the function and add it to the symbol table
        self.process_function(ctx);

        // Visit children to process function details
        self.visit_children(ctx);
    }

    fn visit_source_definition(&mut self, ctx: &Source_definitionContext<'input>) {
        // Process source definition
        println!("Visiting source definition: {}", ctx.get_text());

        // Process the source and add it to the symbol table
        let source_symbol = self.process_source_definition(ctx);

        // Set current source scope before visiting children
        let old_scope = self.current_source_scope.clone();
        self.set_current_source_scope(source_symbol);

        // Visit children to process source details
        self.visit_children(ctx);

        // Restore previous scope
        self.set_current_source_scope(old_scope);
    }

    fn visit_named_table_detail(&mut self, ctx: &Named_table_detailContext<'input>) {
        // Process named table detail (strings in the source)
        println!("Visiting named table detail: {}", ctx.get_text());

        // Process the strings and add them to the symbol table
        // Clone the source symbol to avoid borrow issues
        if let Some(source_symbol) = self.current_source_scope().cloned() {
            self.process_named_table_detail(ctx, &source_symbol);
        }

        // Visit children to process any nested details
        self.visit_children(ctx);
    }

    // We delegate to the TypeVisitor for type-related nodes
    fn visit_literal_basic_type(&mut self, ctx: &Literal_basic_typeContext<'input>) {
        self.type_visitor.visit_literal_basic_type(ctx);
    }

    fn visit_literal_complex_type(&mut self, ctx: &Literal_complex_typeContext<'input>) {
        self.type_visitor.visit_literal_complex_type(ctx);
    }

    // We use the default implementation for other visitor methods,
    // which will call visit_children to traverse the tree
}

/// The PipelineVisitor processes pipeline definitions.
///
/// This visitor is the third phase in the multiphase parsing approach,
/// focusing on pipeline structures.
pub struct PipelineVisitor<'input> {
    symbol_table: SymbolTable,
    error_listener: Arc<ErrorListener>,
    _phantom: std::marker::PhantomData<&'input ()>,
}

impl<'input> PipelineVisitor<'input> {
    /// Creates a new PipelineVisitor.
    pub fn new(symbol_table: SymbolTable, error_listener: Arc<ErrorListener>) -> Self {
        Self {
            symbol_table,
            error_listener,
            _phantom: std::marker::PhantomData,
        }
    }

    /// Gets the symbol table.
    pub fn symbol_table(&self) -> SymbolTable {
        self.symbol_table.clone()
    }

    /// Gets the error listener.
    pub fn error_listener(&self) -> Arc<ErrorListener> {
        self.error_listener.clone()
    }

    /// Adds an error message to the error listener.
    pub fn add_error<'a>(
        &self,
        token: &impl std::ops::Deref<Target = GenericToken<std::borrow::Cow<'a, str>>>,
        message: &str,
    ) {
        let location = token_to_location(token);
        self.error_listener.add_error(message.to_string(), location);
    }

    /// Updates or creates a relation symbol for the given pipeline node.
    /// Matches C++ SubstraitPlanPipelineVisitor::updateRelationSymbol.
    ///
    /// If the symbol doesn't exist, creates a stub Relation symbol with
    /// RelationType::Unknown and empty RelationData to handle incomplete plans.
    fn update_relation_symbol(
        &mut self,
        ctx: &PipelineContext<'input>,
        relation_name: &str,
    ) -> Arc<SymbolInfo> {
        // Check if symbol already exists
        if let Some(symbol) = self.symbol_table.lookup_symbol_by_name(relation_name) {
            // Symbol exists, return it
            // Note: We don't call add_permanent_location here because it requires
            // exclusive access to the Arc, which we can't get if the symbol is
            // already referenced. The location was already set when the symbol
            // was first created.
            return symbol;
        }

        // Symbol doesn't exist - create a stub Relation with Unknown type
        println!(
            "  Creating stub Relation symbol '{}' (missing definition)",
            relation_name
        );
        let location = token_to_location(&ctx.start());
        let relation_data = RelationData::new_empty();
        let blob = Some(Arc::new(Mutex::new(relation_data)) as Arc<Mutex<dyn Any + Send + Sync>>);

        self.symbol_table.define_symbol(
            relation_name.to_string(),
            location,
            SymbolType::Relation,
            Some(Box::new(RelationType::Unknown)),
            blob,
        )
    }
}

impl<'input> PlanVisitor<'input> for PipelineVisitor<'input> {
    fn error_listener(&self) -> Arc<ErrorListener> {
        self.error_listener.clone()
    }

    fn symbol_table(&self) -> SymbolTable {
        self.symbol_table.clone()
    }
}

// ANTLR visitor implementation for PipelineVisitor
impl<'input> ParseTreeVisitor<'input, SubstraitPlanParserContextType> for PipelineVisitor<'input> {}

impl<'input> SubstraitPlanParserVisitor<'input> for PipelineVisitor<'input> {
    // Override specific visitor methods for pipeline processing

    fn visit_plan(&mut self, ctx: &PlanContext<'input>) {
        // Handle the plan node ourselves, not delegating to plan_visitor
        // This ensures our visit_pipeline override gets called
        println!("PipelineVisitor visiting plan node");
        self.visit_children(ctx);
        println!("PipelineVisitor finished visiting plan");
    }

    fn visit_pipelines(&mut self, ctx: &PipelinesContext<'input>) {
        // Process the pipelines section
        println!("PipelineVisitor processing pipelines: {}", ctx.get_text());

        // Only visit the direct pipeline children (top-level of each chain)
        // pipeline_all() returns ALL pipeline contexts including nested ones,
        // but we only want the direct children since visit_pipeline handles recursion
        let all_pipelines = ctx.pipeline_all();
        println!("  Found {} total pipeline contexts", all_pipelines.len());

        // Filter to only those whose parent is this pipelines context
        for pipeline in all_pipelines {
            if let Some(parent) = pipeline.get_parent_ctx() {
                // Check if the parent is a pipelines context (not another pipeline)
                if parent.get_rule_index() == RULE_pipelines {
                    println!("  Visiting top-level pipeline: {}", pipeline.get_text());
                    self.visit_pipeline(&pipeline);
                } else {
                    println!("  Skipping nested pipeline: {}", pipeline.get_text());
                }
            }
        }
    }

    fn visit_pipeline(&mut self, ctx: &PipelineContext<'input>) {
        // Following C++ SubstraitPlanPipelineVisitor::visitPipeline
        println!("PipelineVisitor processing pipeline: {}", ctx.get_text());

        // Get the relation name from this pipeline
        let relation_name = if let Some(relation_ref) = ctx.relation_ref() {
            println!("  Found relation_ref");
            if let Some(id) = relation_ref.id(0) {
                let name = id.get_text();
                println!("  Relation name: {}", name);
                name
            } else {
                println!("  No id(0) in relation_ref, returning early");
                return;
            }
        } else {
            println!("  No relation_ref, returning early");
            return;
        };

        // Ensure the symbol exists (create stub if missing)
        let symbol = self.update_relation_symbol(ctx, &relation_name);
        println!("  Using symbol '{}'", relation_name);

        // Process nested pipeline first (left-to-right processing)
        if let Some(nested_pipeline) = ctx.pipeline() {
            println!("  Processing nested pipeline for '{}'", relation_name);
            self.visit_pipeline(&nested_pipeline);
            println!("  Finished nested pipeline for '{}'", relation_name);
        }

        // Get the RelationData for this symbol
        let blob_lock = if let Some(blob) = &symbol.blob {
            blob
        } else {
            eprintln!("Warning: Symbol '{}' has no blob", relation_name);
            return;
        };

        let mut blob_data = match blob_lock.lock() {
            Ok(data) => data,
            Err(_) => {
                eprintln!("Warning: Failed to lock blob for '{}'", relation_name);
                return;
            }
        };

        let relation_data = if let Some(data) = blob_data.downcast_mut::<RelationData>() {
            data
        } else {
            eprintln!("Warning: Blob is not RelationData for '{}'", relation_name);
            return;
        };

        // Check for accidental cross-pipeline use
        if relation_data.continuing_pipeline.is_some() {
            eprintln!(
                "Error: Relation {} is already a non-terminating participant in a pipeline",
                relation_name
            );
            return;
        }

        // Get left symbol (nested pipeline)
        let left_symbol = if let Some(nested_pipeline) = ctx.pipeline() {
            if let Some(nested_ref) = nested_pipeline.relation_ref() {
                if let Some(nested_id) = nested_ref.id(0) {
                    let nested_name = nested_id.get_text();
                    self.symbol_table.lookup_symbol_by_name(&nested_name)
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
        };

        // Get right symbol (parent pipeline)
        // In ANTLR, the parent context might be another pipeline or a root relation
        // Following C++ logic: if parent is a Pipeline context, look up symbol there
        let right_symbol: Option<Arc<SymbolInfo>> = {
            // Check if parent context exists and is a Pipeline
            if let Some(parent_ctx) = ctx.get_parent_ctx() {
                // Check if parent is a pipeline by checking rule index
                if parent_ctx.get_rule_index() == RULE_pipeline {
                    // Try to downcast parent to PipelineContext
                    if let Some(parent_pipeline) = parent_ctx.downcast_ref::<PipelineContext>() {
                        // Parent is a pipeline, get the relation name from it
                        if let Some(parent_ref) = parent_pipeline.relation_ref() {
                            if let Some(parent_id) = parent_ref.id(0) {
                                let parent_name = parent_id.get_text();
                                self.symbol_table.lookup_symbol_by_name(&parent_name)
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                } else {
                    None
                }
            } else {
                None
            }
        };

        // Determine rightmost symbol (pipeline start)
        // Since we process left-to-right, the left (child) has already been processed
        // and has its pipeline_start set. We should use that.
        // If there's no left symbol, we are the rightmost (start of pipeline).
        let rightmost_symbol = if let Some(ref left) = left_symbol {
            if let Some(left_blob) = &left.blob {
                if let Ok(left_data) = left_blob.lock() {
                    if let Some(left_rel_data) = left_data.downcast_ref::<RelationData>() {
                        // Left is a Relation, use its pipeline_start (or left itself if it's the start)
                        left_rel_data.pipeline_start.clone().or(Some(left.clone()))
                    } else {
                        // Left has blob but not RelationData, we are the start
                        Some(symbol.clone())
                    }
                } else {
                    // Failed to lock, we are the start
                    Some(symbol.clone())
                }
            } else {
                // Left has no blob, we are the start
                Some(symbol.clone())
            }
        } else {
            // No left symbol, we are the rightmost (start of pipeline)
            Some(symbol.clone())
        };

        // Set pipeline start
        if let Some(rightmost) = rightmost_symbol {
            relation_data.pipeline_start = Some(rightmost);
        }

        // Connect to the left symbol
        if let Some(left) = left_symbol {
            // Determine the relation category for pipeline connections:
            // - Binary relations (Join, Cross, etc.) use new_pipelines for multiple inputs
            // - Root/terminal relations (Fetch, etc.) use new_pipelines to be identified as terminals
            // - Unary relations (Filter, Project, etc.) use continuing_pipeline for single input

            // Check if this is a terminal/root relation by name (since rel_type may not be set yet)
            // Only the "root" relation itself is terminal by name, not relations named "root1", "root2", etc.
            let is_root_by_name = relation_name == "root";

            let relation_category = if is_root_by_name {
                "terminal"
            } else {
                match &relation_data.relation.rel_type {
                    Some(::substrait::proto::rel::RelType::Join(_)) => "binary",
                    Some(::substrait::proto::rel::RelType::Cross(_)) => "binary",
                    Some(::substrait::proto::rel::RelType::Set(_)) => "binary",
                    Some(::substrait::proto::rel::RelType::HashJoin(_)) => "binary",
                    Some(::substrait::proto::rel::RelType::MergeJoin(_)) => "binary",
                    Some(::substrait::proto::rel::RelType::Fetch(_)) => "unary",
                    Some(::substrait::proto::rel::RelType::ExtensionSingle(_)) => "unary",
                    Some(::substrait::proto::rel::RelType::ExtensionLeaf(_)) => "terminal",
                    _ => "unary",
                }
            };

            if relation_category == "binary" || relation_category == "terminal" {
                // Binary or terminal relation: use new_pipelines for inputs
                println!(
                    "  Pipeline: {} ({} relation) adds new branch with left: {}",
                    relation_name,
                    relation_category,
                    left.name()
                );
                relation_data.new_pipelines.push(left);
            } else {
                // Unary relation: use continuing_pipeline for single input
                println!(
                    "  Pipeline: {} (unary relation) continues with left: {}",
                    relation_name,
                    left.name()
                );
                relation_data.continuing_pipeline = Some(left);
            }
        } else {
            println!("  Pipeline: {} has no left symbol", relation_name);
        }

        // Drop the lock before calling visit_children to avoid deadlock
        drop(blob_data);

        println!("  Finished setting up connections for '{}'", relation_name);

        // DON'T visit children - we already processed nested pipeline above
        // Calling visit_children here would cause infinite recursion
        // self.visit_children(ctx);

        println!("  Completed visit_pipeline for '{}'", relation_name);
    }

    // For plan_detail, continue traversing to find pipelines
    fn visit_plan_detail(&mut self, ctx: &Plan_detailContext<'input>) {
        // Continue traversing to find pipelines
        self.visit_children(ctx);
    }

    fn visit_relation(&mut self, ctx: &RelationContext<'input>) {
        // Skip - already processed
    }

    fn visit_extensionspace(&mut self, ctx: &ExtensionspaceContext<'input>) {
        // Skip - already processed
    }

    fn visit_function(&mut self, ctx: &FunctionContext<'input>) {
        // Skip - already processed
    }

    fn visit_source_definition(&mut self, ctx: &Source_definitionContext<'input>) {
        // Skip - already processed
    }

    fn visit_named_table_detail(&mut self, ctx: &Named_table_detailContext<'input>) {
        // Skip - already processed
    }

    // We use the default implementation for other visitor methods,
    // which will call visit_children to traverse the tree
}

/// The RelationVisitor processes relations and expressions.
///
/// This visitor is the fourth phase in the multiphase parsing approach,
/// handling relation structures and their expressions.
pub struct RelationVisitor<'input> {
    symbol_table: SymbolTable,
    error_listener: Arc<ErrorListener>,
    current_relation_scope: Option<Arc<SymbolInfo>>,
    prescan_mode: bool,
    processing_emit: bool, // Track if we're currently processing an emit clause
    _phantom: std::marker::PhantomData<&'input ()>,
}

/// Helper function to parse sort direction from string
fn parse_sort_direction(text: &str) -> i32 {
    // Normalize the text: lowercase and remove underscores/special chars
    let normalized = text
        .to_lowercase()
        .chars()
        .filter(|c| c.is_alphanumeric())
        .collect::<String>();

    use ::substrait::proto::sort_field::SortDirection;
    match normalized.as_str() {
        "ascnullsfirst" => SortDirection::AscNullsFirst as i32,
        "ascnullslast" => SortDirection::AscNullsLast as i32,
        "descnullsfirst" => SortDirection::DescNullsFirst as i32,
        "descnullslast" => SortDirection::DescNullsLast as i32,
        "clustered" => SortDirection::Clustered as i32,
        _ => {
            eprintln!(
                "Unrecognized sort direction: {}, using ASC_NULLS_LAST",
                text
            );
            SortDirection::AscNullsLast as i32
        }
    }
}

impl<'input> RelationVisitor<'input> {
    /// Creates a new RelationVisitor.
    pub fn new(symbol_table: SymbolTable, error_listener: Arc<ErrorListener>) -> Self {
        Self {
            symbol_table,
            error_listener,
            current_relation_scope: None,
            prescan_mode: false,
            processing_emit: false,
            _phantom: std::marker::PhantomData,
        }
    }

    /// Sets prescan mode - when true, only marks subqueries without building expressions.
    pub fn set_prescan_mode(&mut self, prescan: bool) {
        self.prescan_mode = prescan;
    }

    /// Gets the symbol table.
    pub fn symbol_table(&self) -> SymbolTable {
        self.symbol_table.clone()
    }

    /// Gets a mutable reference to the symbol table.
    fn symbol_table_mut(&mut self) -> &mut SymbolTable {
        &mut self.symbol_table
    }

    /// Gets the error listener.
    pub fn error_listener(&self) -> Arc<ErrorListener> {
        self.error_listener.clone()
    }

    /// Adds an error message to the error listener.
    pub fn add_error<'a>(
        &self,
        token: &impl std::ops::Deref<Target = GenericToken<std::borrow::Cow<'a, str>>>,
        message: &str,
    ) {
        let location = token_to_location(token);
        self.error_listener.add_error(message.to_string(), location);
    }

    /// Gets the current relation scope, if any.
    pub fn current_relation_scope(&self) -> Option<&Arc<SymbolInfo>> {
        self.current_relation_scope.as_ref()
    }

    /// Sets the current relation scope.
    pub fn set_current_relation_scope(&mut self, scope: Option<Arc<SymbolInfo>>) {
        self.current_relation_scope = scope;
    }

    /// Populates field_references for a relation from its input pipelines.
    /// This is called lazily when lookup_field_index needs field information.
    fn add_input_fields_to_schema(&mut self, relation_symbol: &Arc<SymbolInfo>) {
        use std::cell::RefCell;
        use std::collections::HashSet;

        thread_local! {
            static VISITING: RefCell<HashSet<String>> = RefCell::new(HashSet::new());
        }

        // Check if we're currently visiting this relation (cycle detection)
        let is_visiting = VISITING.with(|v| v.borrow().contains(relation_symbol.name()));
        if is_visiting {
            println!(
                "    CYCLE DETECTED: Already visiting '{}', stopping recursion",
                relation_symbol.name()
            );
            return;
        }

        println!(
            "    add_input_fields_to_schema called for '{}'",
            relation_symbol.name()
        );

        // Check if already populated (early return to avoid unnecessary work)
        // But first ensure upstream generated fields are populated
        if let Some(blob_lock) = &relation_symbol.blob {
            if let Ok(blob_data) = blob_lock.lock() {
                if let Some(relation_data) = blob_data.downcast_ref::<RelationData>() {
                    if !relation_data.field_references.is_empty() {
                        println!(
                            "      '{}' already has {} field_references",
                            relation_symbol.name(),
                            relation_data.field_references.len()
                        );

                        // Before returning, ensure upstream generated fields are populated
                        // (This is needed because upstream may not have generated its fields yet)
                        let continuing = relation_data.continuing_pipeline.clone();
                        let new_pipes = relation_data.new_pipelines.clone();
                        drop(blob_data);
                        drop(blob_lock);

                        if let Some(cont) = continuing {
                            self.add_expressions_to_schema(&cont);
                        }
                        for pipe in &new_pipes {
                            self.add_expressions_to_schema(pipe);
                        }

                        return; // Already populated
                    }
                }
            }
        }

        // Mark as visiting
        VISITING.with(|v| v.borrow_mut().insert(relation_symbol.name().to_string()));

        // Recursively populate upstream relations first
        if let Some(blob_lock) = &relation_symbol.blob {
            if let Ok(blob_data) = blob_lock.lock() {
                if let Some(relation_data) = blob_data.downcast_ref::<RelationData>() {
                    // Collect upstream relations to populate (need to drop lock before recursing)
                    let mut upstreams = Vec::new();
                    if let Some(cont) = &relation_data.continuing_pipeline {
                        upstreams.push(cont.clone());
                    }
                    for pipe in &relation_data.new_pipelines {
                        upstreams.push(pipe.clone());
                    }
                    drop(blob_data);

                    // Recursively populate upstreams
                    for upstream in upstreams {
                        self.add_input_fields_to_schema(&upstream);
                    }
                }
            }
        }

        // Now populate this relation's field_references from its (now-populated) upstreams
        // IMPORTANT: Don't hold locks while trying to acquire other locks to avoid deadlock!

        // First, collect upstream field references without holding our lock
        let mut collected_fields = Vec::new();

        // Check if this is a READ relation and needs schema fields
        let is_read_with_schema = if let Some(blob_lock) = &relation_symbol.blob {
            if let Ok(blob_data) = blob_lock.lock() {
                if let Some(relation_data) = blob_data.downcast_ref::<RelationData>() {
                    if let Some(RelType::Read(_)) = &relation_data.relation.rel_type {
                        relation_data.schema.clone()
                    } else {
                        None
                    }
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
        };

        if let Some(schema_arc) = is_read_with_schema {
            // For READ relations, collect fields from schema
            for symbol in self.symbol_table().symbols() {
                if symbol.symbol_type() == SymbolType::SchemaColumn {
                    if let Some(symbol_schema) = symbol.schema() {
                        if Arc::ptr_eq(&symbol_schema, &schema_arc) {
                            collected_fields.push(symbol.clone());
                        }
                    }
                }
            }
        } else {
            // For non-READ relations, collect fields from pipelines
            // Get continuing_pipeline and new_pipelines without holding lock
            let (continuing_pipeline, new_pipelines) =
                if let Some(blob_lock) = &relation_symbol.blob {
                    if let Ok(blob_data) = blob_lock.lock() {
                        if let Some(relation_data) = blob_data.downcast_ref::<RelationData>() {
                            (
                                relation_data.continuing_pipeline.clone(),
                                relation_data.new_pipelines.clone(),
                            )
                        } else {
                            (None, Vec::new())
                        }
                    } else {
                        (None, Vec::new())
                    }
                } else {
                    (None, Vec::new())
                };

            // Now process pipelines without holding our lock
            if let Some(continuing_pipeline) = continuing_pipeline {
                // Ensure the upstream relation's generated_field_references are populated first
                // This must be done before we copy its fields
                self.add_expressions_to_schema(&continuing_pipeline);

                if let Some(cont_blob_lock) = &continuing_pipeline.blob {
                    if let Ok(cont_blob_data) = cont_blob_lock.lock() {
                        if let Some(cont_relation_data) =
                            cont_blob_data.downcast_ref::<RelationData>()
                        {
                            if !cont_relation_data.output_field_references.is_empty() {
                                collected_fields
                                    .extend(cont_relation_data.output_field_references.clone());
                            } else {
                                collected_fields
                                    .extend(cont_relation_data.field_references.clone());
                                collected_fields
                                    .extend(cont_relation_data.generated_field_references.clone());
                            }
                        }
                    }
                }
            }

            for pipeline in &new_pipelines {
                // Ensure the upstream relation's generated_field_references are populated first
                self.add_expressions_to_schema(pipeline);

                if let Some(pipe_blob_lock) = &pipeline.blob {
                    if let Ok(pipe_blob_data) = pipe_blob_lock.lock() {
                        if let Some(pipe_relation_data) =
                            pipe_blob_data.downcast_ref::<RelationData>()
                        {
                            if !pipe_relation_data.output_field_references.is_empty() {
                                collected_fields
                                    .extend(pipe_relation_data.output_field_references.clone());
                            } else {
                                collected_fields
                                    .extend(pipe_relation_data.field_references.clone());
                                collected_fields
                                    .extend(pipe_relation_data.generated_field_references.clone());
                            }
                        }
                    }
                }
            }
        }

        // Finally, update our field_references with collected fields
        if let Some(blob_lock) = &relation_symbol.blob {
            if let Ok(mut blob_data) = blob_lock.lock() {
                if let Some(relation_data) = blob_data.downcast_mut::<RelationData>() {
                    relation_data.field_references = collected_fields;
                }
            }
        }

        // Remove from visiting set
        VISITING.with(|v| v.borrow_mut().remove(relation_symbol.name()));

        println!(
            "    Finished populating field_references for '{}'",
            relation_symbol.name()
        );
    }

    /// Populates generated_field_references for a relation from its proto expressions.
    /// This is called after the relation details (expressions, measures, etc.) have been visited.
    /// Follows the C++ SubstraitPlanRelationVisitor::addExpressionsToSchema pattern.
    fn add_expressions_to_schema(&mut self, relation_symbol: &Arc<SymbolInfo>) {
        use substrait::proto::rel::RelType;

        println!(
            "    add_expressions_to_schema called for '{}'",
            relation_symbol.name()
        );

        // Phase 1: Collect expression information while holding the lock
        enum ExprInfo {
            FieldSelection(Arc<SymbolInfo>),
            ComplexExpression(Option<String>), // alias if exists
        }

        let expr_infos: Vec<ExprInfo> = if let Some(blob_lock) = &relation_symbol.blob {
            if let Ok(blob_data) = blob_lock.lock() {
                if let Some(relation_data) = blob_data.downcast_ref::<RelationData>() {
                    // Skip if already populated (idempotent)
                    if !relation_data.generated_field_references.is_empty() {
                        println!(
                            "      Already has {} generated_field_references, skipping",
                            relation_data.generated_field_references.len()
                        );
                        return;
                    }

                    // Process based on relation type
                    if let Some(rel_type) = &relation_data.relation.rel_type {
                        match rel_type {
                            RelType::Project(project_rel) => {
                                println!(
                                    "      Processing {} project expressions for '{}'",
                                    project_rel.expressions.len(),
                                    relation_symbol.name()
                                );

                                let mut infos = Vec::new();
                                for (expression_number, expr) in
                                    project_rel.expressions.iter().enumerate()
                                {
                                    // Check if this is a simple field selection
                                    if let Some(substrait::proto::expression::RexType::Selection(
                                        field_ref,
                                    )) = &expr.rex_type
                                    {
                                        if let Some(
                                            substrait::proto::expression::field_reference::ReferenceType::DirectReference(
                                                ref_segment,
                                            ),
                                        ) = &field_ref.reference_type
                                        {
                                            if let Some(
                                                substrait::proto::expression::reference_segment::ReferenceType::StructField(
                                                    struct_field,
                                                ),
                                            ) = &ref_segment.reference_type
                                            {
                                                // Simple field selection
                                                let field_index = struct_field.field as usize;
                                                if field_index < relation_data.field_references.len() {
                                                    let field_symbol = relation_data.field_references
                                                        [field_index]
                                                        .clone();
                                                    println!(
                                                        "        Expr {}: field selection -> will add '{}'",
                                                        expression_number, field_symbol.name()
                                                    );
                                                    infos.push(ExprInfo::FieldSelection(field_symbol));
                                                } else {
                                                    println!(
                                                        "        Expr {}: field index {} out of range",
                                                        expression_number, field_index
                                                    );
                                                }
                                                continue;
                                            }
                                        }
                                    }

                                    // Not a simple field selection - complex expression
                                    let alias = relation_data
                                        .generated_field_reference_aliases
                                        .get(&expression_number)
                                        .cloned();
                                    infos.push(ExprInfo::ComplexExpression(alias));
                                }

                                infos
                            }
                            _ => {
                                println!(
                                    "      Relation '{}' is not a project, skipping",
                                    relation_symbol.name()
                                );
                                Vec::new()
                            }
                        }
                    } else {
                        Vec::new()
                    }
                } else {
                    Vec::new()
                }
            } else {
                Vec::new()
            }
        } else {
            Vec::new()
        };

        // Phase 2: Create symbols without holding any locks
        // IMPORTANT: Both field selections AND complex expressions are added to generated_field_references.
        // Field selections reuse existing symbols but still occupy positions in the generated list.
        // This matches substrait-cpp behavior (SubstraitPlanRelationVisitor.cpp:1983).
        let mut generated_symbols = Vec::new();
        for (expr_num, info) in expr_infos.into_iter().enumerate() {
            match info {
                ExprInfo::FieldSelection(field_sym) => {
                    // Field selections reuse existing field symbols but add them to generated_field_references
                    // This is critical for emit to find the correct indices!
                    println!(
                        "        Expr {}: field selection '{}' -> adding to generated_field_references",
                        expr_num,
                        field_sym.name()
                    );
                    generated_symbols.push(field_sym);
                }
                ExprInfo::ComplexExpression(alias) => {
                    // Get unique name and create symbol
                    let unique_name = if let Some(alias_name) = alias {
                        alias_name
                    } else {
                        self.symbol_table_mut().get_unique_name("intermediate")
                    };

                    println!(
                        "        Expr {}: complex expression -> creating '{}'",
                        expr_num, unique_name
                    );

                    let new_symbol = self.symbol_table_mut().define_symbol(
                        unique_name,
                        relation_symbol.source_location().box_clone(),
                        SymbolType::Unknown,
                        None,
                        None,
                    );

                    generated_symbols.push(new_symbol);
                }
            }
        }

        // Phase 3: Add symbols to generated_field_references
        if !generated_symbols.is_empty() {
            if let Some(blob_lock) = &relation_symbol.blob {
                if let Ok(mut blob_data) = blob_lock.lock() {
                    if let Some(relation_data) = blob_data.downcast_mut::<RelationData>() {
                        relation_data.generated_field_references = generated_symbols;
                        println!(
                            "      Finished: {} generated_field_references for '{}'",
                            relation_data.generated_field_references.len(),
                            relation_symbol.name()
                        );
                    }
                }
            }
        }
    }

    /// Process a relation type and update the relation symbol.
    /// Note: This function is now deprecated - the relation type and blob
    /// are set in process_relation() when the symbol is created.
    fn process_relation_type(
        &mut self,
        _ctx: &Relation_typeContext<'input>,
        _relation_symbol: &Arc<SymbolInfo>,
    ) {
        // This function is no longer needed since we create the blob
        // when defining the symbol in process_relation()
    }

    /// Process a filter relation and add it to the symbol table.
    fn process_filter_relation(
        &mut self,
        ctx: &RelationFilterContext<'input>,
    ) -> Option<Arc<SymbolInfo>> {
        // Create a location from the context's start token
        let token = ctx.start();
        let location = token_to_location(&token);

        // Define the filter relation in the symbol table
        let symbol = self.symbol_table_mut().define_symbol(
            "filter".to_string(),
            location,
            SymbolType::Relation,
            Some(Box::new(RelationType::Filter)),
            None,
        );

        Some(symbol)
    }

    /// Process an expression relation and add it to the symbol table.
    fn process_expression_relation(
        &mut self,
        ctx: &RelationExpressionContext<'input>,
    ) -> Option<Arc<SymbolInfo>> {
        // Create a location from the context's start token
        // Get the start token directly - it's not an Option
        let token = ctx.start();
        let location = token_to_location(&token);

        // Define the expression relation in the symbol table
        let symbol = self.symbol_table_mut().define_symbol(
            "expression".to_string(),
            location,
            SymbolType::Relation,
            Some(Box::new(RelationType::Project)),
            None,
        );

        Some(symbol)
    }

    /// Process a join relation and add it to the symbol table.
    fn process_join_relation(
        &mut self,
        ctx: &RelationJoinTypeContext<'input>,
    ) -> Option<Arc<SymbolInfo>> {
        // Get the join type
        let join_type_text = ctx.get_text().to_lowercase();
        let join_type = if join_type_text.contains("hash") {
            RelationType::HashJoin
        } else if join_type_text.contains("merge") {
            RelationType::MergeJoin
        } else {
            RelationType::Join
        };

        // Create a location from the context's start token
        let token = ctx.start();
        let location = token_to_location(&token);

        // Define the join relation in the symbol table
        let symbol = self.symbol_table_mut().define_symbol(
            "join".to_string(),
            location,
            SymbolType::Relation,
            Some(Box::new(join_type)),
            None,
        );

        Some(symbol)
    }

    /// Process a constant expression and add it to the symbol table.
    fn process_constant_expression(
        &mut self,
        ctx: &ExpressionConstantContext<'input>,
    ) -> Option<Arc<SymbolInfo>> {
        // Get the constant value
        let value = ctx
            .constant()
            .map_or("unknown".to_string(), |c| c.get_text().to_string());

        // Create a location from the context's start token
        // Get the start token directly - it's not an Option
        let token = ctx.start();
        let location = token_to_location(&token);

        // Define the constant in the symbol table
        let symbol = self.symbol_table_mut().define_symbol(
            value.to_string(),
            location,
            SymbolType::Field,
            None,
            None,
        );

        Some(symbol)
    }

    /// Process a column reference expression and add it to the symbol table.
    fn process_column_expression(
        &mut self,
        ctx: &ExpressionColumnContext<'input>,
    ) -> Option<Arc<SymbolInfo>> {
        // Get the column name
        let name = ctx
            .column_name()
            .map_or("unnamed_column".to_string(), |c| c.get_text().to_string());

        // Create a location from the context's start token
        // Get the start token directly - it's not an Option
        let token = ctx.start();
        let location = token_to_location(&token);

        // Define the column in the symbol table
        let symbol = self.symbol_table_mut().define_symbol(
            name.to_string(),
            location,
            SymbolType::Field,
            None,
            None,
        );

        Some(symbol)
    }

    /// Process a function call expression and add it to the symbol table.
    fn process_function_expression(
        &mut self,
        ctx: &ExpressionFunctionUseContext<'input>,
    ) -> Option<Arc<SymbolInfo>> {
        // Get the function name - simplify for now
        let function_name = "unnamed_function"; // We'll fix this when we have the proper context

        // Create a location from the context's start token
        // Get the start token directly - it's not an Option
        let token = ctx.start();
        let location = token_to_location(&token);

        // Define the function call in the symbol table
        let symbol = self.symbol_table_mut().define_symbol(
            function_name.to_string(),
            location,
            SymbolType::Function,
            None,
            None,
        );

        Some(symbol)
    }

    /// Build an Expression protobuf from an expression AST node
    fn build_expression(
        &mut self,
        expr_ctx: &Rc<ExpressionContextAll<'input>>,
    ) -> ::substrait::proto::Expression {
        // In prescan mode, only process subquery expressions to mark them
        if self.prescan_mode {
            match expr_ctx.as_ref() {
                ExpressionContextAll::ExpressionScalarSubqueryContext(ctx) => {
                    return self.build_scalar_subquery(ctx);
                }
                ExpressionContextAll::ExpressionSetComparisonSubqueryContext(ctx) => {
                    return self.build_set_comparison_subquery(ctx);
                }
                ExpressionContextAll::ExpressionInPredicateSubqueryContext(ctx) => {
                    return self.build_in_predicate_subquery(ctx);
                }
                ExpressionContextAll::ExpressionFunctionUseContext(ctx) => {
                    // Still process function calls to find nested subqueries
                    return self.build_function_call(ctx);
                }
                _ => {
                    // Skip other expression types in prescan mode
                    return ::substrait::proto::Expression {
                        rex_type: Some(::substrait::proto::expression::RexType::Literal(
                            ::substrait::proto::expression::Literal {
                                literal_type: Some(
                                    ::substrait::proto::expression::literal::LiteralType::I64(0),
                                ),
                                nullable: false,
                                type_variation_reference: 0,
                            },
                        )),
                    };
                }
            }
        }

        // Match on the expression type
        match expr_ctx.as_ref() {
            ExpressionContextAll::ExpressionConstantContext(ctx) => {
                // Parse the constant value
                if let Some(constant_ctx) = ctx.constant() {
                    self.build_constant(&constant_ctx)
                } else {
                    // Fallback to placeholder
                    ::substrait::proto::Expression {
                        rex_type: Some(::substrait::proto::expression::RexType::Literal(
                            ::substrait::proto::expression::Literal {
                                literal_type: Some(
                                    ::substrait::proto::expression::literal::LiteralType::I64(0),
                                ),
                                nullable: false,
                                type_variation_reference: 0,
                            },
                        )),
                    }
                }
            }
            ExpressionContextAll::ExpressionColumnContext(ctx) => {
                println!("  Building column reference expression");
                self.build_column_reference(ctx)
            }
            ExpressionContextAll::ExpressionFunctionUseContext(ctx) => {
                println!("  Building function call expression");
                self.build_function_call(ctx)
            }
            ExpressionContextAll::ExpressionCastContext(ctx) => {
                println!("  Building cast expression");
                self.build_cast_expression(ctx)
            }
            ExpressionContextAll::ExpressionSetComparisonSubqueryContext(ctx) => {
                println!("  Building set comparison subquery expression");
                self.build_set_comparison_subquery(ctx)
            }
            ExpressionContextAll::ExpressionScalarSubqueryContext(ctx) => {
                println!("  Building scalar subquery expression");
                self.build_scalar_subquery(ctx)
            }
            ExpressionContextAll::ExpressionInPredicateSubqueryContext(ctx) => {
                println!("  Building IN predicate subquery expression");
                self.build_in_predicate_subquery(ctx)
            }
            ExpressionContextAll::ExpressionSetPredicateSubqueryContext(ctx) => {
                println!("  Building set predicate subquery expression");
                self.build_set_predicate_subquery(ctx)
            }
            _ => {
                println!("  Building unknown expression type (placeholder)");
                ::substrait::proto::Expression {
                    rex_type: Some(::substrait::proto::expression::RexType::Literal(
                        ::substrait::proto::expression::Literal {
                            literal_type: Some(
                                ::substrait::proto::expression::literal::LiteralType::I64(0),
                            ),
                            nullable: false,
                            type_variation_reference: 0,
                        },
                    )),
                }
            }
        }
    }

    /// Build a column reference expression from a column context
    fn build_column_reference(
        &mut self,
        ctx: &ExpressionColumnContext<'input>,
    ) -> ::substrait::proto::Expression {
        // Get the column name
        let column_name = ctx
            .column_name()
            .map(|c| c.get_text())
            .unwrap_or_else(|| "unknown".to_string());

        println!("    Column reference: {}", column_name);

        // Check if this is an outer reference (from a parent scope)
        let (field_index, steps_out) = self.lookup_field_with_scope(&column_name);

        println!(
            "      -> field index: {}, steps_out: {}",
            field_index, steps_out
        );

        // Create the appropriate root_type based on whether this is an outer reference
        let root_type = if steps_out > 0 {
            Some(
                ::substrait::proto::expression::field_reference::RootType::OuterReference(
                    ::substrait::proto::expression::field_reference::OuterReference {
                        steps_out: steps_out as u32,
                    },
                ),
            )
        } else {
            Some(
                ::substrait::proto::expression::field_reference::RootType::RootReference(
                    ::substrait::proto::expression::field_reference::RootReference {},
                ),
            )
        };

        ::substrait::proto::Expression {
            rex_type: Some(::substrait::proto::expression::RexType::Selection(Box::new(
                ::substrait::proto::expression::FieldReference {
                    reference_type: Some(::substrait::proto::expression::field_reference::ReferenceType::DirectReference(
                        ::substrait::proto::expression::ReferenceSegment {
                            reference_type: Some(::substrait::proto::expression::reference_segment::ReferenceType::StructField(Box::new(
                                ::substrait::proto::expression::reference_segment::StructField {
                                    field: field_index as i32,
                                    child: None,
                                }
                            ))),
                        }
                    )),
                    root_type,
                },
            ))),
        }
    }

    /// Get the parent query location for a relation, following C++ getParentQueryLocation.
    /// First checks the relation's own parent_query info, then checks pipeline_start.
    /// Returns the relation symbol that has parent_query info set, or None.
    fn find_relation_with_parent_query_info(
        &self,
        relation: &Arc<SymbolInfo>,
    ) -> Option<Arc<SymbolInfo>> {
        println!(
            "      DEBUG find_relation_with_parent_query_info: checking '{}', parent_index={}",
            relation.name(),
            relation.parent_query_index()
        );

        // First check if this relation has parent_query info
        if relation.parent_query_index() >= 0 {
            println!(
                "      DEBUG find_relation_with_parent_query_info: Found parent info on '{}'",
                relation.name()
            );
            return Some(relation.clone());
        }

        // If not, check pipeline_start
        if let Some(blob_lock) = &relation.blob {
            if let Ok(blob_data) = blob_lock.lock() {
                if let Some(relation_data) = blob_data.downcast_ref::<crate::textplan::common::structured_symbol_data::RelationData>() {
                    if let Some(pipeline_start) = &relation_data.pipeline_start {
                        println!(
                            "      DEBUG find_relation_with_parent_query_info: checking pipeline_start '{}', parent_index={}",
                            pipeline_start.name(),
                            pipeline_start.parent_query_index()
                        );
                        if pipeline_start.parent_query_index() >= 0 {
                            println!(
                                "      DEBUG find_relation_with_parent_query_info: Found parent info on pipeline_start '{}'",
                                pipeline_start.name()
                            );
                            return Some(pipeline_start.clone());
                        }
                    }
                }
            }
        }

        None
    }

    /// Sets pipeline_start on all relations in a subquery pipeline.
    /// This follows the C++ PipelineVisitor pattern.
    fn set_pipeline_start_for_subquery(&self, subquery_root: &Arc<SymbolInfo>) {
        println!(
            "      Setting pipeline_start for subquery rooted at '{}'",
            subquery_root.name()
        );

        // Walk the pipeline backward via continuing_pipeline and set both pipeline_start
        // AND parent_query_index on all relations in the subquery (following substrait-cpp)
        if let Some(blob_lock) = &subquery_root.blob {
            if let Ok(mut blob_data) = blob_lock.lock() {
                if let Some(relation_data) = blob_data.downcast_mut::<crate::textplan::common::structured_symbol_data::RelationData>() {
                    // Set this relation's pipeline_start to itself
                    relation_data.pipeline_start = Some(subquery_root.clone());

                    // Walk the continuing_pipeline chain
                    let mut current = relation_data.continuing_pipeline.clone();
                    while let Some(current_rel) = current {
                        println!("        Setting pipeline_start and parent_query_index on '{}'", current_rel.name());

                        // Set parent_query_index to mark this relation as part of the subquery
                        // This must match substrait-cpp behavior
                        current_rel.set_parent_query_index(0);

                        if let Some(curr_blob_lock) = &current_rel.blob {
                            if let Ok(mut curr_blob_data) = curr_blob_lock.lock() {
                                if let Some(curr_relation_data) = curr_blob_data.downcast_mut::<crate::textplan::common::structured_symbol_data::RelationData>() {
                                    curr_relation_data.pipeline_start = Some(subquery_root.clone());
                                    current = curr_relation_data.continuing_pipeline.clone();
                                } else {
                                    break;
                                }
                            } else {
                                break;
                            }
                        } else {
                            break;
                        }
                    }
                }
            }
        }
    }

    /// Look up a field with scope information, returning (field_index, steps_out).
    /// If the field is found in the current relation, steps_out = 0.
    /// If the field is found in a parent relation, steps_out = number of levels up.
    fn lookup_field_with_scope(&mut self, column_name: &str) -> (usize, usize) {
        // First determine if this is an outer reference by checking if the schema belongs to a parent
        let steps_out = self.calculate_steps_out(column_name);

        // If it's an outer reference, look up the field index in the parent relation
        let field_index = if steps_out > 0 {
            println!(
                "      DEBUG: Outer reference detected, looking up '{}' in parent relation",
                column_name
            );
            self.lookup_field_index_in_parent_relation(column_name)
        } else {
            // Look up in current relation
            self.lookup_field_index(column_name)
        };

        (field_index, steps_out)
    }

    /// Look up a field index in the parent relation.
    /// This is used for outer references.
    fn lookup_field_index_in_parent_relation(&mut self, column_name: &str) -> usize {
        if let Some(current_rel) = self.current_relation_scope() {
            // Find the parent relation by checking parent_query_location
            let parent_loc = current_rel.parent_query_location();

            if let Some(parent) = self
                .symbol_table()
                .lookup_symbol_by_location_and_type(parent_loc.as_ref(), SymbolType::Relation)
            {
                println!(
                    "      DEBUG: Looking up '{}' in parent '{}'",
                    column_name,
                    parent.name()
                );

                // Ensure parent's field_references are populated
                if let Some(blob_lock) = &parent.blob {
                    if let Ok(blob_data) = blob_lock.lock() {
                        if let Some(relation_data) = blob_data.downcast_ref::<crate::textplan::common::structured_symbol_data::RelationData>() {
                            if relation_data.field_references.is_empty() {
                                drop(blob_data);
                                drop(blob_lock);
                                self.add_input_fields_to_schema(&parent);
                            }
                        }
                    }
                }

                // Now look up the field in the parent using the same logic as lookup_field_index
                // Parse column name - can be "field" or "schema.field"
                let (schema_name, field_name) = if let Some(dot_pos) = column_name.rfind('.') {
                    (&column_name[..dot_pos], &column_name[dot_pos + 1..])
                } else {
                    ("", column_name)
                };

                if let Some(blob_lock) = &parent.blob {
                    if let Ok(blob_data) = blob_lock.lock() {
                        if let Some(relation_data) = blob_data.downcast_ref::<crate::textplan::common::structured_symbol_data::RelationData>() {
                            // Search through field_references
                            for (index, field_sym) in relation_data.field_references.iter().enumerate() {
                                if self.field_matches(field_sym, field_name, schema_name, column_name) {
                                    println!("      DEBUG: Found '{}' at index {} in parent '{}'", column_name, index, parent.name());
                                    return index;
                                }
                            }

                            // Also check generated_field_references
                            let field_ref_size = relation_data.field_references.len();
                            for (index, field_sym) in relation_data.generated_field_references.iter().enumerate() {
                                if self.field_matches(field_sym, field_name, schema_name, column_name) {
                                    let actual_index = field_ref_size + index;
                                    println!("      DEBUG: Found '{}' at index {} in parent '{}' (generated)", column_name, actual_index, parent.name());
                                    return actual_index;
                                }
                            }

                            println!("      WARNING: Field '{}' not found in parent '{}', defaulting to 0", column_name, parent.name());
                        }
                    }
                }
            } else {
                println!("      WARNING: Could not find parent relation for outer reference '{}', defaulting to 0", column_name);
            }
        }

        // Default to 0 if not found
        0
    }

    /// Calculate steps_out for a field reference to determine if it's an outer reference.
    /// Returns 0 if the field belongs to the current relation.
    /// Returns 1 if the field belongs to the parent relation (outer reference).
    fn calculate_steps_out(&self, column_name: &str) -> usize {
        // Parse column name to extract schema prefix
        let (schema_name, _field_name) = if let Some(dot_pos) = column_name.rfind('.') {
            (&column_name[..dot_pos], &column_name[dot_pos + 1..])
        } else {
            // No schema prefix - assume it's in current scope
            println!(
                "      DEBUG calculate_steps_out: no schema prefix for '{}', assuming local",
                column_name
            );
            return 0;
        };

        println!(
            "      DEBUG calculate_steps_out: checking '{}' with schema '{}'",
            column_name, schema_name
        );

        // If there's a schema prefix, check if it belongs to current or parent relation
        if let Some(current_rel) = self.current_relation_scope() {
            println!("      DEBUG: current relation is '{}'", current_rel.name());

            if let Some(schema_symbol) = self.symbol_table().lookup_symbol_by_name(schema_name) {
                println!(
                    "      DEBUG: found schema symbol '{}'",
                    schema_symbol.name()
                );

                // Check if schema belongs to current relation
                if let Some(blob_lock) = &current_rel.blob {
                    if let Ok(blob_data) = blob_lock.lock() {
                        if let Some(relation_data) = blob_data.downcast_ref::<crate::textplan::common::structured_symbol_data::RelationData>() {
                            if let Some(current_schema) = &relation_data.schema {
                                println!("      DEBUG: current relation schema is '{}'", current_schema.name());
                                if Arc::ptr_eq(current_schema, &schema_symbol) {
                                    // Schema belongs to current relation - not an outer reference
                                    println!("      DEBUG: schema matches current relation, not an outer ref");
                                    return 0;
                                }
                            } else {
                                println!("      DEBUG: current relation has no schema");
                            }

                            // Schema doesn't match current relation - check if we're in a subquery
                            // Get parent query location
                            println!("      DEBUG: checking parent_query_index = {}", current_rel.parent_query_index());
                            let parent_query_location = if current_rel.parent_query_index() >= 0 {
                                println!("      DEBUG: using current relation's parent_query_location");
                                Some(current_rel.parent_query_location())
                            } else if let Some(pipeline_start) = &relation_data.pipeline_start {
                                println!("      DEBUG: checking pipeline_start '{}', parent_index = {}",
                                    pipeline_start.name(), pipeline_start.parent_query_index());
                                if pipeline_start.parent_query_index() >= 0 {
                                    Some(pipeline_start.parent_query_location())
                                } else {
                                    None
                                }
                            } else {
                                println!("      DEBUG: no pipeline_start");
                                None
                            };

                            if let Some(parent_location) = parent_query_location {
                                println!("      DEBUG: found parent query, looking up parent relation");
                                // We're in a subquery - check if schema belongs to parent
                                if let Some(parent_rel) = self.symbol_table().lookup_symbol_by_location_and_type(
                                    parent_location.as_ref(),
                                    SymbolType::Relation,
                                ) {
                                    println!("      DEBUG: parent relation is '{}'", parent_rel.name());
                                    if let Some(parent_blob_lock) = &parent_rel.blob {
                                        if let Ok(parent_blob_data) = parent_blob_lock.lock() {
                                            if let Some(parent_relation_data) = parent_blob_data.downcast_ref::<crate::textplan::common::structured_symbol_data::RelationData>() {
                                                if let Some(parent_schema) = &parent_relation_data.schema {
                                                    println!("      DEBUG: parent schema is '{}'", parent_schema.name());
                                                    if Arc::ptr_eq(parent_schema, &schema_symbol) {
                                                        // Schema belongs to parent - this is an outer reference!
                                                        println!("      ✓✓✓ '{}' IS AN OUTER REFERENCE (steps_out=1)", column_name);
                                                        return 1;
                                                    } else {
                                                        println!("      DEBUG: parent schema doesn't match field schema");
                                                    }
                                                } else {
                                                    println!("      DEBUG: parent relation has no schema");
                                                }
                                            }
                                        }
                                    }
                                } else {
                                    println!("      DEBUG: couldn't find parent relation");
                                }
                            } else {
                                println!("      DEBUG: not in a subquery (no parent_query_location)");
                            }
                        }
                    }
                }
            } else {
                println!("      DEBUG: schema symbol '{}' not found", schema_name);
            }
        } else {
            println!("      DEBUG: no current relation scope");
        }

        // Not an outer reference
        println!("      DEBUG: '{}' is NOT an outer reference", column_name);
        0
    }

    /// Look up the field index from the current relation's field_references.
    /// This returns the index within the field_references vector, which is the
    /// position relative to this relation's input, not the absolute schema position.
    /// Falls back to schema-based lookup if field_references is not yet populated.
    /// Check if a field symbol matches the given name criteria
    /// Matches on: alias, name, or fully qualified name (schema.field)
    fn field_matches(
        &self,
        field_symbol: &Arc<SymbolInfo>,
        field_name: &str,
        schema_name: &str,
        full_column_name: &str,
    ) -> bool {
        // Check alias first (if present)
        if let Some(alias) = field_symbol.alias() {
            if alias == field_name || alias == full_column_name {
                return true;
            }
        }

        // Check field name
        if field_symbol.name() == field_name {
            // If schema prefix provided, verify it matches
            if !schema_name.is_empty() {
                if let Some(field_schema) = field_symbol.schema() {
                    return field_schema.name() == schema_name;
                }
                return false;
            }
            return true;
        }

        // Check fully qualified name (schema.field)
        if !schema_name.is_empty() {
            if let Some(field_schema) = field_symbol.schema() {
                let qualified = format!("{}.{}", field_schema.name(), field_symbol.name());
                if qualified == full_column_name {
                    return true;
                }
            }
        }

        false
    }

    fn lookup_field_index(&mut self, column_name: &str) -> usize {
        // Parse column name - can be "field" or "schema.field"
        let (schema_name, field_name) = if let Some(dot_pos) = column_name.rfind('.') {
            (&column_name[..dot_pos], &column_name[dot_pos + 1..])
        } else {
            // No schema prefix
            ("", column_name)
        };

        // Try to look up from current relation's field references
        // Search order (like C++): generated_field_references first, then field_references
        if let Some(relation_symbol) = self.current_relation_scope().cloned() {
            // Lazily populate field_references if empty (ensures all schemas are linked first)
            if let Some(blob_lock) = &relation_symbol.blob {
                if let Ok(blob_data) = blob_lock.lock() {
                    if let Some(relation_data) = blob_data.downcast_ref::<crate::textplan::common::structured_symbol_data::RelationData>() {
                        if relation_data.field_references.is_empty() && relation_data.generated_field_references.is_empty() {
                            drop(blob_data);
                            drop(blob_lock);
                            self.add_input_fields_to_schema(&relation_symbol);
                        }
                    }
                }
            }

            // Ensure generated field references from upstream are available
            // This must be done even if field_references is already populated
            if let Some(blob_lock) = &relation_symbol.blob {
                if let Ok(blob_data) = blob_lock.lock() {
                    if let Some(relation_data) = blob_data.downcast_ref::<crate::textplan::common::structured_symbol_data::RelationData>() {
                        // Get upstream relations to populate their generated fields
                        let continuing = relation_data.continuing_pipeline.clone();
                        let new_pipes = relation_data.new_pipelines.clone();
                        drop(blob_data);
                        drop(blob_lock);

                        // Ensure upstream relations have their generated fields populated
                        if let Some(cont) = continuing {
                            self.add_expressions_to_schema(&cont);
                        }
                        for pipe in &new_pipes {
                            self.add_expressions_to_schema(pipe);
                        }
                    }
                }
            }

            if let Some(blob_lock) = &relation_symbol.blob {
                if let Ok(blob_data) = blob_lock.lock() {
                    if let Some(relation_data) = blob_data.downcast_ref::<crate::textplan::common::structured_symbol_data::RelationData>() {
                        // Following C++ behavior: For aggregates during emit processing,
                        // set field_ref_size to 0 so lookups only find generated fields
                        // (C++ SubstraitPlanRelationVisitor::findFieldReferenceByName lines 2036-2038)
                        let is_aggregate = matches!(&relation_data.relation.rel_type, Some(::substrait::proto::rel::RelType::Aggregate(_)));
                        let field_ref_size = if is_aggregate && self.processing_emit {
                            0  // Aggregates during emit only expose generated fields
                        } else {
                            relation_data.field_references.len()
                        };

                        println!("      Looking up '{}' in relation '{}': field_refs={}, generated={}, processing_emit={}",
                            column_name, relation_symbol.name(), field_ref_size, relation_data.generated_field_references.len(), self.processing_emit);

                        // First search generated_field_references (in reverse order, like C++)
                        for (rev_idx, field_symbol) in relation_data.generated_field_references.iter().rev().enumerate() {
                            if self.field_matches(field_symbol, field_name, schema_name, column_name) {
                                let actual_idx = relation_data.generated_field_references.len() - rev_idx - 1;
                                return field_ref_size + actual_idx;
                            }
                        }

                        // Then search field_references (in reverse order, like C++)
                        for (rev_idx, field_symbol) in relation_data.field_references.iter().rev().enumerate() {
                            if self.field_matches(field_symbol, field_name, schema_name, column_name) {
                                return relation_data.field_references.len() - rev_idx - 1;
                            }
                        }

                        // Not found in either list
                        if field_ref_size > 0 || !relation_data.generated_field_references.is_empty() {
                            println!(
                                "      WARNING: Field '{}' not found (field_refs={}, generated={}), defaulting to index 0",
                                column_name, field_ref_size, relation_data.generated_field_references.len()
                            );
                            return 0;
                        }
                        // Fall through to schema-based lookup
                    }
                }
            }
        }

        // Fallback: Look up from schema (for parser, before field_references is populated)
        if !schema_name.is_empty() {
            if let Some(schema_symbol) = self.symbol_table().lookup_symbol_by_name(schema_name) {
                if let Some(field_index) =
                    self.get_field_index_from_schema(&schema_symbol, field_name)
                {
                    return field_index;
                }
            }
        } else {
            // No schema prefix - try current relation's schema
            if let Some(relation_symbol) = self.current_relation_scope() {
                if let Some(blob_lock) = &relation_symbol.blob {
                    if let Ok(blob_data) = blob_lock.lock() {
                        if let Some(relation_data) = blob_data.downcast_ref::<crate::textplan::common::structured_symbol_data::RelationData>() {
                            if let Some(schema_arc) = &relation_data.schema {
                                if let Some(field_index) = self.get_field_index_from_schema(schema_arc, field_name) {
                                    return field_index;
                                }
                            }
                        }
                    }
                }
            }
        }

        println!(
            "      WARNING: Field '{}' not found, defaulting to index 0",
            column_name
        );
        // Default to 0 if not found
        0
    }

    /// Get field index from a schema symbol by looking up the field name
    fn get_field_index_from_schema(
        &self,
        schema_symbol: &Arc<SymbolInfo>,
        field_name: &str,
    ) -> Option<usize> {
        // Iterate through all symbols in the symbol table to find schema columns
        // that belong to this schema
        let mut index = 0;
        for symbol in self.symbol_table().symbols() {
            if symbol.symbol_type() == SymbolType::SchemaColumn {
                // Check if this column belongs to our schema
                if let Some(sym_schema) = symbol.schema() {
                    if Arc::ptr_eq(&sym_schema, schema_symbol) {
                        // This column belongs to our schema - check if name matches
                        if symbol.name() == field_name {
                            return Some(index);
                        }
                        index += 1;
                    }
                }
            }
        }
        None
    }

    /// Build a function call expression from a function use context
    fn build_function_call(
        &mut self,
        ctx: &ExpressionFunctionUseContext<'input>,
    ) -> ::substrait::proto::Expression {
        // Get the function name
        let function_name = ctx
            .id()
            .map(|id| id.get_text())
            .unwrap_or_else(|| "unknown".to_string());

        println!("    Function call: {}", function_name);

        // Look up function reference from symbol table
        let function_reference = self.lookup_function_reference(&function_name);
        println!("      -> function reference: {}", function_reference);

        // Recursively build arguments
        // Check if an expression is actually an enum argument (ends with _enum)
        let mut arguments = Vec::new();
        for expr_ctx in ctx.expression_all() {
            use crate::textplan::parser::antlr::substraitplanparser::ExpressionContextAll;

            // Check if this is a column reference ending in _enum
            if let ExpressionContextAll::ExpressionColumnContext(column_expr_ctx) =
                expr_ctx.as_ref()
            {
                if let Some(column_ctx) = column_expr_ctx.column_name() {
                    let column_text = column_ctx.get_text();
                    if column_text.ends_with("_enum") {
                        // This is an enum argument, not an expression
                        let enum_value = column_text.strip_suffix("_enum").unwrap().to_string();
                        println!("      Enum argument: {}", enum_value);
                        arguments.push(::substrait::proto::FunctionArgument {
                            arg_type: Some(::substrait::proto::function_argument::ArgType::Enum(
                                enum_value,
                            )),
                        });
                        continue;
                    }
                }
            }

            // Not an enum, build as expression
            let arg_expr = self.build_expression(&expr_ctx);
            arguments.push(::substrait::proto::FunctionArgument {
                arg_type: Some(::substrait::proto::function_argument::ArgType::Value(
                    arg_expr,
                )),
            });
        }

        println!("      with {} arguments", arguments.len());

        // Extract output type if present (from ARROW literal_complex_type)
        let output_type = if let Some(type_ctx) = ctx.literal_complex_type() {
            let type_text = type_ctx.get_text();
            // Create a temporary TypeVisitor to parse the type
            let type_visitor =
                TypeVisitor::new(self.symbol_table.clone(), self.error_listener.clone());
            Some(type_visitor.text_to_type_proto(ctx, &type_text))
        } else {
            None
        };

        ::substrait::proto::Expression {
            rex_type: Some(::substrait::proto::expression::RexType::ScalarFunction(
                ::substrait::proto::expression::ScalarFunction {
                    function_reference,
                    arguments,
                    output_type,
                    options: Vec::new(),
                    ..Default::default()
                },
            )),
        }
    }

    /// Build a cast expression from a cast context.
    fn build_cast_expression(
        &mut self,
        ctx: &ExpressionCastContext<'input>,
    ) -> ::substrait::proto::Expression {
        // Get the expression being cast
        let input_expr = if let Some(expr) = ctx.expression() {
            Box::new(self.build_expression(&expr))
        } else {
            // No input expression - return placeholder
            return ::substrait::proto::Expression {
                rex_type: Some(::substrait::proto::expression::RexType::Literal(
                    ::substrait::proto::expression::Literal {
                        literal_type: Some(
                            ::substrait::proto::expression::literal::LiteralType::I64(0),
                        ),
                        nullable: false,
                        type_variation_reference: 0,
                    },
                )),
            };
        };

        // Get the target type
        let mut target_type = if let Some(type_ctx) = ctx.literal_complex_type() {
            let type_text = type_ctx.get_text();
            // Create a temporary TypeVisitor to parse the type
            let type_visitor =
                TypeVisitor::new(self.symbol_table.clone(), self.error_listener.clone());
            type_visitor.text_to_type_proto(ctx, &type_text)
        } else {
            // No target type - return placeholder
            return ::substrait::proto::Expression {
                rex_type: Some(::substrait::proto::expression::RexType::Literal(
                    ::substrait::proto::expression::Literal {
                        literal_type: Some(
                            ::substrait::proto::expression::literal::LiteralType::I64(0),
                        ),
                        nullable: false,
                        type_variation_reference: 0,
                    },
                )),
            };
        };

        // Special handling: if casting a string literal to fixedchar/varchar without explicit length,
        // infer the length from the string
        if let Some(::substrait::proto::expression::RexType::Literal(literal)) =
            &input_expr.rex_type
        {
            if let Some(::substrait::proto::expression::literal::LiteralType::String(
                string_value,
            )) = &literal.literal_type
            {
                use ::substrait::proto::r#type::Kind;
                match &mut target_type.kind {
                    Some(Kind::FixedChar(ref mut fc_type)) if fc_type.length == 0 => {
                        // Infer length from string
                        fc_type.length = string_value.len() as i32;
                        println!("    Inferred fixedchar length: {}", fc_type.length);
                    }
                    Some(Kind::Varchar(ref mut vc_type)) if vc_type.length == 0 => {
                        // Infer length from string
                        vc_type.length = string_value.len() as i32;
                        println!("    Inferred varchar length: {}", vc_type.length);
                    }
                    _ => {}
                }
            }
        }

        ::substrait::proto::Expression {
            rex_type: Some(::substrait::proto::expression::RexType::Cast(Box::new(
                ::substrait::proto::expression::Cast {
                    r#type: Some(target_type),
                    input: Some(input_expr),
                    failure_behavior:
                        ::substrait::proto::expression::cast::FailureBehavior::Unspecified as i32,
                },
            ))),
        }
    }

    /// Extract a numeric value from a constant context.
    fn extract_number_from_constant(constant_ctx: &Rc<ConstantContextAll<'input>>) -> i32 {
        if let Some(number_token) = constant_ctx.NUMBER() {
            number_token.get_text().parse::<i32>().unwrap_or(0)
        } else {
            0
        }
    }

    /// Build a constant literal expression from a constant AST node
    fn build_constant(
        &self,
        constant_ctx: &Rc<ConstantContextAll<'input>>,
    ) -> ::substrait::proto::Expression {
        use ::substrait::proto::expression::literal::LiteralType;

        // Check what type of constant this is
        let literal_type = if let Some(number_token) = constant_ctx.NUMBER() {
            // Parse number literal
            let number_text = number_token.get_text();

            // Check if there's a type suffix (e.g., _date, _decimal<3,2>)
            if let Some(type_ctx) = constant_ctx.literal_basic_type() {
                // Get the type name
                let type_text = type_ctx.get_text();

                // Parse the type name
                let type_name = if let Some(id_ctx) = type_ctx.id() {
                    id_ctx.get_text().to_lowercase()
                } else {
                    "".to_string()
                };

                // Handle different literal types based on type name
                match type_name.as_str() {
                    "date" => {
                        if let Ok(days) = number_text.parse::<i32>() {
                            Some(LiteralType::Date(days))
                        } else {
                            Some(LiteralType::Date(0))
                        }
                    }
                    "time" => {
                        if let Ok(micros) = number_text.parse::<i64>() {
                            Some(LiteralType::Time(micros))
                        } else {
                            Some(LiteralType::Time(0))
                        }
                    }
                    "interval_year" => {
                        // Parse as years directly
                        if let Ok(years) = number_text.parse::<i32>() {
                            Some(LiteralType::IntervalYearToMonth(
                                ::substrait::proto::expression::literal::IntervalYearToMonth {
                                    years,
                                    months: 0,
                                },
                            ))
                        } else {
                            Some(LiteralType::IntervalYearToMonth(
                                ::substrait::proto::expression::literal::IntervalYearToMonth {
                                    years: 0,
                                    months: 0,
                                },
                            ))
                        }
                    }
                    "interval_year_month" => {
                        // Parse as total months, convert to years/months
                        if let Ok(total_months) = number_text.parse::<i32>() {
                            let years = total_months / 12;
                            let months = total_months % 12;
                            Some(LiteralType::IntervalYearToMonth(
                                ::substrait::proto::expression::literal::IntervalYearToMonth {
                                    years,
                                    months,
                                },
                            ))
                        } else {
                            Some(LiteralType::IntervalYearToMonth(
                                ::substrait::proto::expression::literal::IntervalYearToMonth {
                                    years: 0,
                                    months: 0,
                                },
                            ))
                        }
                    }
                    "decimal" => {
                        // Parse the numeric value and convert to i128 bytes
                        if let Ok(value) = number_text.parse::<i128>() {
                            let bytes = value.to_le_bytes().to_vec();

                            // Get precision and scale from literal_specifier if present
                            let (precision, scale) =
                                if let Some(spec_ctx) = type_ctx.literal_specifier() {
                                    // Parse numbers from the specifier (<precision,scale>)
                                    let numbers: Vec<i32> = spec_ctx
                                        .NUMBER_all()
                                        .iter()
                                        .filter_map(|n| n.get_text().parse::<i32>().ok())
                                        .collect();
                                    if numbers.len() >= 2 {
                                        (numbers[0], numbers[1])
                                    } else if numbers.len() == 1 {
                                        (numbers[0], 0)
                                    } else {
                                        (38, 0)
                                    }
                                } else {
                                    (38, 0) // Default precision and scale
                                };

                            Some(LiteralType::Decimal(
                                ::substrait::proto::expression::literal::Decimal {
                                    value: bytes,
                                    precision,
                                    scale,
                                },
                            ))
                        } else {
                            Some(LiteralType::Decimal(
                                ::substrait::proto::expression::literal::Decimal {
                                    value: vec![0; 16],
                                    precision: 38,
                                    scale: 0,
                                },
                            ))
                        }
                    }
                    "i8" => {
                        if let Ok(val) = number_text.parse::<i32>() {
                            Some(LiteralType::I8(val))
                        } else {
                            Some(LiteralType::I8(0))
                        }
                    }
                    "i16" => {
                        if let Ok(val) = number_text.parse::<i32>() {
                            Some(LiteralType::I16(val))
                        } else {
                            Some(LiteralType::I16(0))
                        }
                    }
                    "i32" => {
                        if let Ok(val) = number_text.parse::<i32>() {
                            Some(LiteralType::I32(val))
                        } else {
                            Some(LiteralType::I32(0))
                        }
                    }
                    "i64" => {
                        if let Ok(val) = number_text.parse::<i64>() {
                            Some(LiteralType::I64(val))
                        } else {
                            Some(LiteralType::I64(0))
                        }
                    }
                    "fp32" => {
                        if let Ok(val) = number_text.parse::<f32>() {
                            Some(LiteralType::Fp32(val))
                        } else {
                            Some(LiteralType::Fp32(0.0))
                        }
                    }
                    "fp64" => {
                        if let Ok(val) = number_text.parse::<f64>() {
                            Some(LiteralType::Fp64(val))
                        } else {
                            Some(LiteralType::Fp64(0.0))
                        }
                    }
                    _ => {
                        // Unknown type, default to i64
                        if let Ok(val) = number_text.parse::<i64>() {
                            Some(LiteralType::I64(val))
                        } else {
                            Some(LiteralType::I64(0))
                        }
                    }
                }
            } else {
                // No type suffix, try to parse as i32 first, then i64
                if let Ok(val) = number_text.parse::<i32>() {
                    Some(LiteralType::I32(val))
                } else if let Ok(val) = number_text.parse::<i64>() {
                    Some(LiteralType::I64(val))
                } else {
                    Some(LiteralType::I64(0))
                }
            }
        } else if let Some(string_token) = constant_ctx.STRING() {
            // Parse string literal (remove quotes)
            let string_text = string_token.get_text();
            let string_value = if string_text.starts_with('"') && string_text.ends_with('"') {
                string_text[1..string_text.len() - 1].to_string()
            } else {
                string_text.to_string()
            };

            // Check for type suffix for FixedChar or VarChar
            if let Some(type_ctx) = constant_ctx.literal_basic_type() {
                if let Some(id_ctx) = type_ctx.id() {
                    let type_name = id_ctx.get_text().to_lowercase();
                    match type_name.as_str() {
                        "fixedchar" => Some(LiteralType::FixedChar(string_value)),
                        "varchar" => Some(LiteralType::VarChar(
                            ::substrait::proto::expression::literal::VarChar {
                                value: string_value,
                                length: 0,
                            },
                        )),
                        _ => Some(LiteralType::String(string_value)),
                    }
                } else {
                    Some(LiteralType::String(string_value))
                }
            } else {
                Some(LiteralType::String(string_value))
            }
        } else if constant_ctx.TRUEVAL().is_some() {
            Some(LiteralType::Boolean(true))
        } else if constant_ctx.FALSEVAL().is_some() {
            Some(LiteralType::Boolean(false))
        } else if constant_ctx.NULLVAL().is_some() {
            Some(LiteralType::Null(::substrait::proto::Type::default()))
        } else if let Some(struct_ctx) = constant_ctx.struct_literal() {
            // Handle struct literals - check if it has an interval type suffix
            // NOTE: struct_literal uses literal_complex_type, not literal_basic_type (per grammar line 108)
            if let Some(type_ctx) = constant_ctx.literal_complex_type() {
                // Extract the type name from the complex type
                let type_text = type_ctx.get_text();
                let type_name = type_text.to_lowercase();

                // Parse the constants inside the struct
                let constants = struct_ctx.constant_all();

                match type_name.as_str() {
                    "interval_day_second" => {
                        // Expect {days, seconds, microseconds}_interval_day_second (deprecated format)
                        // IMPORTANT: Use PrecisionMode::Microseconds with microseconds field for compatibility
                        // The proto has both old (microseconds) and new (precision+subseconds) forms
                        if constants.len() >= 3 {
                            let days = Self::extract_number_from_constant(&constants[0]);
                            let seconds = Self::extract_number_from_constant(&constants[1]);
                            let microseconds = Self::extract_number_from_constant(&constants[2]);

                            use ::substrait::proto::expression::literal::interval_day_to_second::PrecisionMode;
                            Some(LiteralType::IntervalDayToSecond(
                                ::substrait::proto::expression::literal::IntervalDayToSecond {
                                    days,
                                    seconds,
                                    subseconds: 0, // Must be 0 when using microseconds
                                    precision_mode: Some(PrecisionMode::Microseconds(microseconds)),
                                },
                            ))
                        } else {
                            // Not enough components - create zero interval
                            use ::substrait::proto::expression::literal::interval_day_to_second::PrecisionMode;
                            Some(LiteralType::IntervalDayToSecond(
                                ::substrait::proto::expression::literal::IntervalDayToSecond {
                                    days: 0,
                                    seconds: 0,
                                    subseconds: 0,
                                    precision_mode: Some(PrecisionMode::Microseconds(0)),
                                },
                            ))
                        }
                    }
                    "interval_year_month" => {
                        // Expect {years, months}
                        if constants.len() >= 2 {
                            let years = Self::extract_number_from_constant(&constants[0]);
                            let months = Self::extract_number_from_constant(&constants[1]);

                            Some(LiteralType::IntervalYearToMonth(
                                ::substrait::proto::expression::literal::IntervalYearToMonth {
                                    years,
                                    months,
                                },
                            ))
                        } else {
                            // Not enough components
                            Some(LiteralType::IntervalYearToMonth(
                                ::substrait::proto::expression::literal::IntervalYearToMonth {
                                    years: 0,
                                    months: 0,
                                },
                            ))
                        }
                    }
                    _ => {
                        // Unknown struct literal type
                        Some(LiteralType::I64(0))
                    }
                }
            } else {
                // Struct without type suffix
                Some(LiteralType::I64(0))
            }
        } else {
            // TODO: Handle map_literal
            // For now, default to i64(0)
            Some(LiteralType::I64(0))
        };

        ::substrait::proto::Expression {
            rex_type: Some(::substrait::proto::expression::RexType::Literal(
                ::substrait::proto::expression::Literal {
                    literal_type,
                    nullable: false,
                    type_variation_reference: 0,
                },
            )),
        }
    }

    /// Look up the function reference (anchor) from the symbol table
    fn lookup_function_reference(&self, function_name: &str) -> u32 {
        // Iterate through all symbols to find functions with matching name
        for symbol in self.symbol_table().symbols() {
            if symbol.symbol_type() == SymbolType::Function {
                // Check if the symbol name (alias) matches what we're looking for
                if symbol.name() == function_name {
                    // Get the function data from the blob to get the anchor
                    if let Some(blob_lock) = &symbol.blob {
                        if let Ok(blob_data) = blob_lock.lock() {
                            if let Some(function_data) = blob_data.downcast_ref::<crate::textplan::common::structured_symbol_data::FunctionData>() {
                                return function_data.anchor;
                            }
                        }
                    }
                }
            }
        }

        // Function not found - use default reference 0
        0
    }

    /// Build a set comparison subquery expression (e.g., expression LT ANY SUBQUERY relation)
    fn build_set_comparison_subquery(
        &mut self,
        ctx: &ExpressionSetComparisonSubqueryContext<'input>,
    ) -> ::substrait::proto::Expression {
        use ::substrait::proto::expression::subquery::set_comparison::{ComparisonOp, ReductionOp};

        // Extract left expression
        let left_expr = if let Some(expr) = ctx.expression() {
            Some(Box::new(self.build_expression(&expr)))
        } else {
            None
        };

        // Extract comparison operator
        let comp_op_text = ctx
            .COMPARISON()
            .map(|t| t.get_text().to_uppercase())
            .unwrap_or_default();
        let comparison_op = match comp_op_text.as_str() {
            "LT" | "<" => ComparisonOp::Lt as i32,
            "GT" | ">" => ComparisonOp::Gt as i32,
            "EQ" | "=" | "==" => ComparisonOp::Eq as i32,
            "NE" | "!=" | "<>" => ComparisonOp::Ne as i32,
            "LE" | "<=" => ComparisonOp::Le as i32,
            "GE" | ">=" => ComparisonOp::Ge as i32,
            _ => ComparisonOp::Unspecified as i32,
        };

        // Extract reduction operator (ANY or ALL)
        let reduction_op = if ctx.ANY().is_some() {
            ReductionOp::Any as i32
        } else if ctx.ALL().is_some() {
            ReductionOp::All as i32
        } else {
            ReductionOp::Unspecified as i32
        };

        // Extract subquery relation reference
        let relation_name = ctx
            .relation_ref()
            .map(|r| r.get_text())
            .unwrap_or_else(|| "unknown".to_string());

        println!(
            "    Set comparison subquery: left={:?}, comp_op={}, reduction_op={}, relation={}",
            left_expr.is_some(),
            comparison_op,
            reduction_op,
            relation_name
        );

        // Look up the subquery relation in the symbol table and mark it as a subquery
        if let Some(rel_symbol) = self.symbol_table().lookup_symbol_by_name(&relation_name) {
            // Mark this relation as a subquery by setting its parent query info
            // This will prevent it from being output as a top-level PlanRel in save_binary
            if let Some(parent_rel) = self.current_relation_scope() {
                println!(
                    "      Marking '{}' as subquery of '{}'",
                    relation_name,
                    parent_rel.name()
                );
                rel_symbol.set_parent_query_location(parent_rel.source_location().box_clone());
                rel_symbol.set_parent_query_index(0);
            }
        } else {
            println!(
                "      WARNING: Subquery relation '{}' not found",
                relation_name
            );
        }

        // NOTE: Following Rust implementation philosophy, we do NOT copy the Rel protobuf here.
        // Instead, we leave `right` as None. Later, save_binary will build the Rel from the
        // symbol tree using add_inputs_to_relation, ensuring inputs come from pipeline connections.
        let right_rel = None;

        ::substrait::proto::Expression {
            rex_type: Some(::substrait::proto::expression::RexType::Subquery(Box::new(
                ::substrait::proto::expression::Subquery {
                    subquery_type: Some(
                        ::substrait::proto::expression::subquery::SubqueryType::SetComparison(
                            Box::new(::substrait::proto::expression::subquery::SetComparison {
                                left: left_expr,
                                comparison_op,
                                reduction_op,
                                right: right_rel,
                            }),
                        ),
                    ),
                },
            ))),
        }
    }

    /// Build a scalar subquery expression (e.g., SUBQUERY relation)
    fn build_scalar_subquery(
        &mut self,
        ctx: &ExpressionScalarSubqueryContext<'input>,
    ) -> ::substrait::proto::Expression {
        // Get the relation reference name (e.g., "SUBQUERY some_relation")
        if let Some(rel_ref) = ctx.relation_ref() {
            let relation_name = rel_ref.get_text();
            println!("    Scalar subquery: relation name = '{}'", relation_name);

            // Look up the relation symbol in the symbol table
            if let Some(relation_symbol) = self.symbol_table.lookup_symbol_by_name(&relation_name) {
                // Mark this relation as a subquery by setting its parent query info
                if let Some(parent_rel) = self.current_relation_scope() {
                    let parent_loc = parent_rel.source_location();
                    println!(
                        "      Marking '{}' as scalar subquery of '{}' (location hash: {})",
                        relation_name,
                        parent_rel.name(),
                        parent_loc.location_hash()
                    );
                    relation_symbol.set_parent_query_location(parent_loc.box_clone());
                    relation_symbol.set_parent_query_index(0);

                    // Set pipeline_start on all relations in the subquery pipeline
                    // (following C++ PipelineVisitor pattern)
                    self.set_pipeline_start_for_subquery(&relation_symbol);
                }

                // Get the relation proto from the relation data
                if let Some(blob_lock) = &relation_symbol.blob {
                    if let Ok(blob_data) = blob_lock.lock() {
                        if let Some(relation_data) = blob_data.downcast_ref::<crate::textplan::common::structured_symbol_data::RelationData>() {
                            // Create the scalar subquery expression
                            return ::substrait::proto::Expression {
                                rex_type: Some(::substrait::proto::expression::RexType::Subquery(Box::new(
                                    ::substrait::proto::expression::Subquery {
                                        subquery_type: Some(::substrait::proto::expression::subquery::SubqueryType::Scalar(
                                            Box::new(::substrait::proto::expression::subquery::Scalar {
                                                input: Some(Box::new(relation_data.relation.clone())),
                                            }),
                                        )),
                                    },
                                ))),
                            };
                        }
                    }
                }
            } else {
                println!(
                    "    WARNING: Scalar subquery relation '{}' not found in symbol table",
                    relation_name
                );
            }
        }

        // Fallback: return a placeholder
        println!("    Scalar subquery: failed to build, returning placeholder");
        ::substrait::proto::Expression {
            rex_type: Some(::substrait::proto::expression::RexType::Literal(
                ::substrait::proto::expression::Literal {
                    literal_type: Some(::substrait::proto::expression::literal::LiteralType::I64(
                        0,
                    )),
                    nullable: false,
                    type_variation_reference: 0,
                },
            )),
        }
    }

    /// Build an IN predicate subquery expression (e.g., expression_list IN SUBQUERY relation)
    fn build_in_predicate_subquery(
        &mut self,
        ctx: &ExpressionInPredicateSubqueryContext<'input>,
    ) -> ::substrait::proto::Expression {
        // Extract needle expressions (left-hand side of IN)
        let mut needles = Vec::new();

        // Get expressions from expression_list
        if let Some(expr_list) = ctx.expression_list() {
            for expr in expr_list.expression_all() {
                needles.push(self.build_expression(&expr));
            }
        }

        // Extract subquery relation reference (haystack)
        let relation_name = ctx
            .relation_ref()
            .map(|r| r.get_text())
            .unwrap_or_else(|| "unknown".to_string());

        println!(
            "    IN predicate subquery: {} needles, haystack={}",
            needles.len(),
            relation_name
        );

        // Look up the subquery relation in the symbol table and mark it as a subquery
        if let Some(rel_symbol) = self.symbol_table().lookup_symbol_by_name(&relation_name) {
            // Mark this relation as a subquery by setting its parent query info
            if let Some(parent_rel) = self.current_relation_scope() {
                println!(
                    "      Marking '{}' as IN predicate subquery of '{}'",
                    relation_name,
                    parent_rel.name()
                );
                rel_symbol.set_parent_query_location(parent_rel.source_location().box_clone());
                rel_symbol.set_parent_query_index(0);
            }
        } else {
            println!(
                "      WARNING: IN predicate subquery relation '{}' not found",
                relation_name
            );
        }

        // NOTE: Following the pattern from SetComparison, we do NOT copy the Rel protobuf here.
        // Instead, we leave `haystack` as None. Later, save_binary will build the Rel from the
        // symbol tree using add_inputs_to_relation, ensuring inputs come from pipeline connections.
        let haystack_rel = None;

        ::substrait::proto::Expression {
            rex_type: Some(::substrait::proto::expression::RexType::Subquery(Box::new(
                ::substrait::proto::expression::Subquery {
                    subquery_type: Some(
                        ::substrait::proto::expression::subquery::SubqueryType::InPredicate(
                            Box::new(::substrait::proto::expression::subquery::InPredicate {
                                needles,
                                haystack: haystack_rel,
                            }),
                        ),
                    ),
                },
            ))),
        }
    }

    /// Build a set predicate subquery expression (e.g., EXISTS IN SUBQUERY relation)
    fn build_set_predicate_subquery(
        &mut self,
        ctx: &ExpressionSetPredicateSubqueryContext<'input>,
    ) -> ::substrait::proto::Expression {
        // Determine predicate operator (EXISTS or UNIQUE)
        let predicate_op = if ctx.EXISTS().is_some() {
            ::substrait::proto::expression::subquery::set_predicate::PredicateOp::Exists as i32
        } else if ctx.UNIQUE().is_some() {
            ::substrait::proto::expression::subquery::set_predicate::PredicateOp::Unique as i32
        } else {
            ::substrait::proto::expression::subquery::set_predicate::PredicateOp::Unspecified as i32
        };

        // Extract subquery relation reference
        let relation_name = ctx
            .relation_ref()
            .map(|r| r.get_text())
            .unwrap_or_else(|| "unknown".to_string());

        println!(
            "    SET predicate subquery: op={}, relation={}",
            predicate_op, relation_name
        );

        // Look up the subquery relation in the symbol table and mark it as a subquery
        if let Some(rel_symbol) = self.symbol_table().lookup_symbol_by_name(&relation_name) {
            // Mark this relation as a subquery by setting its parent query info
            if let Some(parent_rel) = self.current_relation_scope() {
                println!(
                    "      Marking '{}' as SET predicate subquery of '{}'",
                    relation_name,
                    parent_rel.name()
                );
                rel_symbol.set_parent_query_location(parent_rel.source_location().box_clone());
                rel_symbol.set_parent_query_index(0);
            }
        } else {
            println!(
                "      WARNING: SET predicate subquery relation '{}' not found",
                relation_name
            );
        }

        // NOTE: Following the pattern from InPredicate, we do NOT copy the Rel protobuf here.
        // Instead, we leave `tuples` as None. Later, save_binary will build the Rel from the
        // symbol tree using add_inputs_to_relation, ensuring inputs come from pipeline connections.
        let tuples_rel = None;

        ::substrait::proto::Expression {
            rex_type: Some(::substrait::proto::expression::RexType::Subquery(Box::new(
                ::substrait::proto::expression::Subquery {
                    subquery_type: Some(
                        ::substrait::proto::expression::subquery::SubqueryType::SetPredicate(
                            Box::new(::substrait::proto::expression::subquery::SetPredicate {
                                predicate_op,
                                tuples: tuples_rel,
                            }),
                        ),
                    ),
                },
            ))),
        }
    }
}

impl<'input> PlanVisitor<'input> for RelationVisitor<'input> {
    fn error_listener(&self) -> Arc<ErrorListener> {
        self.error_listener.clone()
    }

    fn symbol_table(&self) -> SymbolTable {
        self.symbol_table.clone()
    }
}

// ANTLR visitor implementation for RelationVisitor
impl<'input> ParseTreeVisitor<'input, SubstraitPlanParserContextType> for RelationVisitor<'input> {}

impl<'input> SubstraitPlanParserVisitor<'input> for RelationVisitor<'input> {
    // Override specific visitor methods for relation processing

    fn visit_relation(&mut self, ctx: &RelationContext<'input>) {
        // Look up the relation symbol that was already created in MainPlanVisitor phase
        // (C++ does: lookupSymbolByLocationAndType(Location(ctx), SymbolType::kRelation))
        let relation_ref = ctx.relation_ref();
        let symbol = if let Some(rel_ref) = relation_ref {
            if let Some(id) = rel_ref.id(0) {
                let name = id.get_text();
                self.symbol_table.lookup_symbol_by_name(&name)
            } else {
                None
            }
        } else {
            None
        };

        // Set as current scope if found
        if let Some(relation_symbol) = symbol.clone() {
            self.set_current_relation_scope(Some(relation_symbol.clone()));

            // IMPORTANT: Populate upstream generated fields BEFORE visiting children
            // This ensures that when expressions/measures are built, upstream generated
            // fields are already available for lookup
            if let Some(blob_lock) = &relation_symbol.blob {
                if let Ok(blob_data) = blob_lock.lock() {
                    if let Some(relation_data) = blob_data.downcast_ref::<RelationData>() {
                        let continuing = relation_data.continuing_pipeline.clone();
                        let new_pipes = relation_data.new_pipelines.clone();
                        drop(blob_data);
                        drop(blob_lock);

                        if let Some(cont) = continuing {
                            self.add_expressions_to_schema(&cont);
                        }
                        for pipe in &new_pipes {
                            self.add_expressions_to_schema(pipe);
                        }
                    }
                }
            }
        }

        // Continue visiting children to process relation details
        self.visit_children(ctx);

        // Add generated field references from expressions (after visiting relation details)
        // This matches the C++ addExpressionsToSchema pattern
        if let Some(relation_symbol) = symbol.clone() {
            self.add_expressions_to_schema(&relation_symbol);

            // Special handling for aggregates: copy generatedFieldReferences to outputFieldReferences
            // Following C++ SubstraitPlanRelationVisitor::visitRelation lines 387-393
            if let Some(blob_lock) = &relation_symbol.blob {
                if let Ok(blob_data) = blob_lock.lock() {
                    if let Some(relation_data) = blob_data.downcast_ref::<RelationData>() {
                        let is_aggregate = matches!(
                            &relation_data.relation.rel_type,
                            Some(::substrait::proto::rel::RelType::Aggregate(_))
                        );
                        if is_aggregate {
                            let generated_refs = relation_data.generated_field_references.clone();
                            drop(blob_data);
                            drop(blob_lock);

                            // Now update outputFieldReferences
                            if let Some(blob_lock) = &relation_symbol.blob {
                                if let Ok(mut blob_data) = blob_lock.lock() {
                                    if let Some(relation_data) =
                                        blob_data.downcast_mut::<RelationData>()
                                    {
                                        relation_data
                                            .output_field_references
                                            .extend(generated_refs);
                                        println!("  Aggregate '{}': copied {} generated fields to output_field_references",
                                            relation_symbol.name(),
                                            relation_data.output_field_references.len());
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        // Clear scope when done
        self.set_current_relation_scope(None);
    }

    fn visit_root_relation(&mut self, ctx: &Root_relationContext<'input>) {
        // Look up the root symbol that was already created
        // Root relations are handled in MainPlanVisitor, just visit children here
        self.visit_children(ctx);
    }

    fn visit_relation_type(&mut self, ctx: &Relation_typeContext<'input>) {
        // This will be handled by visit_relation
        // Just visit children
        self.visit_children(ctx);
    }

    fn visit_relationFilter(&mut self, ctx: &RelationFilterContext<'input>) {
        // Add filter condition to the current relation (should be a Filter)
        // Grammar: relation_filter_behavior? FILTER expression SEMICOLON
        if let Some(relation_symbol) = self.current_relation_scope().cloned() {
            // Build the filter condition expression from AST
            let condition = if let Some(expr_ctx) = ctx.expression() {
                self.build_expression(&expr_ctx)
            } else {
                // No expression - use placeholder
                ::substrait::proto::Expression {
                    rex_type: Some(::substrait::proto::expression::RexType::Literal(
                        ::substrait::proto::expression::Literal {
                            literal_type: Some(
                                ::substrait::proto::expression::literal::LiteralType::Boolean(true),
                            ),
                            nullable: false,
                            type_variation_reference: 0,
                        },
                    )),
                }
            };

            // Add the condition to the FilterRel
            if let Some(blob_lock) = &relation_symbol.blob {
                if let Ok(mut blob_data) = blob_lock.lock() {
                    if let Some(relation_data) = blob_data.downcast_mut::<crate::textplan::common::structured_symbol_data::RelationData>() {
                        // Get mutable access to the Rel
                        if let Some(::substrait::proto::rel::RelType::Filter(ref mut filter_rel)) = relation_data.relation.rel_type {
                            filter_rel.condition = Some(Box::new(condition));
                            println!("  Added filter condition to filter relation '{}'", relation_symbol.name());
                        }
                    }
                }
            }
        }

        // Visit children to process any nested expressions
        self.visit_children(ctx);
    }

    fn visit_relationExpression(&mut self, ctx: &RelationExpressionContext<'input>) {
        // Add expression to the current relation (Project or Join)
        // Grammar: EXPRESSION expression SEMICOLON
        if let Some(relation_symbol) = self.current_relation_scope().cloned() {
            // Try to build actual expression from AST
            let expression = if let Some(expr_ctx) = ctx.expression() {
                self.build_expression(&expr_ctx)
            } else {
                // Fallback to placeholder if no expression context
                ::substrait::proto::Expression {
                    rex_type: Some(::substrait::proto::expression::RexType::Literal(
                        ::substrait::proto::expression::Literal {
                            literal_type: Some(
                                ::substrait::proto::expression::literal::LiteralType::I64(0),
                            ),
                            nullable: false,
                            type_variation_reference: 0,
                        },
                    )),
                }
            };

            // Add the expression to the appropriate relation type
            if let Some(blob_lock) = &relation_symbol.blob {
                if let Ok(mut blob_data) = blob_lock.lock() {
                    if let Some(relation_data) = blob_data.downcast_mut::<crate::textplan::common::structured_symbol_data::RelationData>() {
                        // Get mutable access to the Rel
                        use ::substrait::proto::rel::RelType;
                        match &mut relation_data.relation.rel_type {
                            Some(RelType::Project(ref mut project_rel)) => {
                                project_rel.expressions.push(expression);
                                println!("  Added expression to project relation '{}'", relation_symbol.name());
                            }
                            Some(RelType::Join(ref mut join_rel)) => {
                                join_rel.expression = Some(Box::new(expression));
                                println!("  Set join expression on join relation '{}'", relation_symbol.name());
                            }
                            Some(RelType::HashJoin(ref mut join_rel)) => {
                                // HashJoinRel uses post_join_filter, not expression
                                join_rel.post_join_filter = Some(Box::new(expression));
                                println!("  Set post-join filter on hash join relation '{}'", relation_symbol.name());
                            }
                            Some(RelType::MergeJoin(ref mut join_rel)) => {
                                // MergeJoinRel uses post_join_filter, not expression
                                join_rel.post_join_filter = Some(Box::new(expression));
                                println!("  Set post-join filter on merge join relation '{}'", relation_symbol.name());
                            }
                            _ => {
                                eprintln!("  Warning: EXPRESSION property used on unsupported relation type");
                            }
                        }
                    }
                }
            }
        }

        // Visit children to process any nested expressions
        self.visit_children(ctx);
    }

    #[allow(deprecated)]
    fn visit_relationGrouping(&mut self, ctx: &RelationGroupingContext<'input>) {
        // Add grouping expressions to the current relation (should be an Aggregate)
        // Grammar: GROUPING expression SEMICOLON
        // NOTE: Using deprecated Grouping.grouping_expressions format for now to match test data
        if let Some(relation_symbol) = self.current_relation_scope().cloned() {
            if let Some(expr_ctx) = ctx.expression() {
                // Build the grouping expression
                let expr = self.build_expression(&expr_ctx);

                // Add to deprecated Grouping.grouping_expressions (old format for roundtrip compatibility)
                if let Some(blob_lock) = &relation_symbol.blob {
                    if let Ok(mut blob_data) = blob_lock.lock() {
                        if let Some(relation_data) = blob_data.downcast_mut::<crate::textplan::common::structured_symbol_data::RelationData>() {
                            if let Some(::substrait::proto::rel::RelType::Aggregate(ref mut agg_rel)) = relation_data.relation.rel_type {
                                // Ensure there's at least one Grouping, or create one
                                if agg_rel.groupings.is_empty() {
                                    agg_rel.groupings.push(::substrait::proto::aggregate_rel::Grouping {
                                        grouping_expressions: Vec::new(),
                                        expression_references: Vec::new(),
                                    });
                                }

                                // Add expression directly to the deprecated field
                                agg_rel.groupings[0].grouping_expressions.push(expr.clone());

                                // Following C++ behavior: If this is a simple field selection,
                                // add the referenced field to generated_field_references
                                if let Some(::substrait::proto::expression::RexType::Selection(ref selection)) = expr.rex_type {
                                    // Check if this is a root reference (field selection from current relation)
                                    if let Some(::substrait::proto::expression::field_reference::RootType::RootReference(_)) = selection.root_type {
                                        // Check if it's a direct struct field reference
                                        if let Some(::substrait::proto::expression::field_reference::ReferenceType::DirectReference(ref ref_segment)) = selection.reference_type {
                                            if let Some(::substrait::proto::expression::reference_segment::ReferenceType::StructField(ref struct_field)) = ref_segment.reference_type {
                                                let field_index = struct_field.field as usize;
                                                if field_index < relation_data.field_references.len() {
                                                    let field_symbol = relation_data.field_references[field_index].clone();
                                                    relation_data.generated_field_references.push(field_symbol);
                                                    println!("  Added grouping field '{}' to generated_field_references",
                                                        relation_data.field_references[field_index].name());
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        // Visit children
        self.visit_children(ctx);
    }

    fn visit_relationSort(&mut self, ctx: &RelationSortContext<'input>) {
        // Add sort field to the current relation (should be a Sort)
        // Grammar: sort_field -> SORT expression (BY id)? SEMICOLON
        if let Some(relation_symbol) = self.current_relation_scope().cloned() {
            if let Some(sort_field_ctx) = ctx.sort_field() {
                if let Some(expr_ctx) = sort_field_ctx.expression() {
                    // Build the sort expression
                    let expr = self.build_expression(&expr_ctx);

                    // Parse the direction if provided
                    let direction = if let Some(id_ctx) = sort_field_ctx.id() {
                        parse_sort_direction(&id_ctx.get_text())
                    } else {
                        // Default to ASC NULLS LAST if not specified
                        ::substrait::proto::sort_field::SortDirection::AscNullsLast as i32
                    };

                    // Create the SortField
                    let sort_field = ::substrait::proto::SortField {
                        expr: Some(expr),
                        sort_kind: Some(::substrait::proto::sort_field::SortKind::Direction(
                            direction,
                        )),
                    };

                    // Add to SortRel
                    if let Some(blob_lock) = &relation_symbol.blob {
                        if let Ok(mut blob_data) = blob_lock.lock() {
                            if let Some(relation_data) = blob_data.downcast_mut::<crate::textplan::common::structured_symbol_data::RelationData>() {
                                if let Some(::substrait::proto::rel::RelType::Sort(ref mut sort_rel)) = relation_data.relation.rel_type {
                                    sort_rel.sorts.push(sort_field);
                                }
                            }
                        }
                    }
                }
            }
        }

        // Visit children
        self.visit_children(ctx);
    }

    fn visit_relationMeasure(&mut self, ctx: &RelationMeasureContext<'input>) {
        // Add measures to the current relation (should be an Aggregate)
        // Grammar: MEASURE LEFTBRACE measure_detail* RIGHTBRACE
        if let Some(relation_symbol) = self.current_relation_scope().cloned() {
            // First pass: collect invocation if specified
            let mut invocation =
                ::substrait::proto::aggregate_function::AggregationInvocation::Unspecified;
            for measure_detail_ctx in ctx.measure_detail_all() {
                if measure_detail_ctx.INVOCATION().is_some() {
                    // Get the invocation value (id after INVOCATION keyword)
                    if let Some(id_ctx) = measure_detail_ctx.id(0) {
                        let invocation_str = id_ctx.get_text().to_lowercase();
                        invocation = match invocation_str.as_str() {
                            "all" => ::substrait::proto::aggregate_function::AggregationInvocation::All,
                            "distinct" => ::substrait::proto::aggregate_function::AggregationInvocation::Distinct,
                            _ => ::substrait::proto::aggregate_function::AggregationInvocation::Unspecified,
                        };
                    }
                }
            }

            // Second pass: process each measure_detail to build actual measures
            for measure_detail_ctx in ctx.measure_detail_all() {
                // Check if this is a MEASURE expression detail (not FILTER, INVOCATION, or sort)
                if let Some(expr_ctx) = measure_detail_ctx.expression() {
                    // For aggregate measures, we need to extract the function reference and arguments
                    // directly instead of building a ScalarFunction expression
                    let (function_reference, arguments, output_type) = match expr_ctx.as_ref() {
                        ExpressionContextAll::ExpressionFunctionUseContext(func_ctx) => {
                            // Get function name and look up reference
                            let function_name = func_ctx
                                .id()
                                .map(|id| id.get_text())
                                .unwrap_or_else(|| "unknown".to_string());
                            let func_ref = self.lookup_function_reference(&function_name);

                            // Build arguments directly
                            let mut args = Vec::new();
                            for arg_expr in func_ctx.expression_all() {
                                let expr = self.build_expression(&arg_expr);
                                args.push(::substrait::proto::FunctionArgument {
                                    arg_type: Some(
                                        ::substrait::proto::function_argument::ArgType::Value(expr),
                                    ),
                                });
                            }

                            // Extract output type if present
                            let out_type = if let Some(type_ctx) = func_ctx.literal_complex_type() {
                                let type_text = type_ctx.get_text();
                                let type_visitor = TypeVisitor::new(
                                    self.symbol_table.clone(),
                                    self.error_listener.clone(),
                                );
                                Some(type_visitor.text_to_type_proto(func_ctx, &type_text))
                            } else {
                                None
                            };

                            (func_ref, args, out_type)
                        }
                        _ => {
                            // For non-function expressions, wrap in arguments
                            let measure_expr = self.build_expression(&expr_ctx);
                            let arg = ::substrait::proto::FunctionArgument {
                                arg_type: Some(
                                    ::substrait::proto::function_argument::ArgType::Value(
                                        measure_expr,
                                    ),
                                ),
                            };
                            (0, vec![arg], None)
                        }
                    };

                    // Create the AggregateFunction
                    let agg_func = ::substrait::proto::AggregateFunction {
                        function_reference,
                        arguments,
                        output_type,
                        phase: ::substrait::proto::AggregationPhase::InitialToResult.into(),
                        invocation: invocation.into(),
                        ..Default::default()
                    };

                    // Create the Measure
                    let measure = ::substrait::proto::aggregate_rel::Measure {
                        measure: Some(agg_func),
                        filter: None,
                    };

                    // Extract NAMED alias if present (grammar: NAMED id)
                    // Check for NAMED keyword followed by an id
                    let measure_alias = if measure_detail_ctx.NAMED().is_some() {
                        // Get all id() contexts - the last one after NAMED is the alias
                        let ids = measure_detail_ctx.id_all();
                        // The grammar is: ... (ATSIGN id)? (NAMED id)?
                        // So if we have NAMED, the last id is the alias
                        if !ids.is_empty() {
                            Some(ids.last().unwrap().get_text())
                        } else {
                            None
                        }
                    } else {
                        None
                    };

                    // Add to the AggregateRel and track the measure index
                    let measure_index = if let Some(blob_lock) = &relation_symbol.blob {
                        if let Ok(mut blob_data) = blob_lock.lock() {
                            if let Some(relation_data) = blob_data.downcast_mut::<crate::textplan::common::structured_symbol_data::RelationData>() {
                                if let Some(::substrait::proto::rel::RelType::Aggregate(ref mut agg_rel)) = relation_data.relation.rel_type {
                                    let idx = agg_rel.measures.len();
                                    agg_rel.measures.push(measure);
                                    Some(idx)
                                } else {
                                    None
                                }
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    } else {
                        None
                    };

                    // If there's a NAMED alias, add it to generated_field_references
                    if let (Some(alias), Some(measure_idx)) = (measure_alias, measure_index) {
                        if let Some(blob_lock) = &relation_symbol.blob {
                            if let Ok(mut blob_data) = blob_lock.lock() {
                                if let Some(relation_data) = blob_data.downcast_mut::<crate::textplan::common::structured_symbol_data::RelationData>() {
                                    // Calculate the field index for this measure
                                    // Aggregate outputs: grouping_fields + measures
                                    // Field index = number_of_grouping_fields + measure_index
                                    if let Some(::substrait::proto::rel::RelType::Aggregate(ref agg_rel)) = relation_data.relation.rel_type {
                                        let num_grouping_fields = agg_rel.groupings.first()
                                            .map(|g| g.grouping_expressions.len())
                                            .unwrap_or(0);
                                        let field_index = num_grouping_fields + measure_idx;

                                        // Create a symbol for this measure alias
                                        // Use get_unique_name to ensure uniqueness (matching converter behavior)
                                        // Use SymbolType::Measure to match converter behavior
                                        let location = TextLocation::new(0, 0);
                                        let unique_alias = self.symbol_table().get_unique_name(&alias);
                                        let measure_symbol = self.symbol_table_mut().define_symbol(
                                            unique_alias.clone(),
                                            location,
                                            SymbolType::Measure,
                                            None,
                                            None,
                                        );

                                        // Add to generated_field_references
                                        // Note: The field index is implicit based on position
                                        // For aggregates: grouping_fields come first, then measures
                                        relation_data.generated_field_references.push(measure_symbol);
                                        println!(
                                            "  Added measure alias '{}' (unique: '{}') as generated field at index {}",
                                            alias, unique_alias, field_index
                                        );
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        // Visit children to process any nested expressions
        self.visit_children(ctx);
    }

    fn visit_relationJoinType(&mut self, ctx: &RelationJoinTypeContext<'input>) {
        // This handles the "TYPE <join_type>;" property inside a join relation definition.
        // Extract the join type (e.g., "LEFT", "INNER", "RIGHT", etc.)
        if let Some(id_node) = ctx.id() {
            let join_type_str = id_node.get_text().to_uppercase();

            // Map the join type string to the protobuf enum value
            let join_type_enum = match join_type_str.as_str() {
                "INNER" => 1,           // JOIN_TYPE_INNER
                "OUTER" => 2,           // JOIN_TYPE_OUTER
                "LEFT" => 3,            // JOIN_TYPE_LEFT
                "RIGHT" => 4,           // JOIN_TYPE_RIGHT
                "LEFT_SEMI" => 5,       // JOIN_TYPE_LEFT_SEMI
                "RIGHT_SEMI" => 6,      // JOIN_TYPE_RIGHT_SEMI
                "LEFT_ANTI" => 7,       // JOIN_TYPE_LEFT_ANTI
                "RIGHT_ANTI" => 8,      // JOIN_TYPE_RIGHT_ANTI
                "LEFT_SINGLE" => 9,     // JOIN_TYPE_LEFT_SINGLE
                "RIGHT_SINGLE" => 10,   // JOIN_TYPE_RIGHT_SINGLE
                "LEFT_MARK" => 11,      // JOIN_TYPE_LEFT_MARK
                "RIGHT_MARK" => 12,     // JOIN_TYPE_RIGHT_MARK
                _ => 0,                 // JOIN_TYPE_UNSPECIFIED
            };

            // Set the join type on the current relation (should be a join relation)
            if let Some(relation_symbol) = self.current_relation_scope() {
                if let Some(blob_lock) = &relation_symbol.blob {
                    if let Ok(mut blob_data) = blob_lock.lock() {
                        if let Some(relation_data) = blob_data.downcast_mut::<RelationData>() {
                            use ::substrait::proto::rel::RelType;
                            match &mut relation_data.relation.rel_type {
                                Some(RelType::Join(join_rel)) => {
                                    join_rel.r#type = join_type_enum;
                                    println!("  Set join type to {} ({})", join_type_str, join_type_enum);
                                }
                                Some(RelType::HashJoin(join_rel)) => {
                                    join_rel.r#type = join_type_enum;
                                    println!("  Set hash join type to {} ({})", join_type_str, join_type_enum);
                                }
                                Some(RelType::MergeJoin(join_rel)) => {
                                    join_rel.r#type = join_type_enum;
                                    println!("  Set merge join type to {} ({})", join_type_str, join_type_enum);
                                }
                                _ => {
                                    eprintln!("  Warning: TYPE property used on non-join relation");
                                }
                            }
                        }
                    }
                }
            }
        }

        // Visit children
        self.visit_children(ctx);
    }

    fn visit_expressionConstant(&mut self, ctx: &ExpressionConstantContext<'input>) {
        // Process the constant expression
        self.process_constant_expression(ctx);

        // Visit children
        self.visit_children(ctx);
    }

    fn visit_expressionColumn(&mut self, ctx: &ExpressionColumnContext<'input>) {
        // Process the column reference expression
        self.process_column_expression(ctx);

        // Visit children
        self.visit_children(ctx);
    }

    fn visit_expressionFunctionUse(&mut self, ctx: &ExpressionFunctionUseContext<'input>) {
        // Process the function call expression
        self.process_function_expression(ctx);

        // Visit children
        self.visit_children(ctx);
    }

    fn visit_expressionCast(&mut self, ctx: &ExpressionCastContext<'input>) {
        // We'll just visit children for now
        self.visit_children(ctx);
    }

    fn visit_relationUsesSchema(&mut self, ctx: &RelationUsesSchemaContext<'input>) {
        // Link the current relation to its schema
        // Grammar: BASE_SCHEMA id SEMICOLON
        if let Some(relation_symbol) = self.current_relation_scope() {
            if let Some(schema_id) = ctx.id() {
                let schema_name = schema_id.get_text();
                if let Some(schema_symbol) = self.symbol_table.lookup_symbol_by_name(&schema_name) {
                    // Set the schema reference in the relation's RelationData
                    if let Some(blob_lock) = &relation_symbol.blob {
                        if let Ok(mut blob_data) = blob_lock.lock() {
                            if let Some(relation_data) = blob_data.downcast_mut::<crate::textplan::common::structured_symbol_data::RelationData>() {
                                relation_data.schema = Some(schema_symbol);
                                println!("  Linked relation '{}' to schema '{}'", relation_symbol.name(), schema_name);
                            }
                        }
                    }
                } else {
                    // Schema not found yet (might be defined later in textplan)
                    // Store the name for later resolution in save_binary
                    if let Some(blob_lock) = &relation_symbol.blob {
                        if let Ok(mut blob_data) = blob_lock.lock() {
                            if let Some(relation_data) = blob_data.downcast_mut::<crate::textplan::common::structured_symbol_data::RelationData>() {
                                relation_data.schema_name = Some(schema_name.to_string());
                            }
                        }
                    }
                }
            }
        }
        self.visit_children(ctx);
    }

    fn visit_relationSourceReference(&mut self, ctx: &RelationSourceReferenceContext<'input>) {
        // Link the current relation to its source
        // Grammar: source_reference SEMICOLON
        if let Some(relation_symbol) = self.current_relation_scope() {
            if let Some(source_ref_ctx) = ctx.source_reference() {
                if let Some(source_id) = source_ref_ctx.id() {
                    let source_name = source_id.get_text();
                    if let Some(source_symbol) =
                        self.symbol_table.lookup_symbol_by_name(&source_name)
                    {
                        // Set the source reference in the relation's RelationData
                        if let Some(blob_lock) = &relation_symbol.blob {
                            if let Ok(mut blob_data) = blob_lock.lock() {
                                if let Some(relation_data) = blob_data.downcast_mut::<crate::textplan::common::structured_symbol_data::RelationData>() {
                                    relation_data.source = Some(source_symbol);
                                    println!("  Linked relation '{}' to source '{}'", relation_symbol.name(), source_name);
                                }
                            }
                        }
                    }
                }
            }
        }
        self.visit_children(ctx);
    }

    fn visit_relationEmit(&mut self, ctx: &RelationEmitContext<'input>) {
        // CRITICAL: Populate generated_field_references BEFORE processing emit
        // Emit indices reference generated fields, so they must exist first
        if let Some(relation_symbol) = self.current_relation_scope().cloned() {
            self.add_expressions_to_schema(&relation_symbol);
        }

        // Mark that we're processing emit (affects field lookup behavior for aggregates)
        // Following C++ SubstraitPlanRelationVisitor which tracks processingEmit_
        self.processing_emit = true;

        // Handle EMIT column_name SEMICOLON
        // Emit specifies which fields from the input should be output
        if let Some(relation_symbol) = self.current_relation_scope().cloned() {
            if let Some(column_name_ctx) = ctx.column_name() {
                // Extract the column name (can be "id" or "id.id")
                let field_name = column_name_ctx.get_text();

                // Look up the field index in the current relation's field space
                let field_index = self.lookup_field_index(&field_name);

                println!(
                    "  Emit field '{}' at index {} in relation '{}'",
                    field_name,
                    field_index,
                    relation_symbol.name()
                );

                // Look up the actual field symbol to add to output_field_references
                if let Some(blob_lock) = &relation_symbol.blob {
                    if let Ok(blob_data) = blob_lock.lock() {
                        if let Some(relation_data) = blob_data.downcast_ref::<crate::textplan::common::structured_symbol_data::RelationData>() {
                            // Find the field symbol at this index
                            let field_symbol = if field_index < relation_data.field_references.len() {
                                Some(relation_data.field_references[field_index].clone())
                            } else {
                                let gen_index = field_index - relation_data.field_references.len();
                                if gen_index < relation_data.generated_field_references.len() {
                                    Some(relation_data.generated_field_references[gen_index].clone())
                                } else {
                                    None
                                }
                            };

                            drop(blob_data);
                            drop(blob_lock);

                            // Add to output_field_references
                            if let Some(field_sym) = field_symbol {
                                if let Some(blob_lock) = &relation_symbol.blob {
                                    if let Ok(mut blob_data) = blob_lock.lock() {
                                        if let Some(relation_data) = blob_data.downcast_mut::<crate::textplan::common::structured_symbol_data::RelationData>() {
                                            relation_data.output_field_references.push(field_sym);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        self.visit_children(ctx);

        // Reset processing_emit flag
        self.processing_emit = false;
    }

    fn visit_relationCount(&mut self, ctx: &RelationCountContext<'input>) {
        // Handle COUNT NUMBER SEMICOLON for Fetch relations
        if let Some(relation_symbol) = self.current_relation_scope().cloned() {
            if let Some(number_ctx) = ctx.NUMBER() {
                let count_text = number_ctx.get_text();
                if let Ok(count_value) = count_text.parse::<i64>() {
                    // Update the FetchRel with count
                    if let Some(blob_lock) = &relation_symbol.blob {
                        if let Ok(mut blob_data) = blob_lock.lock() {
                            if let Some(relation_data) = blob_data.downcast_mut::<crate::textplan::common::structured_symbol_data::RelationData>() {
                                use ::substrait::proto::rel::RelType;
                                if let Some(RelType::Fetch(ref mut fetch_rel)) = &mut relation_data.relation.rel_type {
                                    use ::substrait::proto::fetch_rel::CountMode;
                                    fetch_rel.count_mode = Some(CountMode::Count(count_value));
                                }
                            }
                        }
                    }
                }
            }
        }
        self.visit_children(ctx);
    }

    fn visit_relationOffset(&mut self, ctx: &RelationOffsetContext<'input>) {
        // Handle OFFSET NUMBER SEMICOLON for Fetch relations
        if let Some(relation_symbol) = self.current_relation_scope().cloned() {
            if let Some(number_ctx) = ctx.NUMBER() {
                let offset_text = number_ctx.get_text();
                if let Ok(offset_value) = offset_text.parse::<i64>() {
                    // Update the FetchRel with offset
                    if let Some(blob_lock) = &relation_symbol.blob {
                        if let Ok(mut blob_data) = blob_lock.lock() {
                            if let Some(relation_data) = blob_data.downcast_mut::<crate::textplan::common::structured_symbol_data::RelationData>() {
                                use ::substrait::proto::rel::RelType;
                                if let Some(RelType::Fetch(ref mut fetch_rel)) = &mut relation_data.relation.rel_type {
                                    use ::substrait::proto::fetch_rel::OffsetMode;
                                    fetch_rel.offset_mode = Some(OffsetMode::Offset(offset_value));
                                }
                            }
                        }
                    }
                }
            }
        }
        self.visit_children(ctx);
    }

    // We use the default implementation for other visitor methods,
    // which will call visit_children to traverse the tree
}

/// The SubqueryRelationVisitor processes subquery relations.
///
/// This visitor is the fifth and final phase in the multiphase parsing approach,
/// specifically handling subquery relationships.
pub struct SubqueryRelationVisitor<'input> {
    symbol_table: SymbolTable,
    error_listener: Arc<ErrorListener>,
    current_relation_scope: Option<Arc<SymbolInfo>>,
    // Stores (field_index, steps_out) for each column expression in visit order
    expression_field_info: std::cell::RefCell<Vec<(usize, usize)>>,
    // Tracks the next index to consume from expression_field_info
    expression_field_info_index: std::cell::RefCell<usize>,
    _phantom: std::marker::PhantomData<&'input ()>,
}

impl<'input> SubqueryRelationVisitor<'input> {
    /// Creates a new SubqueryRelationVisitor.
    pub fn new(mut symbol_table: SymbolTable, error_listener: Arc<ErrorListener>) -> Self {
        // Populate sub_query_pipelines now that RelationVisitor has created all the blobs
        Self::populate_subquery_pipelines(&mut symbol_table);

        Self {
            symbol_table,
            error_listener,
            current_relation_scope: None,
            expression_field_info: std::cell::RefCell::new(Vec::new()),
            expression_field_info_index: std::cell::RefCell::new(0),
            _phantom: std::marker::PhantomData,
        }
    }

    /// Populates sub_query_pipelines for all relations by finding subquery relations
    /// and adding them to their parent relations.
    fn populate_subquery_pipelines(symbol_table: &mut SymbolTable) {
        println!("DEBUG SUBQUERY: Populating sub_query_pipelines in SubqueryRelationVisitor");

        // Find all subquery relations (those with parent_query_index >= 0)
        let mut subqueries: Vec<Arc<SymbolInfo>> = Vec::new();
        for symbol in symbol_table.symbols() {
            if symbol.symbol_type() != SymbolType::Relation {
                continue;
            }

            // Check if this relation or its pipeline_start has parent_query_index set
            let is_subquery = if symbol.parent_query_index() >= 0 {
                true
            } else if let Some(blob_lock) = &symbol.blob {
                if let Ok(blob_data) = blob_lock.lock() {
                    if let Some(relation_data) = blob_data.downcast_ref::<RelationData>() {
                        if let Some(pipeline_start) = &relation_data.pipeline_start {
                            pipeline_start.parent_query_index() >= 0
                        } else {
                            false
                        }
                    } else {
                        false
                    }
                } else {
                    false
                }
            } else {
                false
            };

            if is_subquery {
                // Get the pipeline_start (terminus) to add to parent
                let terminus = if let Some(blob_lock) = &symbol.blob {
                    if let Ok(blob_data) = blob_lock.lock() {
                        if let Some(relation_data) = blob_data.downcast_ref::<RelationData>() {
                            relation_data
                                .pipeline_start
                                .clone()
                                .unwrap_or_else(|| symbol.clone())
                        } else {
                            symbol.clone()
                        }
                    } else {
                        symbol.clone()
                    }
                } else {
                    symbol.clone()
                };

                println!(
                    "DEBUG SUBQUERY: Found subquery terminus: '{}'",
                    terminus.name()
                );
                subqueries.push(terminus);
            }
        }

        // Now find parent relations by checking which relation expressions reference these subqueries
        // For now, we'll add all subqueries to relations that might use them
        // A more precise implementation would parse the expressions to find actual references
        for parent_symbol in symbol_table.symbols() {
            if parent_symbol.symbol_type() != SymbolType::Relation {
                continue;
            }

            // Add subqueries to this parent if they reference it
            // For simplicity, we add all subqueries for now - the lookup will filter correctly
            for subquery in &subqueries {
                // Skip if this parent IS the subquery
                if Arc::ptr_eq(parent_symbol, subquery) {
                    continue;
                }

                // Add to parent's sub_query_pipelines
                if let Some(blob_lock) = &parent_symbol.blob {
                    if let Ok(mut blob_data) = blob_lock.lock() {
                        if let Some(relation_data) = blob_data.downcast_mut::<RelationData>() {
                            // Check if not already added
                            let already_added = relation_data
                                .sub_query_pipelines
                                .iter()
                                .any(|sq| Arc::ptr_eq(sq, subquery));

                            if !already_added {
                                println!(
                                    "DEBUG SUBQUERY: Adding '{}' to '{}'",
                                    subquery.name(),
                                    parent_symbol.name()
                                );
                                relation_data.sub_query_pipelines.push(subquery.clone());
                            }
                        }
                    }
                }
            }
        }
    }

    /// Gets the symbol table.
    pub fn symbol_table(&self) -> SymbolTable {
        self.symbol_table.clone()
    }

    /// Gets a mutable reference to the symbol table.
    fn symbol_table_mut(&mut self) -> &mut SymbolTable {
        &mut self.symbol_table
    }

    /// Gets the error listener.
    pub fn error_listener(&self) -> Arc<ErrorListener> {
        self.error_listener.clone()
    }

    /// Adds an error message to the error listener.
    pub fn add_error<'a>(
        &self,
        token: &impl std::ops::Deref<Target = GenericToken<std::borrow::Cow<'a, str>>>,
        message: &str,
    ) {
        let location = token_to_location(token);
        self.error_listener.add_error(message.to_string(), location);
    }

    /// Gets the current relation scope, if any.
    pub fn current_relation_scope(&self) -> Option<&Arc<SymbolInfo>> {
        self.current_relation_scope.as_ref()
    }

    /// Sets the current relation scope.
    pub fn set_current_relation_scope(&mut self, scope: Option<Arc<SymbolInfo>>) {
        self.current_relation_scope = scope;
    }

    /// Process a scalar subquery and add it to the symbol table.
    fn process_scalar_subquery(
        &mut self,
        ctx: &ExpressionScalarSubqueryContext<'input>,
    ) -> Option<Arc<SymbolInfo>> {
        // Create a location from the context's start token
        // Get the start token directly - it's not an Option
        let token = ctx.start();
        let location = token_to_location(&token);

        // Define the subquery in the symbol table
        let symbol = self.symbol_table_mut().define_symbol(
            "scalar_subquery".to_string(),
            location,
            SymbolType::Relation,
            None,
            None,
        );

        Some(symbol)
    }

    /// Process a set comparison subquery and add it to the symbol table.
    fn process_set_comparison_subquery(
        &mut self,
        ctx: &ExpressionSetComparisonSubqueryContext<'input>,
    ) -> Option<Arc<SymbolInfo>> {
        // Create a location from the context's start token
        // Get the start token directly - it's not an Option
        let token = ctx.start();
        let location = token_to_location(&token);

        // Define the subquery in the symbol table
        let symbol = self.symbol_table_mut().define_symbol(
            "set_comparison_subquery".to_string(),
            location,
            SymbolType::Relation,
            None,
            None,
        );

        Some(symbol)
    }

    /// Process an IN predicate subquery and add it to the symbol table.
    fn process_in_predicate_subquery(
        &mut self,
        ctx: &ExpressionInPredicateSubqueryContext<'input>,
    ) -> Option<Arc<SymbolInfo>> {
        // Create a location from the context's start token
        // Get the start token directly - it's not an Option
        let token = ctx.start();
        let location = token_to_location(&token);

        // Define the subquery in the symbol table
        let symbol = self.symbol_table_mut().define_symbol(
            "in_predicate_subquery".to_string(),
            location,
            SymbolType::Relation,
            None,
            None,
        );

        Some(symbol)
    }

    /// Process a set predicate subquery and add it to the symbol table.
    fn process_set_predicate_subquery(
        &mut self,
        ctx: &ExpressionSetPredicateSubqueryContext<'input>,
    ) -> Option<Arc<SymbolInfo>> {
        // Create a location from the context's start token
        // Get the start token directly - it's not an Option
        let token = ctx.start();
        let location = token_to_location(&token);

        // Define the subquery in the symbol table
        let symbol = self.symbol_table_mut().define_symbol(
            "set_predicate_subquery".to_string(),
            location,
            SymbolType::Relation,
            None,
            None,
        );

        Some(symbol)
    }

    /// Gets the parent query relation symbol by searching the symbol table.
    /// Returns the parent relation that has this relation in its sub_query_pipelines.
    /// This uses Rust's Arc-based approach rather than C++'s location pointers.
    fn get_parent_query_relation(&self, symbol: &Arc<SymbolInfo>) -> Option<Arc<SymbolInfo>> {
        println!("        get_parent_query_location for '{}'", symbol.name());

        // Check if THIS relation itself has parent_query_index set (meaning it's IN a subquery)
        // Relations that are part of a subquery will have parent_query_index set either:
        // 1. Directly when marked as a subquery root, OR
        // 2. By set_pipeline_start_for_subquery() when walking the continuing_pipeline chain
        if symbol.parent_query_index() < 0 {
            println!("          Not a subquery (parent_query_index < 0)");
            return None;
        }

        println!(
            "          Has parent_query_index: {}",
            symbol.parent_query_index()
        );

        // Otherwise, search the symbol table to find which relation has this subquery's pipeline_start
        // We need to search for the pipeline_start, not the current relation
        let search_symbol = if let Some(blob_lock) = &symbol.blob {
            if let Ok(blob_data) = blob_lock.lock() {
                if let Some(relation_data) = blob_data.downcast_ref::<RelationData>() {
                    relation_data.pipeline_start.clone()
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
        };

        let search_target = search_symbol.as_ref().unwrap_or(symbol);
        println!(
            "          Searching for parent of pipeline '{}' (terminus of subquery)",
            search_target.name()
        );

        // Find all potential parents that have this subquery
        let mut potential_parents: Vec<Arc<SymbolInfo>> = Vec::new();
        for potential_parent in self.symbol_table.symbols() {
            if potential_parent.symbol_type() != SymbolType::Relation {
                continue;
            }

            if let Some(blob_lock) = &potential_parent.blob {
                if let Ok(blob_data) = blob_lock.lock() {
                    if let Some(relation_data) = blob_data.downcast_ref::<RelationData>() {
                        // Check if pipeline_start is in the potential parent's sub_query_pipelines
                        for subquery in &relation_data.sub_query_pipelines {
                            if Arc::ptr_eq(subquery, search_target)
                                || subquery.name() == search_target.name()
                            {
                                potential_parents.push(potential_parent.clone());
                                break;
                            }
                        }
                    }
                }
            }
        }

        if potential_parents.is_empty() {
            println!("          No parent query found in symbol table");
            return None;
        }

        println!(
            "          Found {} potential parent(s)",
            potential_parents.len()
        );
        // Return the first one (we'll refine this later to find the actual correct parent)
        // For now, this works because we populated sub_query_pipelines to all relations
        Some(potential_parents[0].clone())
    }

    /// Finds a field reference by name, recursively searching parent relations.
    /// Returns (steps_out, field_index) where steps_out is the number of parent
    /// query levels to traverse, and field_index is the index of the field.
    /// This follows the C++ findFieldReferenceByName pattern.
    fn find_field_reference_by_name(
        &self,
        column_name: &str,
        symbol: &Arc<SymbolInfo>,
    ) -> (usize, Option<usize>) {
        self.find_field_reference_by_name_impl(column_name, symbol, 0)
    }

    fn find_field_reference_by_name_impl(
        &self,
        column_name: &str,
        symbol: &Arc<SymbolInfo>,
        depth: usize,
    ) -> (usize, Option<usize>) {
        // Prevent infinite recursion - limit depth to 10 levels
        if depth > 10 {
            println!(
                "      WARNING: Depth limit reached searching for '{}'",
                column_name
            );
            return (0, None);
        }

        println!(
            "      SubqueryRelationVisitor::find_field_reference_by_name: '{}' in relation '{}' (depth {})",
            column_name,
            symbol.name(),
            depth
        );

        // Search for the field in this relation using lookup_field_index_in_relation
        let field_index = self.lookup_field_index_in_relation(column_name, symbol);

        if field_index.is_some() {
            println!(
                "      Found '{}' in current relation at index {:?}",
                column_name, field_index
            );
            return (0, field_index);
        }

        // Field not found in current relation - check parent query
        println!(
            "      Field '{}' not in current relation, searching parent",
            column_name
        );

        // Get the parent query location (following C++ getParentQueryLocation pattern)
        let parent_location = if !symbol.parent_query_location().is_unknown() {
            Some(symbol.parent_query_location())
        } else {
            // Try pipelineStart
            if let Some(blob_lock) = &symbol.blob {
                if let Ok(blob_data) = blob_lock.lock() {
                    if let Some(relation_data) = blob_data.downcast_ref::<RelationData>() {
                        if let Some(pipeline_start) = &relation_data.pipeline_start {
                            if !pipeline_start.parent_query_location().is_unknown() {
                                Some(pipeline_start.parent_query_location())
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                } else {
                    None
                }
            } else {
                None
            }
        };

        // Look up the specific parent symbol by location
        if let Some(parent_loc) = parent_location {
            if let Some(parent_symbol) = self
                .symbol_table
                .lookup_symbol_by_location_and_type(parent_loc.as_ref(), SymbolType::Relation)
            {
                println!("      Found parent relation '{}'", parent_symbol.name());

                // Recursively search in parent (don't hold any locks during recursive call)
                let (parent_steps_out, field_index) =
                    self.find_field_reference_by_name(column_name, &parent_symbol);

                if field_index.is_some() {
                    println!(
                        "      Found '{}' in parent relation '{}', steps_out = {}",
                        column_name,
                        parent_symbol.name(),
                        parent_steps_out + 1
                    );
                    return (parent_steps_out + 1, field_index);
                }
            }
        }

        // Field not found in parent
        (0, None)
    }

    /// Looks up a field index in a specific relation's schema (does NOT walk parent chain).
    fn lookup_field_index_in_relation(
        &self,
        column_name: &str,
        relation_symbol: &Arc<SymbolInfo>,
    ) -> Option<usize> {
        // Parse column name - can be "field" or "schema.field"
        let (_schema_name, field_name) = if let Some(dot_pos) = column_name.rfind('.') {
            (&column_name[..dot_pos], &column_name[dot_pos + 1..])
        } else {
            // No schema prefix
            ("", column_name)
        };

        // Search ONLY in this relation's field_references
        if let Some(blob_lock) = &relation_symbol.blob {
            if let Ok(blob_data) = blob_lock.lock() {
                if let Some(relation_data) = blob_data.downcast_ref::<RelationData>() {
                    // Search through field_references in this relation only
                    // Try matching against full column_name first, then just field_name
                    for (index, field_sym) in relation_data.field_references.iter().enumerate() {
                        if field_sym.name() == column_name || field_sym.name() == field_name {
                            return Some(index);
                        }
                        // Also try matching with schema prefix
                        if let Some(schema) = field_sym.schema() {
                            let qualified_name = format!("{}.{}", schema.name(), field_sym.name());
                            if qualified_name == column_name {
                                return Some(index);
                            }
                        }
                    }

                    // Also check generated_field_references
                    let field_ref_size = relation_data.field_references.len();
                    for (index, field_sym) in
                        relation_data.generated_field_references.iter().enumerate()
                    {
                        if field_sym.name() == column_name || field_sym.name() == field_name {
                            return Some(field_ref_size + index);
                        }
                        if let Some(schema) = field_sym.schema() {
                            let qualified_name = format!("{}.{}", schema.name(), field_sym.name());
                            if qualified_name == column_name {
                                return Some(field_ref_size + index);
                            }
                        }
                    }
                }
            }
        }
        None
    }

    /// Fixes outer references in a relation's expressions.
    /// This traverses the relation proto and updates field references to use outer_reference
    /// when appropriate.
    fn fix_outer_references_in_relation(&self, relation_symbol: &Arc<SymbolInfo>) {
        println!(
            "      Fixing outer references in relation '{}'",
            relation_symbol.name()
        );

        // Only fix outer references if this relation is in a subquery
        if let Some(_parent_symbol) = self.get_parent_query_relation(relation_symbol) {
            println!("        This relation is in a subquery, checking expressions");

            if let Some(blob_lock) = &relation_symbol.blob {
                // First: extract the relation proto (clone it out)
                let mut relation_proto = if let Ok(blob_data) = blob_lock.lock() {
                    if let Some(relation_data) = blob_data.downcast_ref::<RelationData>() {
                        Some(relation_data.relation.clone())
                    } else {
                        None
                    }
                } else {
                    None
                };
                // Lock is dropped here

                // Second: fix expressions in the extracted proto
                // Now fix_expression_outer_references can lock relation_symbol as needed
                if let Some(mut relation) = relation_proto {
                    use substrait::proto::rel::RelType;
                    if let Some(rel_type) = &mut relation.rel_type {
                        match rel_type {
                            RelType::Filter(filter_rel) => {
                                if let Some(condition) = &mut filter_rel.condition {
                                    self.fix_expression_outer_references(
                                        condition,
                                        relation_symbol,
                                    );
                                }
                            }
                            RelType::Project(project_rel) => {
                                for expr in &mut project_rel.expressions {
                                    self.fix_expression_outer_references(expr, relation_symbol);
                                }
                            }
                            RelType::Aggregate(agg_rel) => {
                                // Fix grouping expressions
                                for grouping in &mut agg_rel.groupings {
                                    for expr in &mut grouping.grouping_expressions {
                                        self.fix_expression_outer_references(expr, relation_symbol);
                                    }
                                }
                                // Fix measure expressions
                                for measure in &mut agg_rel.measures {
                                    if let Some(measure_func) = &mut measure.measure {
                                        for arg in &mut measure_func.arguments {
                                            if let Some(
                                                substrait::proto::function_argument::ArgType::Value(
                                                    expr,
                                                ),
                                            ) = &mut arg.arg_type
                                            {
                                                self.fix_expression_outer_references(
                                                    expr,
                                                    relation_symbol,
                                                );
                                            }
                                        }
                                    }
                                }
                            }
                            _ => {
                                // Other relation types - we can add support as needed
                            }
                        }
                    }

                    // Third: put the fixed proto back
                    if let Ok(mut blob_data) = blob_lock.lock() {
                        if let Some(relation_data) = blob_data.downcast_mut::<RelationData>() {
                            relation_data.relation = relation;
                        }
                    }
                }
            }
        } else {
            println!("        This relation is NOT in a subquery, skipping");
        }
    }

    /// Recursively fixes outer references in an expression and its sub-expressions.
    fn fix_expression_outer_references(
        &self,
        expr: &mut substrait::proto::Expression,
        relation_symbol: &Arc<SymbolInfo>,
    ) {
        use substrait::proto::expression::RexType;

        match &mut expr.rex_type {
            Some(RexType::Selection(field_ref)) => {
                // This is a field reference - check if it needs to be an outer reference
                if let Some(
                    substrait::proto::expression::field_reference::ReferenceType::DirectReference(
                        ref_seg,
                    ),
                ) = &mut field_ref.reference_type
                {
                    if let Some(
                        substrait::proto::expression::reference_segment::ReferenceType::StructField(
                            struct_field,
                        ),
                    ) = &mut ref_seg.reference_type
                    {
                        // Consume the next stored field info from visit pass
                        let mut current_index = self.expression_field_info_index.borrow_mut();
                        let field_info = self.expression_field_info.borrow();

                        if *current_index < field_info.len() {
                            let (correct_field_index, steps_out) = field_info[*current_index];
                            *current_index += 1;

                            println!(
                                "          Using stored field info: field_index={}, steps_out={}",
                                correct_field_index, steps_out
                            );

                            // Update the field index to the correct value
                            struct_field.field = correct_field_index as i32;

                            // Set outer_reference if steps_out > 0, otherwise root_reference
                            if steps_out > 0 {
                                println!(
                                    "            ✓✓✓ Converting field {} to OUTER REFERENCE (steps_out={})",
                                    correct_field_index, steps_out
                                );
                                field_ref.root_type = Some(
                                    substrait::proto::expression::field_reference::RootType::OuterReference(
                                        substrait::proto::expression::field_reference::OuterReference {
                                            steps_out: steps_out as u32,
                                        },
                                    ),
                                );
                            } else {
                                println!(
                                    "            Field {} is local (steps_out=0)",
                                    correct_field_index
                                );
                                // Ensure it has root_reference
                                if field_ref.root_type.is_none() {
                                    field_ref.root_type = Some(
                                        substrait::proto::expression::field_reference::RootType::RootReference(
                                            substrait::proto::expression::field_reference::RootReference {},
                                        ),
                                    );
                                }
                            }
                        } else {
                            println!(
                                "          WARNING: No stored field info for this expression (index out of bounds)"
                            );
                        }
                    }
                }
            }
            Some(RexType::ScalarFunction(func)) => {
                // Recursively fix arguments
                for arg in &mut func.arguments {
                    if let Some(substrait::proto::function_argument::ArgType::Value(inner_expr)) =
                        &mut arg.arg_type
                    {
                        self.fix_expression_outer_references(inner_expr, relation_symbol);
                    }
                }
            }
            _ => {
                // Other expression types - we can add support as needed
            }
        }
    }
}

impl<'input> PlanVisitor<'input> for SubqueryRelationVisitor<'input> {
    fn error_listener(&self) -> Arc<ErrorListener> {
        self.error_listener.clone()
    }

    fn symbol_table(&self) -> SymbolTable {
        self.symbol_table.clone()
    }
}

// ANTLR visitor implementation for SubqueryRelationVisitor
impl<'input> ParseTreeVisitor<'input, SubstraitPlanParserContextType>
    for SubqueryRelationVisitor<'input>
{
}

impl<'input> SubstraitPlanParserVisitor<'input> for SubqueryRelationVisitor<'input> {
    // Override specific visitor methods for subquery processing and expression reprocessing

    fn visit_expressionColumn(&mut self, ctx: &ExpressionColumnContext<'input>) {
        // Re-read column name from parse tree and determine correct field index + steps_out
        // This follows C++ SubqueryRelationVisitor::visitExpressionColumn

        let column_name = ctx.get_text();
        println!(
            "SubqueryRelationVisitor::visit_expressionColumn: '{}'",
            column_name
        );

        // We can only process this if we have a current relation scope
        if let Some(current_rel) = self.current_relation_scope().cloned() {
            // Use find_field_reference_by_name to recursively search for the field
            let (steps_out, field_index) =
                self.find_field_reference_by_name(&column_name, &current_rel);

            if let Some(index) = field_index {
                // ONLY store field info for outer references (steps_out > 0)
                // Local references (steps_out=0) don't need fixing
                if steps_out > 0 {
                    self.expression_field_info
                        .borrow_mut()
                        .push((index, steps_out));
                    println!(
                        "      ✓ Stored '{}' as outer reference: field_index={}, steps_out={}",
                        column_name, index, steps_out
                    );
                } else {
                    println!(
                        "      Skipped '{}' (local reference, field_index={})",
                        column_name, index
                    );
                }
            } else {
                println!(
                    "      WARNING: Field '{}' not found in current or parent relations",
                    column_name
                );
            }
        } else {
            println!("      WARNING: No current relation scope");
        }

        // Continue with default visitor behavior
        self.visit_children(ctx);
    }

    fn visit_relation(&mut self, ctx: &RelationContext<'input>) {
        // Visit all relations and set the current scope for expression processing.
        // This follows the C++ SubqueryRelationVisitor::visitRelation pattern.

        println!(
            "SubqueryRelationVisitor::visit_relation: {}",
            ctx.get_text()
        );

        // Look up the relation symbol (should have been created by previous visitors)
        let token = ctx.start();
        let location = token_to_location(&token);

        if let Some(relation_symbol) = self
            .symbol_table
            .lookup_symbol_by_location_and_type(&location, SymbolType::Relation)
        {
            println!(
                "      Found relation symbol '{}' (location hash: {}), setting as current scope",
                relation_symbol.name(),
                relation_symbol.source_location().location_hash()
            );

            // Save the previous scope
            let old_scope = self.current_relation_scope().cloned();

            // Set this relation as the current scope
            self.set_current_relation_scope(Some(relation_symbol.clone()));

            // Visit children with this scope set
            self.visit_children(ctx);

            // Fix outer references in this relation's expressions
            // This updates field references to use outerReference instead of rootReference
            // when they refer to fields in the parent query
            self.fix_outer_references_in_relation(&relation_symbol);

            // Restore the previous scope
            self.set_current_relation_scope(old_scope);
        } else {
            println!(
                "      WARNING: No relation symbol found at location, visiting children anyway"
            );
            // Visit children even if we couldn't find the symbol
            self.visit_children(ctx);
        }
    }

    fn visit_expressionScalarSubquery(&mut self, ctx: &ExpressionScalarSubqueryContext<'input>) {
        // Process a scalar subquery expression
        println!(
            "SubqueryRelationVisitor processing scalar subquery: {}",
            ctx.get_text()
        );

        // Process the subquery and add it to the symbol table
        if let Some(subquery_symbol) = self.process_scalar_subquery(ctx) {
            // Save the current relation scope
            let old_scope = self.current_relation_scope().cloned();

            // Set the subquery relation as the current scope
            self.set_current_relation_scope(Some(subquery_symbol));

            // Visit children to process subquery details
            self.visit_children(ctx);

            // Restore the old scope
            self.set_current_relation_scope(old_scope);
        } else {
            // Just visit children
            self.visit_children(ctx);
        }
    }

    fn visit_expressionSetComparisonSubquery(
        &mut self,
        ctx: &ExpressionSetComparisonSubqueryContext<'input>,
    ) {
        // Process a set comparison subquery expression
        println!(
            "SubqueryRelationVisitor processing set comparison subquery: {}",
            ctx.get_text()
        );

        // Process the subquery and add it to the symbol table
        if let Some(subquery_symbol) = self.process_set_comparison_subquery(ctx) {
            // Save the current relation scope
            let old_scope = self.current_relation_scope().cloned();

            // Set the subquery relation as the current scope
            self.set_current_relation_scope(Some(subquery_symbol));

            // Visit children to process subquery details
            self.visit_children(ctx);

            // Restore the old scope
            self.set_current_relation_scope(old_scope);
        } else {
            // Just visit children
            self.visit_children(ctx);
        }
    }

    fn visit_expressionInPredicateSubquery(
        &mut self,
        ctx: &ExpressionInPredicateSubqueryContext<'input>,
    ) {
        // Process an IN predicate subquery expression
        println!(
            "SubqueryRelationVisitor processing IN predicate subquery: {}",
            ctx.get_text()
        );

        // Process the subquery and add it to the symbol table
        if let Some(subquery_symbol) = self.process_in_predicate_subquery(ctx) {
            // Save the current relation scope
            let old_scope = self.current_relation_scope().cloned();

            // Set the subquery relation as the current scope
            self.set_current_relation_scope(Some(subquery_symbol));

            // Visit children to process subquery details
            self.visit_children(ctx);

            // Restore the old scope
            self.set_current_relation_scope(old_scope);
        } else {
            // Just visit children
            self.visit_children(ctx);
        }
    }

    fn visit_expressionSetPredicateSubquery(
        &mut self,
        ctx: &ExpressionSetPredicateSubqueryContext<'input>,
    ) {
        // Process a set predicate subquery expression
        println!(
            "SubqueryRelationVisitor processing set predicate subquery: {}",
            ctx.get_text()
        );

        // Process the subquery and add it to the symbol table
        if let Some(subquery_symbol) = self.process_set_predicate_subquery(ctx) {
            // Save the current relation scope
            let old_scope = self.current_relation_scope().cloned();

            // Set the subquery relation as the current scope
            self.set_current_relation_scope(Some(subquery_symbol));

            // Visit children to process subquery details
            self.visit_children(ctx);

            // Restore the old scope
            self.set_current_relation_scope(old_scope);
        } else {
            // Just visit children
            self.visit_children(ctx);
        }
    }

    // We use the default implementation for other visitor methods,
    // which will call visit_children to traverse the tree
}
