// SPDX-License-Identifier: Apache-2.0

//! Plan visitor implementation for traversing Substrait plans.
//! 
//! This module provides the base visitor for Substrait plans and related
//! generator tool to automatically create visitor implementations from protobuf definitions.
use crate::proto::substrait;
use crate::proto::substrait::read_rel::local_files::file_or_files::PathType;
use crate::proto::substrait::rel_common::EmitKind;
use crate::textplan::common::error::TextPlanError;

/// Base visitor trait for Substrait plans.
/// 
/// This trait defines the visit methods for all protobuf message types in the Substrait schema.
/// It's intended to be implemented by concrete visitors that need to traverse and process
/// Substrait plans.
pub trait BasePlanProtoVisitor {
    /// The result type of the visitor
    type Result;

    /// Main entry point to visit a plan
    fn visit_plan(&mut self, plan: &substrait::Plan) -> Self::Result;
}

/// Default implementation that can be used as a starting point for concrete visitors.
/// 
/// This implementation traverses the entire plan structure but does nothing with the values.
/// Implementations can override specific methods to process only the parts they care about.
pub struct DefaultPlanVisitor;

impl DefaultPlanVisitor {
    /// Create a new default visitor
    pub fn new() -> Self {
        Self
    }
}

impl BasePlanProtoVisitor for DefaultPlanVisitor {
    type Result = Result<(), TextPlanError>;

    fn visit_plan(&mut self, _plan: &substrait::Plan) -> Self::Result {
        Ok(())
    }
}

/// Pipeline visitor for traversing and processing Substrait plans.
/// 
/// This visitor is used to traverse a Substrait plan and build a symbol table
/// with relationships between plan elements.
pub struct PipelineVisitor {
    /// Symbol table for storing plan element information
    symbol_table: crate::textplan::symbol_table::SymbolTable,
    /// Current relation context for scope resolution
    current_relation_scope: Option<String>,
}

impl PipelineVisitor {
    /// Create a new pipeline visitor with the given symbol table
    pub fn new(symbol_table: crate::textplan::symbol_table::SymbolTable) -> Self {
        Self {
            symbol_table,
            current_relation_scope: None,
        }
    }

    /// Get the symbol table built by this visitor
    pub fn symbol_table(&self) -> &crate::textplan::symbol_table::SymbolTable {
        &self.symbol_table
    }

    /// Get a mutable reference to the symbol table
    pub fn symbol_table_mut(&mut self) -> &mut crate::textplan::symbol_table::SymbolTable {
        &mut self.symbol_table
    }
}

impl BasePlanProtoVisitor for PipelineVisitor {
    type Result = Result<(), TextPlanError>;

    fn visit_plan(&mut self, plan: &substrait::Plan) -> Self::Result {
        // Process each relation in the plan
        for relation in &plan.relations {
            if let Some(rel_type) = &relation.rel_type {
                match rel_type {
                    substrait::plan_rel::RelType::Rel(rel) => {
                        // Process regular relations
                        self.visit_relation(rel)?;
                    }
                    substrait::plan_rel::RelType::Root(root_rel) => {
                        // Process root relations
                        self.visit_relation_root(root_rel)?;
                    }
                }
            }
        }
        
        Ok(())
    }
}

impl PipelineVisitor {
    /// Visit a relation root
    fn visit_relation_root(&mut self, root_rel: &substrait::RelRoot) -> Result<(), TextPlanError> {
        // Register the root relation in the symbol table
        // This will create a symbol for the root and associate any names with it
        for name in &root_rel.names {
            // Add the named relation to the symbol table
            self.symbol_table.add_root_relation(name);
        }
        
        Ok(())
    }
    
    /// Visit a relation
    fn visit_relation(&mut self, relation: &substrait::Rel) -> Result<(), TextPlanError> {
        // Process the relation based on its type
        if let Some(rel_type) = &relation.rel_type {
            match rel_type {
                substrait::rel::RelType::Read(read_rel) => {
                    self.visit_read_relation(read_rel)?;
                }
                substrait::rel::RelType::Filter(filter_rel) => {
                    self.visit_filter_relation(filter_rel)?;
                }
                substrait::rel::RelType::Fetch(fetch_rel) => {
                    self.visit_fetch_relation(fetch_rel)?;
                }
                substrait::rel::RelType::Aggregate(aggregate_rel) => {
                    self.visit_aggregate_relation(aggregate_rel)?;
                }
                substrait::rel::RelType::Sort(sort_rel) => {
                    self.visit_sort_relation(sort_rel)?;
                }
                substrait::rel::RelType::Join(join_rel) => {
                    self.visit_join_relation(join_rel)?;
                }
                substrait::rel::RelType::Project(project_rel) => {
                    self.visit_project_relation(project_rel)?;
                }
                substrait::rel::RelType::Set(set_rel) => {
                    self.visit_set_relation(set_rel)?;
                }
                // Handle other relation types as needed
                _ => {}
            }
        }
        
        Ok(())
    }
    
