// SPDX-License-Identifier: Apache-2.0

//! Expression printer for converting Substrait proto expressions to text format.

use std::sync::Arc;

use crate::textplan::common::error::TextPlanError;
use crate::textplan::common::structured_symbol_data::{FunctionData, RelationData};
use crate::textplan::symbol_table::{SymbolInfo, SymbolTable, SymbolType};

/// A printer for converting Substrait proto expressions to textplan format.
///
/// The ExpressionPrinter handles conversion of Expression protobuf messages
/// into human-readable text representations, including literals, field references,
/// scalar functions, and type annotations.
pub struct ExpressionPrinter<'a> {
    /// The symbol table for looking up function names
    symbol_table: &'a SymbolTable,
    /// The current scope (relation) for resolving field references
    current_scope: Option<&'a Arc<SymbolInfo>>,
    /// Depth of nested function calls (for potential future formatting)
    function_depth: usize,
}

impl<'a> ExpressionPrinter<'a> {
    /// Creates a new ExpressionPrinter with the given symbol table and scope.
    pub fn new(symbol_table: &'a SymbolTable, current_scope: Option<&'a Arc<SymbolInfo>>) -> Self {
        Self {
            symbol_table,
            current_scope,
            function_depth: 0,
        }
    }

    /// Prints an expression to a string.
    pub fn print_expression(
        &mut self,
        expr: &::substrait::proto::Expression,
    ) -> Result<String, TextPlanError> {
        use ::substrait::proto::expression::RexType;

        match &expr.rex_type {
            Some(RexType::Literal(lit)) => self.print_literal(lit),
            Some(RexType::Selection(sel)) => self.print_field_reference(sel),
            Some(RexType::ScalarFunction(func)) => self.print_scalar_function(func),
            Some(RexType::WindowFunction(_)) => {
                Ok("WINDOW_FUNCTION_NOT_YET_IMPLEMENTED".to_string())
            }
            Some(RexType::IfThen(if_then)) => self.print_if_then(if_then),
            Some(RexType::SwitchExpression(_)) => {
                Ok("SWITCH_EXPRESSION_NOT_YET_IMPLEMENTED".to_string())
            }
            Some(RexType::SingularOrList(_)) => {
                Ok("SINGULAR_OR_LIST_NOT_YET_IMPLEMENTED".to_string())
            }
            Some(RexType::MultiOrList(_)) => {
                Ok("MULTI_OR_LIST_NOT_YET_IMPLEMENTED".to_string())
            }
            Some(RexType::Cast(cast)) => self.print_cast(cast),
            Some(RexType::Subquery(_)) => Ok("SUBQUERY_NOT_YET_IMPLEMENTED".to_string()),
            Some(RexType::Nested(_)) => Ok("NESTED_NOT_YET_IMPLEMENTED".to_string()),
            Some(RexType::Enum(_)) => Ok("ENUM_NOT_YET_IMPLEMENTED".to_string()),
            Some(RexType::DynamicParameter(_)) => Ok("DYNAMIC_PARAMETER_NOT_YET_IMPLEMENTED".to_string()),
            None => Err(TextPlanError::InvalidExpression(
                "Expression has no rex_type".to_string(),
            )),
        }
    }

