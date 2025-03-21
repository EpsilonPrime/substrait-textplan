// SPDX-License-Identifier: Apache-2.0

//! Plan visitor implementation for traversing Substrait plans.
//!
//! This module provides implementations of visitors for processing Substrait plans.
//! It builds on the generated BasePlanProtoVisitor trait to provide specialized
//! visitors for different stages of plan processing.

use crate::textplan::common::error::TextPlanError;
use crate::textplan::converter::generated::BasePlanProtoVisitor;
use ::substrait::proto as substrait;
use ::substrait::proto::read_rel::local_files::file_or_files::PathType;
use ::substrait::proto::rel_common::EmitKind;

/// Initial visitor implementation that builds a symbol table from a Substrait plan.
///
/// This visitor is used to populate a symbol table with relationships between plan elements
/// during the first traversal of the plan.
pub struct InitialPlanVisitor {
    /// Symbol table for storing plan element information
    symbol_table: crate::textplan::symbol_table::SymbolTable,
    /// Current relation context for scope resolution
    current_relation_scope: Option<String>,
}

impl InitialPlanVisitor {
    /// Create a new initial visitor with the given symbol table
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

    /// Visit a relation root
    fn visit_relation_root(&mut self, root_rel: &substrait::RelRoot) -> Result<(), TextPlanError> {
        // Register the root relation in the symbol table
        for name in &root_rel.names {
            // Add the named relation to the symbol table
            self.symbol_table.add_root_relation(name);
        }

        Ok(())
    }
}

impl BasePlanProtoVisitor for InitialPlanVisitor {
    fn visit_plan(&mut self, plan: &substrait::Plan) -> impl std::any::Any {
        // Process each relation in the plan
        for relation in &plan.relations {
            if let Some(rel_type) = &relation.rel_type {
                match rel_type {
                    substrait::plan_rel::RelType::Rel(rel) => {
                        // Process regular relations
                        self.visit_rel(rel);
                    }
                    substrait::plan_rel::RelType::Root(root_rel) => {
                        // Process root relations
                        self.visit_rel_root(root_rel);
                    }
                }
            }
        }
    }

    fn visit_rel(&mut self, relation: &substrait::Rel) -> impl std::any::Any {
        // Process the relation based on its type
        if let Some(rel_type) = &relation.rel_type {
            match rel_type {
                substrait::rel::RelType::Read(read_rel) => {
                    self.visit_read_rel(read_rel);
                }
                substrait::rel::RelType::Filter(filter_rel) => {
                    self.visit_filter_rel(filter_rel);
                }
                substrait::rel::RelType::Fetch(fetch_rel) => {
                    self.visit_fetch_rel(fetch_rel);
                }
                substrait::rel::RelType::Aggregate(aggregate_rel) => {
                    self.visit_aggregate_rel(aggregate_rel);
                }
                substrait::rel::RelType::Sort(sort_rel) => {
                    self.visit_sort_rel(sort_rel);
                }
                substrait::rel::RelType::Join(join_rel) => {
                    self.visit_join_rel(join_rel);
                }
                substrait::rel::RelType::Project(project_rel) => {
                    self.visit_project_rel(project_rel);
                }
                substrait::rel::RelType::Set(set_rel) => {
                    self.visit_set_rel(set_rel);
                }
                // Handle other relation types as needed
                _ => {}
            }
        }
    }

    fn visit_read_rel(&mut self, read_rel: &substrait::ReadRel) -> impl std::any::Any {
        // Process common relation elements first
        if let Some(common) = &read_rel.common {
            self.visit_rel_common(common);
        }

        // Process read source
        if let Some(read_type) = &read_rel.read_type {
            match read_type {
                substrait::read_rel::ReadType::VirtualTable(table) => {
                    // Process virtual table data
                    for row in &table.values {
                        for item in &row.fields {
                            self.visit_literal(item);
                        }
                    }
                    // MEGAHACK -- Add support for expressions
                }
                substrait::read_rel::ReadType::NamedTable(table) => {
                    // Register the table in the symbol table
                    if let Some(scope) = &self.current_relation_scope {
                        self.symbol_table.add_named_table(scope, &table.names);
                    }
                }
                substrait::read_rel::ReadType::LocalFiles(files) => {
                    // Process file items
                    for item in &files.items {
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
                    }
                }
                substrait::read_rel::ReadType::ExtensionTable(_) => {
                    // Process extension table as needed
                }
                substrait::read_rel::ReadType::IcebergTable(_) => {}
            }
        }
    }