    /// Visit a read relation
    fn visit_read_relation(&mut self, read_rel: &substrait::ReadRel) -> Result<(), TextPlanError> {
        // Process common relation elements first
        if let Some(common) = &read_rel.common {
            self.visit_relation_common(common)?;
        }
        
        // Process read source
        if let Some(read_type) = &read_rel.read_type {
            match read_type {
                substrait::read_rel::ReadType::VirtualTable(table) => {
                    self.visit_virtual_table(table)?;
                }
                substrait::read_rel::ReadType::NamedTable(table) => {
                    self.visit_named_table(table)?;
                }
                substrait::read_rel::ReadType::LocalFiles(files) => {
                    self.visit_local_files(files)?;
                }
                substrait::read_rel::ReadType::ExtensionTable(table) => {
                    self.visit_extension_table(table)?;
                }
            }
        }
        
        Ok(())
    }
    
    /// Visit a filter relation
    fn visit_filter_relation(&mut self, filter_rel: &substrait::FilterRel) -> Result<(), TextPlanError> {
        // Process common relation elements
        if let Some(common) = &filter_rel.common {
            self.visit_relation_common(common)?;
        }
        
        // Process filter condition
        if let Some(condition) = &filter_rel.condition {
            self.visit_expression(condition)?;
        }
        
        // Process input relation
        if let Some(input) = &filter_rel.input {
            // Save current scope
            let old_scope = self.current_relation_scope.clone();
            
            // Process input relation with current scope
            self.visit_relation(input)?;
            
            // Restore scope
            self.current_relation_scope = old_scope;
        }
        
        Ok(())
    }
    
    /// Visit a fetch relation
    fn visit_fetch_relation(&mut self, fetch_rel: &substrait::FetchRel) -> Result<(), TextPlanError> {
        // Process common relation elements
        if let Some(common) = &fetch_rel.common {
            self.visit_relation_common(common)?;
        }
        
        // Process input relation
        if let Some(input) = &fetch_rel.input {
            // Save current scope
            let old_scope = self.current_relation_scope.clone();
            
            // Process input relation
            self.visit_relation(input)?;
            
            // Restore scope
            self.current_relation_scope = old_scope;
        }
        
        Ok(())
    }
    
    /// Visit an aggregate relation
    fn visit_aggregate_relation(&mut self, aggregate_rel: &substrait::AggregateRel) -> Result<(), TextPlanError> {
        // Process common relation elements
        if let Some(common) = &aggregate_rel.common {
            self.visit_relation_common(common)?;
        }
        
        // Process input relation
        if let Some(input) = &aggregate_rel.input {
            // Save current scope
            let old_scope = self.current_relation_scope.clone();
            
            // Process input relation
            self.visit_relation(input)?;
            
            // Restore scope
            self.current_relation_scope = old_scope;
        }
        
        // Process groupings
        for grouping in &aggregate_rel.groupings {
            self.visit_grouping(grouping)?;
        }
        
        // Process measures
        for measure in &aggregate_rel.measures {
            self.visit_measure(measure)?;
        }
        
        Ok(())
    }
    
    /// Visit a sort relation
    fn visit_sort_relation(&mut self, sort_rel: &substrait::SortRel) -> Result<(), TextPlanError> {
        // Process common relation elements
        if let Some(common) = &sort_rel.common {
            self.visit_relation_common(common)?;
        }
        
        // Process input relation
        if let Some(input) = &sort_rel.input {
            // Save current scope
            let old_scope = self.current_relation_scope.clone();
            
            // Process input relation
            self.visit_relation(input)?;
            
            // Restore scope
            self.current_relation_scope = old_scope;
        }
        
        // Process sort fields
        for sort_field in &sort_rel.sorts {
            self.visit_sort_field(sort_field)?;
        }
        
        Ok(())
    }
    
    /// Visit a join relation
    fn visit_join_relation(&mut self, join_rel: &substrait::JoinRel) -> Result<(), TextPlanError> {
        // Process common relation elements
        if let Some(common) = &join_rel.common {
            self.visit_relation_common(common)?;
        }
        
        // Process left input relation
        if let Some(left) = &join_rel.left {
            // Save current scope
            let old_scope = self.current_relation_scope.clone();
            
            // Process left relation
            self.visit_relation(left)?;
            
            // Restore scope
            self.current_relation_scope = old_scope;
        }
        
        // Process right input relation
        if let Some(right) = &join_rel.right {
            // Save current scope
            let old_scope = self.current_relation_scope.clone();
            
            // Process right relation
            self.visit_relation(right)?;
            
            // Restore scope
            self.current_relation_scope = old_scope;
        }
        
        // Process expression
        if let Some(expression) = &join_rel.expression {
            self.visit_expression(expression)?;
        }
        
        Ok(())
    }
    
    /// Visit a project relation
    fn visit_project_relation(&mut self, project_rel: &substrait::ProjectRel) -> Result<(), TextPlanError> {
        // Process common relation elements
        if let Some(common) = &project_rel.common {
            self.visit_relation_common(common)?;
        }
        
        // Process input relation
        if let Some(input) = &project_rel.input {
            // Save current scope
            let old_scope = self.current_relation_scope.clone();
            
            // Process input relation
            self.visit_relation(input)?;
            
            // Restore scope
            self.current_relation_scope = old_scope;
        }
        
        // Process expressions
        for expression in &project_rel.expressions {
            self.visit_expression(expression)?;
        }
        
        Ok(())
    }
    
    /// Visit a set relation
    fn visit_set_relation(&mut self, set_rel: &substrait::SetRel) -> Result<(), TextPlanError> {
        // Process common relation elements
        if let Some(common) = &set_rel.common {
            self.visit_relation_common(common)?;
        }
        
        // Process input relations
        for input in &set_rel.inputs {
            // Save current scope
            let old_scope = self.current_relation_scope.clone();
            
            // Process input relation
            self.visit_relation(input)?;
            
            // Restore scope
            self.current_relation_scope = old_scope;
        }
        
        Ok(())
    }
    