    /// Prints a literal expression.
    fn print_literal(
        &self,
        literal: &::substrait::proto::expression::Literal,
    ) -> Result<String, TextPlanError> {
        use ::substrait::proto::expression::literal::LiteralType;

        let mut result = match &literal.literal_type {
            Some(LiteralType::Boolean(b)) => b.to_string(),
            Some(LiteralType::I8(v)) => format!("{}_i8", v),
            Some(LiteralType::I16(v)) => format!("{}_i16", v),
            Some(LiteralType::I32(v)) => format!("{}_i32", v),
            Some(LiteralType::I64(v)) => format!("{}_i64", v),
            Some(LiteralType::Fp32(v)) => format!("{}_fp32", v),
            Some(LiteralType::Fp64(v)) => format!("{}_fp64", v),
            Some(LiteralType::String(s)) => format!("\"{}\"", escape_string(s)),
            Some(LiteralType::Binary(_)) => "BINARY_LITERAL_NOT_YET_IMPLEMENTED".to_string(),
            Some(LiteralType::Timestamp(_)) => {
                "TIMESTAMP_LITERAL_NOT_YET_IMPLEMENTED".to_string()
            }
            Some(LiteralType::Date(_)) => "DATE_LITERAL_NOT_YET_IMPLEMENTED".to_string(),
            Some(LiteralType::Time(_)) => "TIME_LITERAL_NOT_YET_IMPLEMENTED".to_string(),
            Some(LiteralType::IntervalYearToMonth(_)) => {
                "INTERVAL_YEAR_TO_MONTH_NOT_YET_IMPLEMENTED".to_string()
            }
            Some(LiteralType::IntervalDayToSecond(_)) => {
                "INTERVAL_DAY_TO_SECOND_NOT_YET_IMPLEMENTED".to_string()
            }
            Some(LiteralType::FixedChar(_)) => "FIXED_CHAR_LITERAL_NOT_YET_IMPLEMENTED".to_string(),
            Some(LiteralType::VarChar(_)) => "VARCHAR_LITERAL_NOT_YET_IMPLEMENTED".to_string(),
            Some(LiteralType::FixedBinary(_)) => {
                "FIXED_BINARY_LITERAL_NOT_YET_IMPLEMENTED".to_string()
            }
            Some(LiteralType::Decimal(_)) => "DECIMAL_LITERAL_NOT_YET_IMPLEMENTED".to_string(),
            Some(LiteralType::Struct(_)) => "STRUCT_LITERAL_NOT_YET_IMPLEMENTED".to_string(),
            Some(LiteralType::Map(_)) => "MAP_LITERAL_NOT_YET_IMPLEMENTED".to_string(),
            Some(LiteralType::TimestampTz(_)) => {
                "TIMESTAMP_TZ_LITERAL_NOT_YET_IMPLEMENTED".to_string()
            }
            Some(LiteralType::Uuid(_)) => "UUID_LITERAL_NOT_YET_IMPLEMENTED".to_string(),
            Some(LiteralType::Null(_)) => "NULL".to_string(),
            Some(LiteralType::List(_)) => "LIST_LITERAL_NOT_YET_IMPLEMENTED".to_string(),
            Some(LiteralType::EmptyList(_)) => "EMPTY_LIST_LITERAL_NOT_YET_IMPLEMENTED".to_string(),
            Some(LiteralType::EmptyMap(_)) => "EMPTY_MAP_LITERAL_NOT_YET_IMPLEMENTED".to_string(),
            Some(LiteralType::UserDefined(_)) => {
                "USER_DEFINED_LITERAL_NOT_YET_IMPLEMENTED".to_string()
            }
            Some(LiteralType::IntervalCompound(_)) => {
                "INTERVAL_COMPOUND_LITERAL_NOT_YET_IMPLEMENTED".to_string()
            }
            Some(LiteralType::PrecisionTime(_)) => {
                "PRECISION_TIME_LITERAL_NOT_YET_IMPLEMENTED".to_string()
            }
            Some(LiteralType::PrecisionTimestamp(_)) => {
                "PRECISION_TIMESTAMP_LITERAL_NOT_YET_IMPLEMENTED".to_string()
            }
            Some(LiteralType::PrecisionTimestampTz(_)) => {
                "PRECISION_TIMESTAMP_TZ_LITERAL_NOT_YET_IMPLEMENTED".to_string()
            }
            None => {
                return Err(TextPlanError::InvalidExpression(
                    "Literal has no literal_type".to_string(),
                ))
            }
        };

        // Add nullable marker if needed
        if literal.nullable {
            result.push('?');
        }

        Ok(result)
    }

    /// Prints a field reference (selection).
    fn print_field_reference(
        &self,
        field_ref: &::substrait::proto::expression::FieldReference,
    ) -> Result<String, TextPlanError> {
        use ::substrait::proto::expression::field_reference::ReferenceType;

        match &field_ref.reference_type {
            Some(ReferenceType::DirectReference(direct_ref)) => {
                // Extract outer reference if it exists from root_type
                let outer_ref = match &field_ref.root_type {
                    Some(::substrait::proto::expression::field_reference::RootType::OuterReference(o)) => Some(o),
                    _ => None,
                };
                self.print_direct_reference(direct_ref, outer_ref)
            }
            Some(ReferenceType::MaskedReference(_)) => {
                Ok("MASKED_REFERENCE_NOT_YET_IMPLEMENTED".to_string())
            }
            None => Err(TextPlanError::InvalidExpression(
                "FieldReference has no reference_type".to_string(),
            )),
        }
    }