    fn visit_filter_rel(&mut self, filter_rel: &substrait::FilterRel) -> impl std::any::Any {
        // Process common relation elements
        if let Some(common) = &filter_rel.common {
            self.visit_rel_common(common);
        }

        // Process filter condition
        if let Some(condition) = &filter_rel.condition {
            self.visit_expression(condition);
        }

        // Process input relation
        if let Some(input) = &filter_rel.input {
            // Save current scope
            let old_scope = self.current_relation_scope.clone();

            // Process input relation with current scope
            self.visit_rel(input);

            // Restore scope
            self.current_relation_scope = old_scope;
        }
    }

    fn visit_fetch_rel(&mut self, fetch_rel: &substrait::FetchRel) -> impl std::any::Any {
        // Process common relation elements
        if let Some(common) = &fetch_rel.common {
            self.visit_rel_common(common);
        }

        // Process input relation
        if let Some(input) = &fetch_rel.input {
            // Save current scope
            let old_scope = self.current_relation_scope.clone();

            // Process input relation
            self.visit_rel(input);

            // Restore scope
            self.current_relation_scope = old_scope;
        }
    }

    fn visit_aggregate_rel(
        &mut self,
        aggregate_rel: &substrait::AggregateRel,
    ) -> impl std::any::Any {
        // Process common relation elements
        if let Some(common) = &aggregate_rel.common {
            self.visit_rel_common(common);
        }

        // Process input relation
        if let Some(input) = &aggregate_rel.input {
            // Save current scope
            let old_scope = self.current_relation_scope.clone();

            // Process input relation
            self.visit_rel(input);

            // Restore scope
            self.current_relation_scope = old_scope;
        }

        // Process groupings
        for grouping in &aggregate_rel.groupings {
            // Process grouping keys
            for key in &grouping.grouping_expressions {
                self.visit_expression(key);
            }
        }

        // Process measures
        for measure in &aggregate_rel.measures {
            // Process measure function
            if let Some(function) = &measure.measure {
                self.visit_aggregate_function(function);
            }
        }
    }

    fn visit_sort_rel(&mut self, sort_rel: &substrait::SortRel) -> impl std::any::Any {
        // Process common relation elements
        if let Some(common) = &sort_rel.common {
            self.visit_rel_common(common);
        }

        // Process input relation
        if let Some(input) = &sort_rel.input {
            // Save current scope
            let old_scope = self.current_relation_scope.clone();

            // Process input relation
            self.visit_rel(input);

            // Restore scope
            self.current_relation_scope = old_scope;
        }

        // Process sort fields
        for sort_field in &sort_rel.sorts {
            // Process sort expression
            if let Some(expr) = &sort_field.expr {
                self.visit_expression(expr);
            }
        }
    }

    fn visit_join_rel(&mut self, join_rel: &substrait::JoinRel) -> impl std::any::Any {
        // Process common relation elements
        if let Some(common) = &join_rel.common {
            self.visit_rel_common(common);
        }

        // Process left input relation
        if let Some(left) = &join_rel.left {
            // Save current scope
            let old_scope = self.current_relation_scope.clone();

            // Process left relation
            self.visit_rel(left);

            // Restore scope
            self.current_relation_scope = old_scope;
        }

        // Process right input relation
        if let Some(right) = &join_rel.right {
            // Save current scope
            let old_scope = self.current_relation_scope.clone();

            // Process right relation
            self.visit_rel(right);

            // Restore scope
            self.current_relation_scope = old_scope;
        }

        // Process expression
        if let Some(expression) = &join_rel.expression {
            self.visit_expression(expression);
        }
    }

    fn visit_project_rel(&mut self, project_rel: &substrait::ProjectRel) -> impl std::any::Any {
        // Process common relation elements
        if let Some(common) = &project_rel.common {
            self.visit_rel_common(common);
        }

        // Process input relation
        if let Some(input) = &project_rel.input {
            // Save current scope
            let old_scope = self.current_relation_scope.clone();

            // Process input relation
            self.visit_rel(input);

            // Restore scope
            self.current_relation_scope = old_scope;
        }

        // Process expressions
        for expression in &project_rel.expressions {
            self.visit_expression(expression);
        }
    }

    fn visit_set_rel(&mut self, set_rel: &substrait::SetRel) -> impl std::any::Any {
        // Process common relation elements
        if let Some(common) = &set_rel.common {
            self.visit_rel_common(common);
        }

        // Process input relations
        for input in &set_rel.inputs {
            // Save current scope
            let old_scope = self.current_relation_scope.clone();

            // Process input relation
            self.visit_rel(input);

            // Restore scope
            self.current_relation_scope = old_scope;
        }
    }