    /// Visit relation common elements
    fn visit_relation_common(&mut self, common: &substrait::RelCommon) -> Result<(), TextPlanError> {
        // Process the emit pattern if present
        if let Some(emit_kind) = &common.emit_kind {
            match emit_kind {
                EmitKind::Direct(_) => {
                    // Nothing to do here.
                },
                EmitKind::Emit(emit) => {
                    for item in &emit.output_mapping {
                        // Register field mappings in the symbol table
                        if let Some(scope) = &self.current_relation_scope {
                            self.symbol_table.add_field_mapping(scope, *item);
                        }
                    }
                }
            }
        }
        
        Ok(())
    }
    
    /// Visit a virtual table
    fn visit_virtual_table(&mut self, table: &substrait::read_rel::VirtualTable) -> Result<(), TextPlanError> {
        // Process the values in the virtual table
        for row in &table.values {
            for item in &row.fields {
                self.visit_literal(item)?;
            }
        }
        
        Ok(())
    }
    
    /// Visit a named table
    fn visit_named_table(&mut self, table: &substrait::read_rel::NamedTable) -> Result<(), TextPlanError> {
        // Register the table in the symbol table
        if let Some(scope) = &self.current_relation_scope {
            self.symbol_table.add_named_table(scope, &table.names);
        }
        
        Ok(())
    }
    
    /// Visit local files
    fn visit_local_files(&mut self, files: &substrait::read_rel::LocalFiles) -> Result<(), TextPlanError> {
        // Process file items
        for item in &files.items {
            self.visit_file_or_files(item)?;
        }
        
        Ok(())
    }
    
    /// Visit extension table
    fn visit_extension_table(&mut self, _table: &substrait::read_rel::ExtensionTable) -> Result<(), TextPlanError> {
        // Process extension table as needed
        Ok(())
    }
    
    /// Visit file or files
    fn visit_file_or_files(&mut self, item: &substrait::read_rel::local_files::FileOrFiles) -> Result<(), TextPlanError> {
        // Register file information in the symbol table
        if let Some(path_type) = &item.path_type {
            match path_type {
                substrait::read_rel::local_files::file_or_files::PathType::UriFile(uri) => {
                    if let Some(scope) = &self.current_relation_scope {
                        self.symbol_table.add_file_source(scope, uri);
                    }
                }
                substrait::read_rel::local_files::file_or_files::PathType::UriFolder(uri) => {
                    if let Some(scope) = &self.current_relation_scope {
                        self.symbol_table.add_folder_source(scope, uri);
                    }
                }
                substrait::read_rel::local_files::file_or_files::PathType::UriPath(paths) => {
                    if let Some(scope) = &self.current_relation_scope {
                        self.symbol_table.add_file_source(scope, paths);
                    }
                }
                PathType::UriPathGlob(_) => {}
            }
        }
        
        Ok(())
    }
    
    /// Visit a grouping
    fn visit_grouping(&mut self, grouping: &substrait::aggregate_rel::Grouping) -> Result<(), TextPlanError> {
        // Process grouping keys
        for key in &grouping.grouping_expressions {
            self.visit_expression(key)?;
        }
        
        Ok(())
    }
    
    /// Visit a measure
    fn visit_measure(&mut self, measure: &substrait::aggregate_rel::Measure) -> Result<(), TextPlanError> {
        // Process measure function
        if let Some(function) = &measure.measure {
            self.visit_aggregate_function(function)?;
        }
        
        Ok(())
    }
    
    /// Visit a sort field
    fn visit_sort_field(&mut self, sort_field: &substrait::SortField) -> Result<(), TextPlanError> {
        // Process sort expression
        if let Some(expr) = &sort_field.expr {
            self.visit_expression(expr)?;
        }
        
        Ok(())
    }
    
    /// Visit an expression
    fn visit_expression(&mut self, expression: &substrait::Expression) -> Result<(), TextPlanError> {
        // Process expressions based on their type
        if let Some(rex_type) = &expression.rex_type {
            match rex_type {
                substrait::expression::RexType::Literal(literal) => {
                    self.visit_literal(literal)?;
                }
                substrait::expression::RexType::ScalarFunction(function) => {
                    self.visit_scalar_function(function)?;
                }
                substrait::expression::RexType::WindowFunction(function) => {
                    self.visit_window_function(function)?;
                }
                substrait::expression::RexType::IfThen(if_then) => {
                    self.visit_if_then(if_then)?;
                }
                substrait::expression::RexType::SwitchExpression(switch) => {
                    self.visit_switch_expression(switch)?;
                }
                substrait::expression::RexType::SingularOrList(list) => {
                    self.visit_singular_or_list(list)?;
                }
                substrait::expression::RexType::MultiOrList(list) => {
                    self.visit_multi_or_list(list)?;
                }
                substrait::expression::RexType::Cast(cast) => {
                    self.visit_cast(cast)?;
                }
                substrait::expression::RexType::Subquery(subquery) => {
                    self.visit_subquery(subquery)?;
                }
                substrait::expression::RexType::Selection(reference) => {
                    self.visit_field_reference(reference)?;
                }
                // Handle other expression types as needed
                _ => {}
            }
        }
        
        Ok(())
    }
    