    /// Prints a direct reference.
    fn print_direct_reference(
        &self,
        direct_ref: &::substrait::proto::expression::ReferenceSegment,
        _outer_ref: Option<&::substrait::proto::expression::field_reference::OuterReference>,
    ) -> Result<String, TextPlanError> {
        use ::substrait::proto::expression::reference_segment::ReferenceType;

        match &direct_ref.reference_type {
            Some(ReferenceType::StructField(struct_field)) => {
                let field_index = struct_field.field as usize;
                self.lookup_field_reference(field_index)
            }
            Some(ReferenceType::MapKey(_)) => {
                Ok("MAP_KEY_REFERENCE_NOT_YET_IMPLEMENTED".to_string())
            }
            Some(ReferenceType::ListElement(_)) => {
                Ok("LIST_ELEMENT_REFERENCE_NOT_YET_IMPLEMENTED".to_string())
            }
            None => Err(TextPlanError::InvalidExpression(
                "DirectReference has no reference_type".to_string(),
            )),
        }
    }

    /// Looks up a field reference in the current scope.
    fn lookup_field_reference(&self, field_index: usize) -> Result<String, TextPlanError> {
        let scope = self.current_scope.ok_or_else(|| {
            TextPlanError::InvalidExpression(
                "Field reference requested outside of a relation scope".to_string(),
            )
        })?;

        // Get the relation data from the scope's blob
        if let Some(blob_lock) = &scope.blob {
            if let Ok(blob_data) = blob_lock.lock() {
                if let Some(relation_data) = blob_data.downcast_ref::<RelationData>() {
                    // First check in field_references
                    if field_index < relation_data.field_references.len() {
                        let symbol = &relation_data.field_references[field_index];
                        return Ok(self.format_field_name(symbol));
                    }

                    // Then check in generated_field_references
                    let adjusted_index = field_index - relation_data.field_references.len();
                    if adjusted_index < relation_data.generated_field_references.len() {
                        let symbol = &relation_data.generated_field_references[adjusted_index];
                        return Ok(self.format_field_name(symbol));
                    }

                    return Err(TextPlanError::InvalidExpression(format!(
                        "Field reference {} out of range (have {} + {} fields)",
                        field_index,
                        relation_data.field_references.len(),
                        relation_data.generated_field_references.len()
                    )));
                }
            }
        }

        Err(TextPlanError::InvalidExpression(
            "Could not access relation data for field reference lookup".to_string(),
        ))
    }

    /// Formats a field name, using fully qualified name if available.
    fn format_field_name(&self, symbol: &Arc<SymbolInfo>) -> String {
        // If the symbol has an alias, use it
        if let Some(alias) = symbol.alias() {
            return alias.to_string();
        }

        // Otherwise, use fully qualified name if schema is available
        if let Some(schema) = symbol.schema() {
            format!("{}.{}", schema.name(), symbol.name())
        } else {
            symbol.name().to_string()
        }
    }

    /// Looks up a function name by its reference anchor.
    fn lookup_function_reference(&self, function_reference: u32) -> String {
        // Search for a function symbol with matching anchor
        for symbol in self.symbol_table.symbols() {
            if symbol.symbol_type() != SymbolType::Function {
                continue;
            }

            // Try to get the function data from the blob
            if let Some(blob_lock) = &symbol.blob {
                if let Ok(blob_data) = blob_lock.lock() {
                    if let Some(function_data) = blob_data.downcast_ref::<FunctionData>() {
                        if function_data.anchor == function_reference {
                            return symbol.name().to_string();
                        }
                    }
                }
            }
        }

        // Function not found, return a placeholder
        format!("functionref#{}", function_reference)
    }

    /// Prints a scalar function call.
    fn print_scalar_function(
        &mut self,
        func: &::substrait::proto::expression::ScalarFunction,
    ) -> Result<String, TextPlanError> {
        self.function_depth += 1;
        let result = self.print_scalar_function_impl(func);
        self.function_depth -= 1;
        result
    }