    fn visit_rel_common(&mut self, common: &substrait::RelCommon) -> impl std::any::Any {
        // Process the emit pattern if present
        if let Some(emit_kind) = &common.emit_kind {
            match emit_kind {
                EmitKind::Direct(_) => {
                    // Nothing to do here.
                }
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
    }

    fn visit_expression(&mut self, expression: &substrait::Expression) -> impl std::any::Any {
        // Process expressions based on their type
        if let Some(rex_type) = &expression.rex_type {
            match rex_type {
                substrait::expression::RexType::Literal(literal) => {
                    self.visit_literal(literal);
                }
                substrait::expression::RexType::ScalarFunction(function) => {
                    self.visit_scalar_function(function);
                }
                substrait::expression::RexType::WindowFunction(function) => {
                    self.visit_window_function(function);
                }
                substrait::expression::RexType::IfThen(if_then) => {
                    self.visit_if_then(if_then);
                }
                substrait::expression::RexType::SwitchExpression(switch) => {
                    self.visit_switch_expression(switch);
                }
                substrait::expression::RexType::SingularOrList(list) => {
                    self.visit_singular_or_list(list);
                }
                substrait::expression::RexType::MultiOrList(list) => {
                    self.visit_multi_or_list(list);
                }
                substrait::expression::RexType::Cast(cast) => {
                    self.visit_cast(cast);
                }
                substrait::expression::RexType::Subquery(subquery) => {
                    self.visit_subquery(subquery);
                }
                substrait::expression::RexType::Selection(reference) => {
                    self.visit_field_reference(reference);
                }
                // Handle other expression types as needed
                _ => {}
            }
        }
    }

    fn visit_literal(&mut self, literal: &substrait::expression::Literal) -> impl std::any::Any {
        // Process literal based on its type
        if let Some(literal_type) = &literal.literal_type {
            match literal_type {
                substrait::expression::literal::LiteralType::String(s) => {
                    // Register string literals in the symbol table if needed
                    if let Some(scope) = &self.current_relation_scope {
                        self.symbol_table.add_string_literal(scope, s);
                    }
                }
                substrait::expression::literal::LiteralType::Struct(s) => {
                    // Process field expressions
                    for field in &s.fields {
                        self.visit_literal(field);
                    }
                }
                substrait::expression::literal::LiteralType::Map(m) => {
                    // Process key-value pairs
                    for kv in &m.key_values {
                        if let Some(key) = &kv.key {
                            self.visit_literal(key);
                        }

                        if let Some(value) = &kv.value {
                            self.visit_literal(value);
                        }
                    }
                }
                substrait::expression::literal::LiteralType::List(l) => {
                    // Process list values
                    for value in &l.values {
                        self.visit_literal(value);
                    }
                }
                _ => {}
            }
        }
    }

    fn visit_scalar_function(
        &mut self,
        function: &substrait::expression::ScalarFunction,
    ) -> impl std::any::Any {
        // Process function arguments
        for arg in &function.arguments {
            if let Some(arg_type) = &arg.arg_type {
                match arg_type {
                    substrait::function_argument::ArgType::Value(expr) => {
                        self.visit_expression(expr);
                    }
                    substrait::function_argument::ArgType::Type(ty) => {
                        self.visit_type(ty);
                    }
                    _ => {}
                }
            }
        }
    }

    fn visit_window_function(
        &mut self,
        function: &substrait::expression::WindowFunction,
    ) -> impl std::any::Any {
        // Process function arguments
        for arg in &function.arguments {
            if let Some(arg_type) = &arg.arg_type {
                match arg_type {
                    substrait::function_argument::ArgType::Value(expr) => {
                        self.visit_expression(expr);
                    }
                    substrait::function_argument::ArgType::Type(ty) => {
                        self.visit_type(ty);
                    }
                    _ => {}
                }
            }
        }

        // Process partitioning expressions
        for expr in &function.partitions {
            self.visit_expression(expr);
        }

        // Process sort fields
        for sort in &function.sorts {
            if let Some(expr) = &sort.expr {
                self.visit_expression(expr);
            }
        }
    }

    fn visit_if_then(&mut self, if_then: &substrait::expression::IfThen) -> impl std::any::Any {
        // Process if clauses
        for clause in &if_then.ifs {
            if let Some(condition) = &clause.r#if {
                self.visit_expression(condition);
            }

            if let Some(then) = &clause.then {
                self.visit_expression(then);
            }
        }

        // Process else expression
        if let Some(else_expr) = &if_then.r#else {
            self.visit_expression(else_expr);
        }
    }

    fn visit_switch_expression(
        &mut self,
        switch: &substrait::expression::SwitchExpression,
    ) -> impl std::any::Any {
        // Process match expression
        if let Some(match_expr) = &switch.r#match {
            self.visit_expression(match_expr);
        }

        // Process if-value cases
        for if_value in &switch.ifs {
            if let Some(if_expr) = &if_value.r#if {
                self.visit_literal(if_expr);
            }

