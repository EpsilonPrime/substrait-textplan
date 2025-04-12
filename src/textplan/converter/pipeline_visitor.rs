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
use scopeguard::guard;
use std::sync::Arc;
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
    current_relation_scope: Option<Arc<crate::textplan::SymbolInfo>>,
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

    fn post_process_rel(&mut self, rel: &substrait::Rel) {
        let symbol = self
            .symbol_table()
            .lookup_symbol_by_location_and_type(self.current_location(), SymbolType::Relation);

        let previous_relation_scope = self.current_relation_scope.clone();
        self.current_relation_scope = symbol.clone();
        let _reset_scope = guard((), |_| {
            self.current_relation_scope = previous_relation_scope;
        });

        symbol
            .unwrap()
            .with_blob::<RelationData, _, _>(|relation_data| match &rel.rel_type {
                Some(substrait::rel::RelType::Read(_)) => {
                    // No relations beyond this one.
                }
                Some(substrait::rel::RelType::Filter(_)) => {
                    let rel_symbol = self.symbol_table.lookup_symbol_by_location_and_type(
                        &self.current_location().field("filter").field("input"),
                        SymbolType::Relation,
                    );
                    relation_data.continuing_pipeline = rel_symbol;
                }
                Some(substrait::rel::RelType::Fetch(_)) => {
                    let rel_symbol = self.symbol_table.lookup_symbol_by_location_and_type(
                        &self.current_location().field("fetch").field("input"),
                        SymbolType::Relation,
                    );
                    relation_data.continuing_pipeline = rel_symbol;
                }
                Some(substrait::rel::RelType::Aggregate(_)) => {
                    let rel_symbol = self.symbol_table.lookup_symbol_by_location_and_type(
                        &self.current_location().field("aggregate").field("input"),
                        SymbolType::Relation,
                    );
                    relation_data.continuing_pipeline = rel_symbol;
                }
                Some(substrait::rel::RelType::Sort(_)) => {
                    let rel_symbol = self.symbol_table.lookup_symbol_by_location_and_type(
                        &self.current_location().field("sort").field("input"),
                        SymbolType::Relation,
                    );
                    relation_data.continuing_pipeline = rel_symbol;
                }
                Some(substrait::rel::RelType::Join(_)) => {
                    let left_symbol = self.symbol_table.lookup_symbol_by_location_and_type(
                        &self.current_location().field("join").field("left"),
                        SymbolType::Relation,
                    );
                    let right_symbol = self.symbol_table.lookup_symbol_by_location_and_type(
                        &self.current_location().field("join").field("right"),
                        SymbolType::Relation,
                    );
                    relation_data.new_pipelines.push(left_symbol.unwrap());
                    relation_data.new_pipelines.push(right_symbol.unwrap());
                }
                Some(substrait::rel::RelType::Project(_)) => {
                    let rel_symbol = self.symbol_table.lookup_symbol_by_location_and_type(
                        &self.current_location().field("project").field("input"),
                        SymbolType::Relation,
                    );
                    relation_data.continuing_pipeline = rel_symbol;
                }
                Some(substrait::rel::RelType::Set(set)) => {
                    for i in 0..set.inputs.len() {
                        let input_symbol = self.symbol_table.lookup_symbol_by_location_and_type(
                            &self
                                .current_location()
                                .field("set")
                                .indexed_field("inputs", i),
                            SymbolType::Relation,
                        );
                        relation_data.new_pipelines.push(input_symbol.unwrap());
                    }
                }
                Some(substrait::rel::RelType::ExtensionSingle(_)) => {
                    let rel_symbol = self.symbol_table.lookup_symbol_by_location_and_type(
                        &self
                            .current_location()
                            .field("extension_single")
                            .field("input"),
                        SymbolType::Relation,
                    );
                    relation_data.continuing_pipeline = rel_symbol;
                }
                Some(substrait::rel::RelType::ExtensionMulti(multi)) => {
                    for i in 0..multi.inputs.len() {
                        let input_symbol = self.symbol_table.lookup_symbol_by_location_and_type(
                            &self
                                .current_location()
                                .field("extension_multi")
                                .indexed_field("inputs", i),
                            SymbolType::Relation,
                        );
                        relation_data.new_pipelines.push(input_symbol.unwrap());
                    }
                }
                Some(substrait::rel::RelType::ExtensionLeaf(_)) => {
                    // No children.
                }
                Some(substrait::rel::RelType::Cross(_)) => {
                    let left_symbol = self.symbol_table.lookup_symbol_by_location_and_type(
                        &self.current_location().field("cross").field("left"),
                        SymbolType::Relation,
                    );
                    let right_symbol = self.symbol_table.lookup_symbol_by_location_and_type(
                        &self.current_location().field("cross").field("right"),
                        SymbolType::Relation,
                    );
                    relation_data.new_pipelines.push(left_symbol.unwrap());
                    relation_data.new_pipelines.push(right_symbol.unwrap());
                }
                Some(substrait::rel::RelType::Reference(_)) => {
                    // TODO -- Add support for references in text plans.
                    todo!()
                }
                Some(substrait::rel::RelType::Write(_)) => {
                    let rel_symbol = self.symbol_table.lookup_symbol_by_location_and_type(
                        &self.current_location().field("write").field("input"),
                        SymbolType::Relation,
                    );
                    relation_data.continuing_pipeline = rel_symbol;
                }
                Some(substrait::rel::RelType::Ddl(_)) => {
                    // TODO -- Add support for DDL in text plans.
                    todo!()
                }
                Some(substrait::rel::RelType::Update(_)) => {
                    // TODO -- Add support for update in text plans.
                    todo!()
                }
                Some(substrait::rel::RelType::HashJoin(_)) => {
                    let left_symbol = self.symbol_table.lookup_symbol_by_location_and_type(
                        &self.current_location().field("hash_join").field("left"),
                        SymbolType::Relation,
                    );
                    let right_symbol = self.symbol_table.lookup_symbol_by_location_and_type(
                        &self.current_location().field("hash_join").field("right"),
                        SymbolType::Relation,
                    );
                    relation_data.new_pipelines.push(left_symbol.unwrap());
                    relation_data.new_pipelines.push(right_symbol.unwrap());
                }
                Some(substrait::rel::RelType::MergeJoin(_)) => {
                    let left_symbol = self.symbol_table.lookup_symbol_by_location_and_type(
                        &self.current_location().field("merge_join").field("left"),
                        SymbolType::Relation,
                    );
                    let right_symbol = self.symbol_table.lookup_symbol_by_location_and_type(
                        &self.current_location().field("merge_join").field("right"),
                        SymbolType::Relation,
                    );
                    relation_data.new_pipelines.push(left_symbol.unwrap());
                    relation_data.new_pipelines.push(right_symbol.unwrap());
                }
                Some(substrait::rel::RelType::NestedLoopJoin(_)) => {
                    let left_symbol = self.symbol_table.lookup_symbol_by_location_and_type(
                        &self
                            .current_location()
                            .field("nested_loop_join")
                            .field("left"),
                        SymbolType::Relation,
                    );
                    let right_symbol = self.symbol_table.lookup_symbol_by_location_and_type(
                        &self
                            .current_location()
                            .field("nested_loop_join")
                            .field("right"),
                        SymbolType::Relation,
                    );
                    relation_data.new_pipelines.push(left_symbol.unwrap());
                    relation_data.new_pipelines.push(right_symbol.unwrap());
                }
                Some(substrait::rel::RelType::Window(_)) => {
                    let rel_symbol = self.symbol_table.lookup_symbol_by_location_and_type(
                        &self.current_location().field("window").field("input"),
                        SymbolType::Relation,
                    );
                    relation_data.continuing_pipeline = rel_symbol;
                }
                Some(substrait::rel::RelType::Exchange(_)) => {
                    let rel_symbol = self.symbol_table.lookup_symbol_by_location_and_type(
                        &self.current_location().field("exchange").field("input"),
                        SymbolType::Relation,
                    );
                    relation_data.continuing_pipeline = rel_symbol;
                }
                Some(substrait::rel::RelType::Expand(_)) => {
                    let rel_symbol = self.symbol_table.lookup_symbol_by_location_and_type(
                        &self.current_location().field("expand").field("input"),
                        SymbolType::Relation,
                    );
                    relation_data.continuing_pipeline = rel_symbol;
                }
                None => {}
            });
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
}