    /// Implementation of scalar function printing.
    fn print_scalar_function_impl(
        &mut self,
        func: &::substrait::proto::expression::ScalarFunction,
    ) -> Result<String, TextPlanError> {
        let mut result = String::new();

        // Look up the function name from the symbol table
        let function_name = self.lookup_function_reference(func.function_reference);

        result.push_str(&function_name);
        result.push('(');

        let mut first = true;

        // Print arguments (newer protobuf style)
        for arg in &func.arguments {
            if !first {
                result.push_str(", ");
            }
            first = false;

            use ::substrait::proto::function_argument::ArgType;
            match &arg.arg_type {
                Some(ArgType::Enum(enum_val)) => {
                    result.push_str(&format!("{}_enum", enum_val));
                }
                Some(ArgType::Type(type_val)) => {
                    result.push_str(&self.print_type(type_val)?);
                }
                Some(ArgType::Value(expr)) => {
                    result.push_str(&self.print_expression(expr)?);
                }
                None => {
                    result.push_str("MISSING_ARGUMENT");
                }
            }
        }

        // Print args (older protobuf style, for compatibility)
        for arg in &func.args {
            if !first {
                result.push_str(", ");
            }
            first = false;
            result.push_str(&self.print_expression(arg)?);
        }

        result.push(')');

        // Add return type annotation
        if let Some(output_type) = &func.output_type {
            result.push_str("->");
            result.push_str(&self.print_type(output_type)?);
        }

        Ok(result)
    }