            if let Some(then) = &if_value.then {
                self.visit_expression(then);
            }
        }

        // Process else expression
        if let Some(else_expr) = &switch.r#else {
            self.visit_expression(else_expr);
        }
    }

    fn visit_singular_or_list(
        &mut self,
        list: &substrait::expression::SingularOrList,
    ) -> impl std::any::Any {
        // Process value expression
        if let Some(value) = &list.value {
            self.visit_expression(value);
        }

        // Process options
        for option in &list.options {
            self.visit_expression(option);
        }
    }

    fn visit_multi_or_list(
        &mut self,
        list: &substrait::expression::MultiOrList,
    ) -> impl std::any::Any {
        // Process value expressions
        for value in &list.value {
            self.visit_expression(value);
        }

        // Process record items
        for record in &list.options {
            // Process record values
            for value in &record.fields {
                self.visit_expression(value);
            }
        }
    }

    fn visit_cast(&mut self, cast: &substrait::expression::Cast) -> impl std::any::Any {
        // Process input expression
        if let Some(expr) = &cast.input {
            self.visit_expression(expr);
        }

        // Process type
        if let Some(ty) = &cast.r#type {
            self.visit_type(ty);
        }
    }

    fn visit_subquery(&mut self, subquery: &substrait::expression::Subquery) -> impl std::any::Any {
        // Process subquery based on type
        if let Some(subquery_type) = &subquery.subquery_type {
            match subquery_type {
                substrait::expression::subquery::SubqueryType::Scalar(scalar) => {
                    // Process input relation
                    if let Some(input) = &scalar.input {
                        // Save current scope
                        let old_scope = self.current_relation_scope.clone();

                        // Process subquery relation
                        self.visit_rel(input);

                        // Restore scope
                        self.current_relation_scope = old_scope;
                    }
                }
                substrait::expression::subquery::SubqueryType::InPredicate(predicate) => {
                    // Process haystack relation
                    if let Some(haystack) = &predicate.haystack {
                        // Save current scope
                        let old_scope = self.current_relation_scope.clone();

                        // Process haystack relation
                        self.visit_rel(haystack);

                        // Restore scope
                        self.current_relation_scope = old_scope;
                    }

                    // Process needles
                    for needle in &predicate.needles {
                        self.visit_expression(needle);
                    }
                }
                substrait::expression::subquery::SubqueryType::SetPredicate(predicate) => {
                    // Process tuples relation
                    if let Some(tuples) = &predicate.tuples {
                        // Save current scope
                        let old_scope = self.current_relation_scope.clone();

                        // Process tuples relation
                        self.visit_rel(tuples);

                        // Restore scope
                        self.current_relation_scope = old_scope;
                    }
                }
                substrait::expression::subquery::SubqueryType::SetComparison(comparison) => {
                    // Process left expressions
                    if let Some(expr) = &comparison.left {
                        self.visit_expression(expr);
                    }

                    // Process right relation
                    if let Some(right) = &comparison.right {
                        // Save current scope
                        let old_scope = self.current_relation_scope.clone();

                        // Process right relation
                        self.visit_rel(right);

                        // Restore scope
                        self.current_relation_scope = old_scope;
                    }
                }
            }
        }
    }

    fn visit_field_reference(
        &mut self,
        _reference: &substrait::expression::FieldReference,
    ) -> impl std::any::Any {
        // Simply record the field reference, no complex processing needed here
    }

    fn visit_type(&mut self, ty: &substrait::Type) -> impl std::any::Any {
        // Process type based on its kind
        if let Some(kind) = &ty.kind {
            match kind {
                substrait::r#type::Kind::Struct(structure) => {
                    // Process struct types
                    for field in &structure.types {
                        self.visit_type(field);
                    }
                }
                substrait::r#type::Kind::List(list) => {
                    // Process list type
                    if let Some(ty) = &list.r#type {
                        self.visit_type(ty);
                    }
                }
                substrait::r#type::Kind::Map(map) => {
                    // Process key and value types
                    if let Some(key) = &map.key {
                        self.visit_type(key);
                    }

                    if let Some(value) = &map.value {
                        self.visit_type(value);
                    }
                }
                _ => {}
            }
        }
    }

    fn visit_aggregate_function(
        &mut self,
        function: &substrait::AggregateFunction,
    ) -> impl std::any::Any {
        // Process function arguments
        for arg in &function.arguments {
            if let Some(arg_type) = &arg.arg_type {
                match arg_type {
                    substrait::function_argument::ArgType::Value(expr) => {
                        self.visit_expression(expr);
                    }
                    substrait::function_argument::ArgType::Type(ty) => {
                        self.visit_type(ty);
                    }
                    _ => {}
                }
            }
        }

        // Process sort fields
        for sort in &function.sorts {
            if let Some(expr) = &sort.expr {
                self.visit_expression(expr);
            }
        }
    }
}