    /// Visit a literal expression
    fn visit_literal(&mut self, literal: &substrait::expression::Literal) -> Result<(), TextPlanError> {
        // Process literal based on its type
        if let Some(literal_type) = &literal.literal_type {
            match literal_type {
                substrait::expression::literal::LiteralType::Boolean(_) => {}
                substrait::expression::literal::LiteralType::I8(_) => {}
                substrait::expression::literal::LiteralType::I16(_) => {}
                substrait::expression::literal::LiteralType::I32(_) => {}
                substrait::expression::literal::LiteralType::I64(_) => {}
                substrait::expression::literal::LiteralType::Fp32(_) => {}
                substrait::expression::literal::LiteralType::Fp64(_) => {}
                substrait::expression::literal::LiteralType::String(s) => {
                    // Register string literals in the symbol table if needed
                    if let Some(scope) = &self.current_relation_scope {
                        self.symbol_table.add_string_literal(scope, s);
                    }
                }
                substrait::expression::literal::LiteralType::Binary(_) => {}
                substrait::expression::literal::LiteralType::Timestamp(_) => {}
                substrait::expression::literal::LiteralType::Date(_) => {}
                substrait::expression::literal::LiteralType::Time(_) => {}
                substrait::expression::literal::LiteralType::IntervalYearToMonth(_) => {}
                substrait::expression::literal::LiteralType::IntervalDayToSecond(_) => {}
                substrait::expression::literal::LiteralType::FixedChar(_) => {}
                substrait::expression::literal::LiteralType::VarChar(_) => {}
                substrait::expression::literal::LiteralType::FixedBinary(_) => {}
                substrait::expression::literal::LiteralType::Decimal(_) => {}
                substrait::expression::literal::LiteralType::Struct(s) => {
                    self.visit_expression_literal_struct(s)?;
                }
                substrait::expression::literal::LiteralType::Map(m) => {
                    self.visit_map(m)?;
                }
                substrait::expression::literal::LiteralType::List(l) => {
                    self.visit_list(l)?;
                }
                substrait::expression::literal::LiteralType::EmptyList(_) => {}
                substrait::expression::literal::LiteralType::Null(_) => {}
                substrait::expression::literal::LiteralType::UserDefined(u) => {
                    self.visit_user_defined(u)?;
                },
                substrait::expression::literal::LiteralType::TimestampTz(_) => {},
                substrait::expression::literal::LiteralType::Uuid(_) => {},
                substrait::expression::literal::LiteralType::EmptyMap(_) => {}
            }
        }
        
        Ok(())
    }
    
    /// Visit a scalar function
    fn visit_scalar_function(&mut self, function: &substrait::expression::ScalarFunction) -> Result<(), TextPlanError> {
        // Process function arguments
        for arg in &function.arguments {
            self.visit_function_argument(arg)?;
        }
        
        // Process function options
        for option in &function.options {
            self.visit_function_option(option)?;
        }
        
        Ok(())
    }
    
    /// Visit a window function
    fn visit_window_function(&mut self, function: &substrait::expression::WindowFunction) -> Result<(), TextPlanError> {
        // Process function arguments
        for arg in &function.arguments {
            self.visit_function_argument(arg)?;
        }
        
        // Process function options
        for option in &function.options {
            self.visit_function_option(option)?;
        }
        
        // Process partitioning expressions
        for expr in &function.partitions {
            self.visit_expression(expr)?;
        }
        
        // Process sort fields
        for sort in &function.sorts {
            self.visit_sort_field(sort)?;
        }
        
        Ok(())
    }
    
    /// Visit an if-then expression
    fn visit_if_then(&mut self, if_then: &substrait::expression::IfThen) -> Result<(), TextPlanError> {
        // Process if clauses
        for clause in &if_then.ifs {
            self.visit_if_clause(clause)?;
        }
        
        // Process else expression
        if let Some(else_expr) = &if_then.r#else {
            self.visit_expression(else_expr)?;
        }
        
