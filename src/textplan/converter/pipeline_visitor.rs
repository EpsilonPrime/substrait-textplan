// SPDX-License-Identifier: Apache-2.0

//! Plan visitor implementation for traversing Substrait plans.
//!
//! This module provides implementations of visitors for processing Substrait plans.
//! It builds on the generated BasePlanProtoVisitor trait to provide specialized
//! visitors for different stages of plan processing.

use crate::textplan::common::structured_symbol_data::RelationData;
use crate::textplan::common::ProtoLocation;
use crate::textplan::converter::generated::base_plan_visitor::Traversable;
use crate::textplan::converter::generated::PlanProtoVisitor;
use crate::textplan::SymbolType;
use ::substrait::proto as substrait;

/// Pipeline visitor implementation that processes a Substrait plan in multiple stages.
///
/// This visitor builds on the initial visitor and provides more advanced processing
/// of Substrait plans. It can be composed into a chain of visitors, each handling
/// a different aspect of plan processing.
pub struct PipelineVisitor {
    /// Symbol table for storing plan element information
    symbol_table: crate::textplan::symbol_table::SymbolTable,
    /// Current relation context for scope resolution
    current_relation_scope: Option<String>,
    /// Current location in the protocol buffer structure
    current_location: ProtoLocation,
}

impl PipelineVisitor {
    /// Create a new pipeline visitor with the given symbol table
    pub fn new(symbol_table: crate::textplan::symbol_table::SymbolTable) -> Self {
        Self {
            symbol_table,
            current_relation_scope: None,
            current_location: ProtoLocation::default(),
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

    pub fn visit_extended_expression(&mut self, obj: &substrait::ExtendedExpression) {
        obj.traverse(self);
    }

    pub fn visit_plan(&mut self, obj: &substrait::Plan) {
        obj.traverse(self);
    }
}

impl PlanProtoVisitor for PipelineVisitor {
    fn current_location(&self) -> &ProtoLocation {
        &self.current_location
    }

    fn set_location(&mut self, location: ProtoLocation) {
        self.current_location = location;
    }

    fn post_process_expression(&mut self, expression: &substrait::Expression) {
        if let Some(rex_type) = &expression.rex_type {
            match rex_type {
                substrait::expression::RexType::Subquery(_) => {
                    /*
                    auto result = visitSubquery(expression.subquery());

                    const ::substrait::proto::Rel* subqueryRelation;
                    switch (expression.subquery().subquery_type_case()) {
                        case ::substrait::proto::Expression_Subquery::kScalar:
                            subqueryRelation = &expression.subquery().scalar().input();
                        break;
                        case ::substrait::proto::Expression_Subquery::kInPredicate:
                            subqueryRelation = &expression.subquery().in_predicate().haystack();
                        break;
                        case ::substrait::proto::Expression_Subquery::kSetPredicate:
                            subqueryRelation = &expression.subquery().set_predicate().tuples();
                        break;
                        case ::substrait::proto::Expression_Subquery::kSetComparison:
                            subqueryRelation = &expression.subquery().set_comparison().right();
                        break;
                        case ::substrait::proto::Expression_Subquery::SUBQUERY_TYPE_NOT_SET:
                        // No need to raise as this would have been exposed earlier.
                        return result;
                    }
                    if (subqueryRelation == nullptr) {
                        // No need to raise as this would have been caught earlier.
                        return result;
                    }

                    auto subquerySymbol = symbolTable_->lookupSymbolByLocationAndType(
                        PROTO_LOCATION(*subqueryRelation), SymbolType::kRelation);
                    auto currentRelationData =
                        ANY_CAST(std::shared_ptr<RelationData>, currentRelationScope_->blob);
                    currentRelationData->subQueryPipelines.push_back(subquerySymbol);

                    // Populate the start of the pipeline for easy later access.
                    const SymbolInfo* current;
                    auto thisRelationData =
                        ANY_CAST(std::shared_ptr<RelationData>, subquerySymbol->blob);
                    thisRelationData->pipelineStart = subquerySymbol;
                    while (thisRelationData->continuingPipeline != nullptr) {
                        current = thisRelationData->continuingPipeline;
                        thisRelationData = ANY_CAST(std::shared_ptr<RelationData>, current->blob);
                        thisRelationData->pipelineStart = subquerySymbol;
                    }
                    return result;
                    */
                }
                _ => {}
            }
        }
    }

    fn post_process_rel(&mut self, rel: &substrait::Rel) {
        todo!()
    }

    fn post_process_plan_rel(&mut self, relation: &substrait::PlanRel) {
        let symbols = self
            .symbol_table()
            .lookup_symbols_by_location(self.current_location());
        // A symbol is guaranteed as we previously visited the parse tree.
        symbols[0].with_blob::<RelationData, _, _>(|relation_data| match relation.rel_type {
            Some(substrait::plan_rel::RelType::Rel(_)) => {
                let rel_symbol = self.symbol_table.lookup_symbol_by_location_and_type(
                    &self.current_location().field("rel"),
                    SymbolType::Relation,
                );
                relation_data.new_pipelines.push(rel_symbol.unwrap());
            }
            Some(substrait::plan_rel::RelType::Root(_)) => {
                let input_symbol = self.symbol_table.lookup_symbol_by_location_and_type(
                    &self.current_location().field("rotation"),
                    SymbolType::Relation,
                );
                relation_data.new_pipelines.push(input_symbol.unwrap());
            }
            None => {}
        });
    }
}