    /// Prints a type annotation.
    fn print_type(&self, type_val: &::substrait::proto::Type) -> Result<String, TextPlanError> {
        use ::substrait::proto::r#type::Kind;

        let mut result = String::new();

        let (base_type, nullable) = match &type_val.kind {
            Some(Kind::Bool(bool_type)) => (
                "bool",
                bool_type.nullability == ::substrait::proto::r#type::Nullability::Nullable as i32,
            ),
            Some(Kind::I8(i8_type)) => (
                "i8",
                i8_type.nullability == ::substrait::proto::r#type::Nullability::Nullable as i32,
            ),
            Some(Kind::I16(i16_type)) => (
                "i16",
                i16_type.nullability == ::substrait::proto::r#type::Nullability::Nullable as i32,
            ),
            Some(Kind::I32(i32_type)) => (
                "i32",
                i32_type.nullability == ::substrait::proto::r#type::Nullability::Nullable as i32,
            ),
            Some(Kind::I64(i64_type)) => (
                "i64",
                i64_type.nullability == ::substrait::proto::r#type::Nullability::Nullable as i32,
            ),
            Some(Kind::Fp32(fp32_type)) => (
                "fp32",
                fp32_type.nullability == ::substrait::proto::r#type::Nullability::Nullable as i32,
            ),
            Some(Kind::Fp64(fp64_type)) => (
                "fp64",
                fp64_type.nullability == ::substrait::proto::r#type::Nullability::Nullable as i32,
            ),
            Some(Kind::String(string_type)) => (
                "string",
                string_type.nullability == ::substrait::proto::r#type::Nullability::Nullable as i32,
            ),
            Some(Kind::Binary(binary_type)) => (
                "binary",
                binary_type.nullability == ::substrait::proto::r#type::Nullability::Nullable as i32,
            ),
            Some(Kind::Timestamp(ts_type)) => (
                "timestamp",
                ts_type.nullability == ::substrait::proto::r#type::Nullability::Nullable as i32,
            ),
            Some(Kind::Date(date_type)) => (
                "date",
                date_type.nullability == ::substrait::proto::r#type::Nullability::Nullable as i32,
            ),
            Some(Kind::Time(time_type)) => (
                "time",
                time_type.nullability == ::substrait::proto::r#type::Nullability::Nullable as i32,
            ),
            Some(Kind::IntervalYear(interval_type)) => (
                "interval_year",
                interval_type.nullability == ::substrait::proto::r#type::Nullability::Nullable as i32,
            ),
            Some(Kind::IntervalDay(interval_type)) => (
                "interval_day",
                interval_type.nullability == ::substrait::proto::r#type::Nullability::Nullable as i32,
            ),
            Some(Kind::TimestampTz(ts_type)) => (
                "timestamp_tz",
                ts_type.nullability == ::substrait::proto::r#type::Nullability::Nullable as i32,
            ),
            Some(Kind::Uuid(uuid_type)) => (
                "uuid",
                uuid_type.nullability == ::substrait::proto::r#type::Nullability::Nullable as i32,
            ),
            Some(Kind::FixedChar(fc_type)) => {
                result.push_str("fixedchar");
                if fc_type.nullability == ::substrait::proto::r#type::Nullability::Nullable as i32 {
                    result.push('?');
                }
                result.push_str(&format!("<{}>", fc_type.length));
                return Ok(result);
            }
            Some(Kind::Varchar(vc_type)) => {
                result.push_str("varchar");
                if vc_type.nullability == ::substrait::proto::r#type::Nullability::Nullable as i32 {
                    result.push('?');
                }
                result.push_str(&format!("<{}>", vc_type.length));
                return Ok(result);
            }
            Some(Kind::FixedBinary(fb_type)) => {
                result.push_str("fixedbinary");
                if fb_type.nullability == ::substrait::proto::r#type::Nullability::Nullable as i32 {
                    result.push('?');
                }
                result.push_str(&format!("<{}>", fb_type.length));
                return Ok(result);
            }
            Some(Kind::Decimal(dec_type)) => {
                result.push_str("decimal");
                if dec_type.nullability == ::substrait::proto::r#type::Nullability::Nullable as i32 {
                    result.push('?');
                }
                result.push_str(&format!("<{},{}>", dec_type.precision, dec_type.scale));
                return Ok(result);
            }
            Some(Kind::Struct(_)) => return Ok("STRUCT_TYPE_NOT_YET_IMPLEMENTED".to_string()),
            Some(Kind::List(_)) => return Ok("LIST_TYPE_NOT_YET_IMPLEMENTED".to_string()),
            Some(Kind::Map(_)) => return Ok("MAP_TYPE_NOT_YET_IMPLEMENTED".to_string()),
            Some(Kind::UserDefined(_)) => {
                return Ok("USER_DEFINED_TYPE_NOT_YET_IMPLEMENTED".to_string())
            }
            Some(Kind::UserDefinedTypeReference(_)) => {
                return Ok("USER_DEFINED_TYPE_REF_NOT_YET_IMPLEMENTED".to_string())
            }
            Some(Kind::IntervalCompound(_)) => {
                return Ok("INTERVAL_COMPOUND_TYPE_NOT_YET_IMPLEMENTED".to_string())
            }
            Some(Kind::PrecisionTime(_)) => {
                return Ok("PRECISION_TIME_TYPE_NOT_YET_IMPLEMENTED".to_string())
            }
            Some(Kind::PrecisionTimestamp(_)) => {
                return Ok("PRECISION_TIMESTAMP_TYPE_NOT_YET_IMPLEMENTED".to_string())
            }
            Some(Kind::PrecisionTimestampTz(_)) => {
                return Ok("PRECISION_TIMESTAMP_TZ_TYPE_NOT_YET_IMPLEMENTED".to_string())
            }
            Some(Kind::Alias(_)) => {
                return Ok("ALIAS_TYPE_NOT_YET_IMPLEMENTED".to_string())
            }
            None => {
                return Err(TextPlanError::InvalidExpression(
                    "Type has no kind".to_string(),
                ))
            }
        };

        result.push_str(base_type);
        if nullable {
            result.push('?');
        }

        Ok(result)
    }

    /// Prints an if-then expression.
    fn print_if_then(
        &mut self,
        if_then: &::substrait::proto::expression::IfThen,
    ) -> Result<String, TextPlanError> {
        let mut result = String::from("IFTHEN(");

        let mut first = true;
        for clause in &if_then.ifs {
            if !first {
                result.push_str(", ");
            }
            first = false;

            if let Some(if_expr) = &clause.r#if {
                result.push_str(&self.print_expression(if_expr)?);
            } else {
                result.push_str("MISSING_IF");
            }

            result.push_str(", ");

            if let Some(then_expr) = &clause.then {
                result.push_str(&self.print_expression(then_expr)?);
            } else {
                result.push_str("MISSING_THEN");
            }
        }

        if let Some(else_expr) = &if_then.r#else {
            if !first {
                result.push_str(", ");
            }
            result.push_str(&self.print_expression(else_expr)?);
        }

        result.push(')');
        Ok(result)
    }

    /// Prints a cast expression.
    fn print_cast(
        &mut self,
        cast: &::substrait::proto::expression::Cast,
    ) -> Result<String, TextPlanError> {
        let mut result = String::new();

        if let Some(input) = &cast.input {
            result.push_str(&self.print_expression(input)?);
        } else {
            result.push_str("MISSING_CAST_INPUT");
        }

        result.push_str(" AS ");

        if let Some(type_val) = &cast.r#type {
            result.push_str(&self.print_type(type_val)?);
        } else {
            result.push_str("MISSING_CAST_TYPE");
        }

        Ok(result)
    }
}

/// Escapes a string for output in textplan format.
fn escape_string(s: &str) -> String {
    let mut result = String::new();
    for c in s.chars() {
        match c {
            '"' => result.push_str("\\\""),
            '\\' => result.push_str("\\\\"),
            '\n' => result.push_str("\\n"),
            '\r' => result.push_str("\\r"),
            '\t' => result.push_str("\\t"),
            _ => result.push(c),
        }
    }
    result
}
