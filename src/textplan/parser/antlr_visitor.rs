// SPDX-License-Identifier: Apache-2.0

//! ANTLR visitor implementation for Substrait textplans.
//!
//! This module contains visitor implementations that process
//! the ANTLR parse tree and build a symbol table following the
//! multiphase approach used in the C++ implementation.

use std::sync::Arc;

use antlr_rust::parser_rule_context::ParserRuleContext;
use antlr_rust::rule_context::RuleContext;
use antlr_rust::TidExt;
use antlr_rust::token::{GenericToken, Token};
use antlr_rust::tree::{ParseTree, ParseTreeVisitor};

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
    // This is a simplified calculation - we're using the token's start column as position
    // and the token's text length as the length
    let position = token.column as i32;
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
            "interval_year" | "interval_year_to_month" => {
                let mut interval_year_type = IntervalYear::default();
                interval_year_type.nullability = nullability.into();
                proto_type.kind = Some(Kind::IntervalYear(interval_year_type));
            }
            "interval_day" | "interval_day_to_second" => {
                let mut interval_day_type = IntervalDay::default();
                interval_day_type.nullability = nullability.into();
                proto_type.kind = Some(Kind::IntervalDay(interval_day_type));
            }
            "uuid" => {
                let mut uuid_type = Uuid::default();
                uuid_type.nullability = nullability.into();
                proto_type.kind = Some(Kind::Uuid(uuid_type));
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
    num_spaces_seen: i32,
    num_functions_seen: i32,
}