        Ok(())
    }
    
    /// Visit a switch expression
    fn visit_switch_expression(&mut self, switch: &substrait::expression::SwitchExpression) -> Result<(), TextPlanError> {
        // Process match expression
        if let Some(match_expr) = &switch.r#match {
            self.visit_expression(match_expr)?;
        }
        
        // Process if-value cases
        for if_value in &switch.ifs {
            self.visit_if_value(if_value)?;
        }
        
        // Process else expression
        if let Some(else_expr) = &switch.r#else {
            self.visit_expression(else_expr)?;
        }
        
        Ok(())
    }
    
    /// Visit a singular or list
    fn visit_singular_or_list(&mut self, list: &substrait::expression::SingularOrList) -> Result<(), TextPlanError> {
        // Process value expression
        if let Some(value) = &list.value {
            self.visit_expression(value)?;
        }
        
        // Process options
        for option in &list.options {
            self.visit_expression(option)?;
        }
        
        Ok(())
    }
    
    /// Visit a multi or list
    fn visit_multi_or_list(&mut self, list: &substrait::expression::MultiOrList) -> Result<(), TextPlanError> {
        // Process value expressions
        for value in &list.value {
            self.visit_expression(value)?;
        }
        
        // Process record items
        for record in &list.options {
            self.visit_record(record)?;
        }
        
        Ok(())
    }
    
    /// Visit a cast expression
    fn visit_cast(&mut self, cast: &substrait::expression::Cast) -> Result<(), TextPlanError> {
        // Process input expression
        if let Some(expr) = &cast.input {
            self.visit_expression(expr)?;
        }
        
        // Process type
        if let Some(ty) = &cast.r#type {
            self.visit_type(ty)?;
        }
        
        Ok(())
    }
    
    /// Visit a subquery
    fn visit_subquery(&mut self, subquery: &substrait::expression::Subquery) -> Result<(), TextPlanError> {
        // Process subquery based on type
        if let Some(subquery_type) = &subquery.subquery_type {
            match subquery_type {
                substrait::expression::subquery::SubqueryType::Scalar(scalar) => {
                    self.visit_subquery_scalar(scalar)?;
                }
                substrait::expression::subquery::SubqueryType::InPredicate(predicate) => {
                    self.visit_subquery_in_predicate(predicate)?;
                }
                substrait::expression::subquery::SubqueryType::SetPredicate(predicate) => {
                    self.visit_subquery_set_predicate(predicate)?;
                }
                substrait::expression::subquery::SubqueryType::SetComparison(comparison) => {
                    self.visit_subquery_set_comparison(comparison)?;
                }
            }
        }
        
        Ok(())
    }
    
    /// Visit a field reference
    fn visit_field_reference(&mut self, reference: &substrait::expression::FieldReference) -> Result<(), TextPlanError> {
        // Process reference segments
        if let Some(ref_type) = &reference.reference_type {
            match ref_type {
                substrait::expression::field_reference::ReferenceType::DirectReference(_) => {

                }
                substrait::expression::field_reference::ReferenceType::MaskedReference(masked) => {
                    self.visit_mask_expression(masked)?;
                }
            }
        }
        
        Ok(())
    }
    
    /// Visit an expression literal struct
    fn visit_expression_literal_struct(&mut self, structure: &substrait::expression::literal::Struct) -> Result<(), TextPlanError> {
        // Process field expressions
        for field in &structure.fields {
            self.visit_literal(field)?;
        }
        
        Ok(())
    }
    
    /// Visit a map
    fn visit_map(&mut self, map: &substrait::expression::literal::Map) -> Result<(), TextPlanError> {
        // Process key-value pairs
        for kv in &map.key_values {
            self.visit_map_key_value(kv)?;
        }
        
        Ok(())
    }
    
    /// Visit a map key-value pair
    fn visit_map_key_value(&mut self, kv: &substrait::expression::literal::map::KeyValue) -> Result<(), TextPlanError> {
        // Process key and value expressions
        if let Some(key) = &kv.key {
            self.visit_literal(key)?;
        }
        
        if let Some(value) = &kv.value {
            self.visit_literal(value)?;
        }
        
        Ok(())
    }
    
    /// Visit a list
    fn visit_list(&mut self, list: &substrait::expression::literal::List) -> Result<(), TextPlanError> {
        // Process list values
        for value in &list.values {
            self.visit_literal(value)?;
        }
        
        Ok(())
    }
    
    /// Visit a user-defined type
    fn visit_user_defined(&mut self, _user_defined: &substrait::expression::literal::UserDefined) -> Result<(), TextPlanError> {
        // Process user-defined type as needed
        Ok(())
    }
    
    /// Visit a function argument
    fn visit_function_argument(&mut self, argument: &substrait::FunctionArgument) -> Result<(), TextPlanError> {
        // Process argument based on type
        if let Some(arg_type) = &argument.arg_type {
            match arg_type {
                substrait::function_argument::ArgType::Value(expr) => {
                    self.visit_expression(expr)?;
                }
                substrait::function_argument::ArgType::Enum(_) => {
                    // Process enum value if needed
                }
                substrait::function_argument::ArgType::Type(ty) => {
                    self.visit_type(ty)?;
                }
            }
        }
        
        Ok(())
    }
    
    /// Visit a function option
    fn visit_function_option(&mut self, _option: &substrait::FunctionOption) -> Result<(), TextPlanError> {
        // Process option name and preference
        // No complex processing needed here for most cases
        
        Ok(())
    }
    
    /// Visit an if clause
    fn visit_if_clause(&mut self, if_clause: &substrait::expression::if_then::IfClause) -> Result<(), TextPlanError> {
        // Process condition and then expressions
        if let Some(condition) = &if_clause.r#if {
            self.visit_expression(condition)?;
        }
        
        if let Some(then) = &if_clause.then {
            self.visit_expression(then)?;
        }
        
        Ok(())
    }
    
    /// Visit an if value
    fn visit_if_value(&mut self, if_value: &substrait::expression::switch_expression::IfValue) -> Result<(), TextPlanError> {
        // Process if and then expressions
        if let Some(if_expr) = &if_value.r#if {
            self.visit_literal(if_expr)?;
        }
        
        if let Some(then) = &if_value.then {
            self.visit_expression(then)?;
        }
        
        Ok(())
    }
    
    /// Visit a record
    fn visit_record(&mut self, record: &substrait::expression::multi_or_list::Record) -> Result<(), TextPlanError> {
        // Process record values
        for value in &record.fields {
            self.visit_expression(value)?;
        }
        
        Ok(())
    }
    
    /// Visit a type
    fn visit_type(&mut self, ty: &substrait::Type) -> Result<(), TextPlanError> {
        // Process type based on its kind
        if let Some(kind) = &ty.kind {
            match kind {
                substrait::r#type::Kind::Bool(_) => {}
                substrait::r#type::Kind::I8(_) => {}
                substrait::r#type::Kind::I16(_) => {}
                substrait::r#type::Kind::I32(_) => {}
                substrait::r#type::Kind::I64(_) => {}
                substrait::r#type::Kind::Fp32(_) => {}
                substrait::r#type::Kind::Fp64(_) => {}
                substrait::r#type::Kind::String(_) => {}
                substrait::r#type::Kind::Binary(_) => {}
                substrait::r#type::Kind::Timestamp(_) => {}
                substrait::r#type::Kind::TimestampTz(_) => {}
                substrait::r#type::Kind::Date(_) => {}
                substrait::r#type::Kind::Time(_) => {}
                substrait::r#type::Kind::IntervalYear(_) => {}
                substrait::r#type::Kind::IntervalDay(_) => {}
                substrait::r#type::Kind::Uuid(_) => {}
                substrait::r#type::Kind::FixedChar(_) => {}
                substrait::r#type::Kind::Varchar(_) => {}
                substrait::r#type::Kind::FixedBinary(_) => {}
                substrait::r#type::Kind::Decimal(_) => {}
                substrait::r#type::Kind::Struct(structure) => {
                    self.visit_struct(structure)?;
                }
                substrait::r#type::Kind::List(list) => {
                    self.visit_type_list(list)?;
                }
                substrait::r#type::Kind::Map(map) => {
                    self.visit_type_map(map)?;
                }
                substrait::r#type::Kind::UserDefined(user_defined) => {
                    self.visit_type_user_defined(user_defined)?;
                }
                substrait::r#type::Kind::UserDefinedTypeReference(_) => {}
            }
        }
        
        Ok(())
    }
    
    /// Visit a struct type
    fn visit_struct(&mut self, structure: &substrait::r#type::Struct) -> Result<(), TextPlanError> {
        // Process struct types
        for field in &structure.types {
            self.visit_type(field)?;
        }
        
        Ok(())
    }
    
    /// Visit a type list
    fn visit_type_list(&mut self, list: &substrait::r#type::List) -> Result<(), TextPlanError> {
        // Process list type
        if let Some(ty) = &list.r#type {
            self.visit_type(ty)?;
        }
        
        Ok(())
    }
    
    /// Visit a type map
    fn visit_type_map(&mut self, map: &substrait::r#type::Map) -> Result<(), TextPlanError> {
        // Process key and value types
        if let Some(key) = &map.key {
            self.visit_type(key)?;
        }
        
        if let Some(value) = &map.value {
            self.visit_type(value)?;
        }
        
        Ok(())
    }
    
    /// Visit a user-defined type
    fn visit_type_user_defined(&mut self, _user_defined: &substrait::r#type::UserDefined) -> Result<(), TextPlanError> {
        // Process user-defined type as needed
        Ok(())
    }
    
    /// Visit a type parameter
    fn visit_type_parameter(&mut self, _parameter: &substrait::r#type::Parameter) -> Result<(), TextPlanError> {
        // Process type parameter as needed
        Ok(())
    }
    
    /// Visit a reference segment
    fn visit_reference_segment(&mut self, segment: &substrait::expression::ReferenceSegment) -> Result<(), TextPlanError> {
        // Process segment based on type
        if let Some(segment_type) = &segment.reference_type {
            match segment_type {
                substrait::expression::reference_segment::ReferenceType::MapKey(map_key) => {
                    self.visit_reference_segment_map_key(map_key)?;
                }
                substrait::expression::reference_segment::ReferenceType::StructField(field) => {
                    self.visit_reference_segment_struct_field(field)?;
                }
                substrait::expression::reference_segment::ReferenceType::ListElement(element) => {
                    self.visit_reference_segment_list_element(element)?;
                }
            }
        }
        
        Ok(())
    }
    
    /// Visit a map key reference segment
    fn visit_reference_segment_map_key(&mut self, map_key: &substrait::expression::reference_segment::MapKey) -> Result<(), TextPlanError> {
        // Process map key child
        if let Some(map_key_expr) = &map_key.map_key {
            self.visit_literal(map_key_expr)?;
        }
        
        Ok(())
    }
    
    /// Visit a struct field reference segment
    fn visit_reference_segment_struct_field(&mut self, _field: &substrait::expression::reference_segment::StructField) -> Result<(), TextPlanError> {
        // Process struct field reference
        // Typically just handles field index, which doesn't need complex processing
        
        Ok(())
    }
    
    /// Visit a list element reference segment
    fn visit_reference_segment_list_element(&mut self, _element: &substrait::expression::reference_segment::ListElement) -> Result<(), TextPlanError> {
        // Process list element reference
        // Typically just handles offset, which doesn't need complex processing
        
        Ok(())
    }
    
    /// Visit a mask expression
    fn visit_mask_expression(&mut self, mask: &substrait::expression::MaskExpression) -> Result<(), TextPlanError> {
        if let Some(struct_select) = &mask.select {

            for (_, struct_item) in struct_select.struct_items.iter().enumerate() {
                if let Some(child) = &struct_item.child {
                    if let Some(child_type) = &child.r#type {
                        match child_type {
                            substrait::expression::mask_expression::select::Type::Struct(struct_select) => {
                                self.visit_struct_select(struct_select)?;
                            }
                            substrait::expression::mask_expression::select::Type::List(list_select) => {
                                self.visit_list_select(list_select)?;
                            }
                            substrait::expression::mask_expression::select::Type::Map(map_select) => {
                                self.visit_map_select(map_select)?;
                            }
                        }
                    }
                }
            }
        }
        
        Ok(())
    }
    
    /// Visit a struct select mask
    fn visit_struct_select(&mut self, select: &substrait::expression::mask_expression::StructSelect) -> Result<(), TextPlanError> {
        // Process struct items
        for item in &select.struct_items {
            self.visit_struct_item(item)?;
        }
        
        Ok(())
    }
    
    /// Visit a struct item
    fn visit_struct_item(&mut self, _item: &substrait::expression::mask_expression::StructItem) -> Result<(), TextPlanError> {
        // Process struct item
        // Typically just handles field index, which doesn't need complex processing
        
        Ok(())
    }
    
    /// Visit a list select mask
    fn visit_list_select(&mut self, select: &substrait::expression::mask_expression::ListSelect) -> Result<(), TextPlanError> {
        // Process list select items
        for item in &select.selection {
            self.visit_list_select_item(item)?;
        }
        
        Ok(())
    }
    
    /// Visit a list select item
    fn visit_list_select_item(&mut self, _item: &substrait::expression::mask_expression::list_select::ListSelectItem) -> Result<(), TextPlanError> {
        // Process list select item
        // Typically just handles index range, which doesn't need complex processing
        
        Ok(())
    }
    
    /// Visit a map select mask
    fn visit_map_select(&mut self, _select: &substrait::expression::mask_expression::MapSelect) -> Result<(), TextPlanError> {
        // Process map select
        // Typically just processes key expressions which are already handled elsewhere
        
        Ok(())
    }
    
    /// Visit a subquery scalar
    fn visit_subquery_scalar(&mut self, scalar: &substrait::expression::subquery::Scalar) -> Result<(), TextPlanError> {
        // Process input relation
        if let Some(input) = &scalar.input {
            // Save current scope
            let old_scope = self.current_relation_scope.clone();
            
            // Process subquery relation
            self.visit_relation(input)?;
            
            // Restore scope
            self.current_relation_scope = old_scope;
        }
        
        Ok(())
    }
    
    /// Visit a subquery in predicate
    fn visit_subquery_in_predicate(&mut self, predicate: &substrait::expression::subquery::InPredicate) -> Result<(), TextPlanError> {
        // Process haystack relation
        if let Some(haystack) = &predicate.haystack {
            // Save current scope
            let old_scope = self.current_relation_scope.clone();
            
            // Process haystack relation
            self.visit_relation(haystack)?;
            
            // Restore scope
            self.current_relation_scope = old_scope;
        }
        
        // Process needles
        for needle in &predicate.needles {
            self.visit_expression(needle)?;
        }
        
        Ok(())
    }
    
    /// Visit a subquery set predicate
    fn visit_subquery_set_predicate(&mut self, predicate: &substrait::expression::subquery::SetPredicate) -> Result<(), TextPlanError> {
        // Process tuples relation
        if let Some(tuples) = &predicate.tuples {
            // Save current scope
            let old_scope = self.current_relation_scope.clone();
            
            // Process tuples relation
            self.visit_relation(tuples)?;
            
            // Restore scope
            self.current_relation_scope = old_scope;
        }
        
        Ok(())
    }
    
    /// Visit a subquery set comparison
    fn visit_subquery_set_comparison(&mut self, comparison: &substrait::expression::subquery::SetComparison) -> Result<(), TextPlanError> {
        // Process left expressions
        if let Some(expr) = &comparison.left {
            self.visit_expression(expr)?;
        }
        
        // Process right relation
        if let Some(right) = &comparison.right {
            // Save current scope
            let old_scope = self.current_relation_scope.clone();
            
            // Process right relation
            self.visit_relation(right)?;
            
            // Restore scope
            self.current_relation_scope = old_scope;
        }
        
        Ok(())
    }
    
    /// Visit an aggregate function
    fn visit_aggregate_function(&mut self, function: &substrait::AggregateFunction) -> Result<(), TextPlanError> {
        // Process function arguments
        for arg in &function.arguments {
            self.visit_function_argument(arg)?;
        }
        
        // Process function options
        for option in &function.options {
            self.visit_function_option(option)?;
        }
        
        // Process sort fields
        for sort in &function.sorts {
            self.visit_sort_field(sort)?;
        }
        
        Ok(())
    }
}