impl<'input> MainPlanVisitor<'input> {
    /// Creates a new MainPlanVisitor.
    pub fn new(symbol_table: SymbolTable, error_listener: Arc<ErrorListener>) -> Self {
        Self {
            type_visitor: TypeVisitor::new(symbol_table, error_listener.clone()),
            current_relation_scope: None,
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

        // Define the extension space in the symbol table
        // Symbol table accessed directly via base
        let symbol = self.type_visitor.base.symbol_table_mut().define_symbol(
            name,
            location,
            SymbolType::ExtensionSpace,
            None,
            None,
        );

        Some(symbol)
    }

    /// Process a function definition and add it to the symbol table.
    fn process_function(&mut self, ctx: &FunctionContext<'input>) -> Option<Arc<SymbolInfo>> {
        // Get the name of the function using the function context trait
        let name = ctx.id()?.get_text();

        // Create a location from the context's start token
        // Get the start token directly - it's not an Option
        let token = ctx.start();
        let location = token_to_location(&token);

        // Define the function in the symbol table
        // Symbol table accessed directly via base
        let symbol = self.type_visitor.base.symbol_table_mut().define_symbol(
            name,
            location,
            SymbolType::Function,
            None,
            None,
        );

        // Note: Signature handling is omitted for now as it needs appropriate context access
        // This would be implemented in a future enhancement

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
            self.type_visitor.base.symbol_table_mut().define_symbol(
                name,
                location,
                SymbolType::SourceDetail,
                None,
                None,
            );
        }

        Some(())
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
                        rel_type: Some(RelType::Join(Box::new(
                            ::substrait::proto::JoinRel::default(),
                        ))),
                    },
                ),
                "cross" => (
                    RelationType::Cross,
                    Rel {
                        rel_type: Some(RelType::Cross(Box::new(
                            ::substrait::proto::CrossRel::default(),
                        ))),
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
                "aggregate" => (
                    RelationType::Aggregate,
                    Rel {
                        rel_type: Some(RelType::Aggregate(Box::new(
                            ::substrait::proto::AggregateRel::default(),
                        ))),
                    },
                ),
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
                        rel_type: Some(RelType::HashJoin(Box::new(
                            ::substrait::proto::HashJoinRel::default(),
                        ))),
                    },
                ),
                "merge_join" => (
                    RelationType::MergeJoin,
                    Rel {
                        rel_type: Some(RelType::MergeJoin(Box::new(
                            ::substrait::proto::MergeJoinRel::default(),
                        ))),
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
        let relation_data = RelationData::new_empty();
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
        // Create a location from the context's start token
        // Get the start token directly - it's not an Option
        let token = ctx.start();
        let location = token_to_location(&token);

        // Define the schema in the symbol table
        // Symbol table accessed directly via base
        let symbol = self.type_visitor.base.symbol_table_mut().define_symbol(
            "schema".to_string(),
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

        // Create a location from the context's start token
        // Get the start token directly - it's not an Option
        let token = ctx.start();
        let location = token_to_location(&token);

        // Define the column in the symbol table
        // Symbol table accessed directly via base
        let symbol = self.type_visitor.base.symbol_table_mut().define_symbol(
            name,
            location,
            SymbolType::SchemaColumn,
            None,
            None,
        );

        // Set the schema as the parent
        // Set the schema as the parent - no mutable reference needed due to interior mutability
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
        self.process_relation(ctx);

        // Visit children to process relation details
        self.visit_children(ctx);

        // Clear the current relation scope when done with this relation
        self.set_current_relation_scope(None);
    }

    fn visit_root_relation(&mut self, ctx: &Root_relationContext<'input>) {
        // Process a root relation definition
        println!("Visiting root relation: {}", ctx.get_text());

        // Process the root relation and add it to the symbol table
        self.process_root_relation(ctx);

        // Visit children to process relation details
        self.visit_children(ctx);

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
        self.process_extension_space(ctx);

        // Visit children to process extension details
        self.visit_children(ctx);
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
        self.process_source_definition(ctx);

        // Visit children to process source details
        self.visit_children(ctx);
    }

    fn visit_named_table_detail(&mut self, ctx: &Named_table_detailContext<'input>) {
        // Process named table detail (strings in the source)
        println!("Visiting named table detail: {}", ctx.get_text());

        // Process the strings and add them to the symbol table
        self.process_named_table_detail(ctx);

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
                    if let Some(parent_pipeline) = parent_ctx
                        .downcast_ref::<PipelineContext>()
                    {
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
                        left_rel_data
                            .pipeline_start
                            .clone()
                            .or(Some(left.clone()))
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
            if right_symbol.is_none() {
                // No right symbol means we're starting a new branch
                println!(
                    "  Pipeline: {} starts new branch with left: {}",
                    relation_name,
                    left.name()
                );
                relation_data.new_pipelines.push(left);
            } else {
                // Right symbol exists, so we're continuing a pipeline
                println!(
                    "  Pipeline: {} continues with left: {}, right: {}",
                    relation_name,
                    left.name(),
                    right_symbol.as_ref().unwrap().name()
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
    _phantom: std::marker::PhantomData<&'input ()>,
}

impl<'input> RelationVisitor<'input> {
    /// Creates a new RelationVisitor.
    pub fn new(symbol_table: SymbolTable, error_listener: Arc<ErrorListener>) -> Self {
        Self {
            symbol_table,
            error_listener,
            current_relation_scope: None,
            _phantom: std::marker::PhantomData,
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
        let symbol = self.symbol_table_mut().define_symbol(value.to_string(), location, SymbolType::Field, None, None);

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
        let symbol = self.symbol_table_mut().define_symbol(name.to_string(), location, SymbolType::Field, None, None);

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
        if let Some(relation_symbol) = symbol {
            self.set_current_relation_scope(Some(relation_symbol.clone()));
        }

        // Continue visiting children to process relation details
        self.visit_children(ctx);

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
        // Process the filter relation
        if let Some(filter_symbol) = self.process_filter_relation(ctx) {
            // Save the current relation scope
            let old_scope = self.current_relation_scope().cloned();

            // Set the filter relation as the current scope
            self.set_current_relation_scope(Some(filter_symbol));

            // Visit children
            self.visit_children(ctx);

            // Restore the old scope
            self.set_current_relation_scope(old_scope);
        } else {
            // Just visit children
            self.visit_children(ctx);
        }
    }

    fn visit_relationExpression(&mut self, ctx: &RelationExpressionContext<'input>) {
        // Process the expression relation
        if let Some(expr_symbol) = self.process_expression_relation(ctx) {
            // Save the current relation scope
            let old_scope = self.current_relation_scope().cloned();

            // Set the expression relation as the current scope
            self.set_current_relation_scope(Some(expr_symbol));

            // Visit children
            self.visit_children(ctx);

            // Restore the old scope
            self.set_current_relation_scope(old_scope);
        } else {
            // Just visit children
            self.visit_children(ctx);
        }
    }

    fn visit_relationJoinType(&mut self, ctx: &RelationJoinTypeContext<'input>) {
        // Process the join relation
        if let Some(join_symbol) = self.process_join_relation(ctx) {
            // Save the current relation scope
            let old_scope = self.current_relation_scope().cloned();

            // Set the join relation as the current scope
            self.set_current_relation_scope(Some(join_symbol));

            // Visit children
            self.visit_children(ctx);

            // Restore the old scope
            self.set_current_relation_scope(old_scope);
        } else {
            // Just visit children
            self.visit_children(ctx);
        }
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
                    if let Some(source_symbol) = self.symbol_table.lookup_symbol_by_name(&source_name) {
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
    _phantom: std::marker::PhantomData<&'input ()>,
}

impl<'input> SubqueryRelationVisitor<'input> {
    /// Creates a new SubqueryRelationVisitor.
    pub fn new(symbol_table: SymbolTable, error_listener: Arc<ErrorListener>) -> Self {
        Self {
            symbol_table,
            error_listener,
            current_relation_scope: None,
            _phantom: std::marker::PhantomData,
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
    // Override specific visitor methods for subquery processing

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