/// Generator tool to create visitor code from protobuf definitions.
/// 
/// This struct provides functionality to generate a Rust implementation of the
/// BasePlanProtoVisitor trait from Substrait protobuf schema.
pub struct VisitorGenerator {
    output: String,
}

impl VisitorGenerator {
    /// Create a new visitor generator
    pub fn new() -> Self {
        Self {
            output: String::new(),
        }
    }

    /// Generate the visitor code
    pub fn generate(&mut self) -> String {
        self.output.clear();
        
        // Generate the header
        self.add_header();
        
        // Generate visitor methods for all message types
        self.generate_visitor_methods();
        
        // Generate default implementation
        self.generate_default_implementation();
        
        self.output.clone()
    }

    /// Generate the file header
    fn add_header(&mut self) {
        self.output.push_str("// SPDX-License-Identifier: Apache-2.0\n\n");
        self.output.push_str("//! GENERATED CODE - DO NOT MODIFY\n");
        self.output.push_str("//! Generated visitor for Substrait protocol buffers.\n\n");
        
        self.output.push_str("use std::fmt;\n");
        self.output.push_str("use crate::proto::substrait;\n");
        self.output.push_str("use crate::textplan::common::error::TextPlanError;\n\n");
    }

    /// Generate visitor methods for all message types
    fn generate_visitor_methods(&mut self) {
        self.output.push_str("/// Base visitor trait for Substrait plans.\n");
        self.output.push_str("/// \n");
        self.output.push_str("/// This trait defines the visit methods for all protobuf message types in the Substrait schema.\n");
        self.output.push_str("/// It's intended to be implemented by concrete visitors that need to traverse and process\n");
        self.output.push_str("/// Substrait plans.\n");
        self.output.push_str("pub trait BasePlanProtoVisitor {\n");
        self.output.push_str("    /// The result type of the visitor\n");
        self.output.push_str("    type Result;\n\n");
        
        // Add the main entry point
        self.output.push_str("    /// Main entry point to visit a plan\n");
        self.output.push_str("    fn visit_plan(&mut self, plan: &substrait::Plan) -> Self::Result;\n");
        
        // For a complete implementation, we would need to add all visit methods here
        // based on the protobuf schema. For now, we'll add just a few examples.
        
        self.output.push_str("\n    /// Visit a relation\n");
        self.output.push_str("    fn visit_relation(&mut self, relation: &substrait::Rel) -> Self::Result;\n");
        
        self.output.push_str("\n    /// Visit a read relation\n");
        self.output.push_str("    fn visit_read_relation(&mut self, relation: &substrait::ReadRel) -> Self::Result;\n");
        
        self.output.push_str("\n    /// Visit an expression\n");
        self.output.push_str("    fn visit_expression(&mut self, expression: &substrait::Expression) -> Self::Result;\n");
        
        // Close the trait
        self.output.push_str("}\n\n");
    }

    /// Generate default implementation
    fn generate_default_implementation(&mut self) {
        self.output.push_str("/// Default implementation that can be used as a starting point for concrete visitors.\n");
        self.output.push_str("/// \n");
        self.output.push_str("/// This implementation traverses the entire plan structure but does nothing with the values.\n");
        self.output.push_str("/// Implementations can override specific methods to process only the parts they care about.\n");
        self.output.push_str("pub struct DefaultPlanVisitor;\n\n");
        
        self.output.push_str("impl DefaultPlanVisitor {\n");
        self.output.push_str("    /// Create a new default visitor\n");
        self.output.push_str("    pub fn new() -> Self {\n");
        self.output.push_str("        Self\n");
        self.output.push_str("    }\n");
        self.output.push_str("}\n\n");
        
        self.output.push_str("impl BasePlanProtoVisitor for DefaultPlanVisitor {\n");
        self.output.push_str("    type Result = Result<(), TextPlanError>;\n\n");
        
        // Add default implementations for the methods
        self.output.push_str("    fn visit_plan(&mut self, plan: &substrait::Plan) -> Self::Result {\n");
        self.output.push_str("        // Visit all relations in the plan\n");
        self.output.push_str("        for relation in &plan.relations {\n");
        self.output.push_str("            if let Some(rel_type) = &relation.rel_type {\n");
        self.output.push_str("                match rel_type {\n");
        self.output.push_str("                    substrait::plan_rel::RelType::Rel(rel) => {\n");
        self.output.push_str("                        self.visit_relation(rel)?;\n");
        self.output.push_str("                    }\n");
        self.output.push_str("                    substrait::plan_rel::RelType::Root(_) => {\n");
        self.output.push_str("                        // Handle root relation\n");
        self.output.push_str("                    }\n");
        self.output.push_str("                }\n");
        self.output.push_str("            }\n");
        self.output.push_str("        }\n");
        self.output.push_str("        Ok(())\n");
        self.output.push_str("    }\n\n");
        
        self.output.push_str("    fn visit_relation(&mut self, relation: &substrait::Rel) -> Self::Result {\n");
        self.output.push_str("        // Visit based on relation type\n");
        self.output.push_str("        if let Some(rel_type) = &relation.rel_type {\n");
        self.output.push_str("            match rel_type {\n");
        self.output.push_str("                substrait::rel::RelType::Read(read_rel) => {\n");
        self.output.push_str("                    self.visit_read_relation(read_rel)?;\n");
        self.output.push_str("                }\n");
        self.output.push_str("                // Handle other relation types\n");
        self.output.push_str("                _ => {}\n");
        self.output.push_str("            }\n");
        self.output.push_str("        }\n");
        self.output.push_str("        Ok(())\n");
        self.output.push_str("    }\n\n");
        
        self.output.push_str("    fn visit_read_relation(&mut self, _relation: &substrait::ReadRel) -> Self::Result {\n");
        self.output.push_str("        // Process read relation\n");
        self.output.push_str("        Ok(())\n");
        self.output.push_str("    }\n\n");
        
        self.output.push_str("    fn visit_expression(&mut self, _expression: &substrait::Expression) -> Self::Result {\n");
        self.output.push_str("        // Process expression\n");
        self.output.push_str("        Ok(())\n");
        self.output.push_str("    }\n");
        
        // Close the impl
        self.output.push_str("}\n");
    }
}