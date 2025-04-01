// SPDX-License-Identifier: Apache-2.0

//! Plan visitor implementation for traversing Substrait plans.
//!
//! This module provides implementations of visitors for processing Substrait plans.
//! It builds on the generated BasePlanProtoVisitor trait to provide specialized
//! visitors for different stages of plan processing.

use crate::textplan::common::error::TextPlanError;
use crate::textplan::common::structured_symbol_data::ExtensionSpaceData;
use crate::textplan::common::structured_symbol_data::FunctionData;
use crate::textplan::common::structured_symbol_data::RelationData;
use crate::textplan::converter::generated::PlanProtoVisitor;
use crate::textplan::converter::generated::Traversable;
use crate::textplan::{ProtoLocation, SymbolType};
use std::any::Any;
use std::sync::Arc;
use std::sync::Mutex;
use ::substrait::proto as substrait;

/// Initial visitor implementation that builds a symbol table from a Substrait plan.
///
/// This visitor is used to populate a symbol table with relationships between plan elements
/// during the first traversal of the plan.
pub struct InitialPlanVisitor {
    /// Symbol table for storing plan element information
    symbol_table: crate::textplan::symbol_table::SymbolTable,
    /// Current relation context for scope resolution
    current_relation_scope: Option<String>,

    internal_location: ProtoLocation,
}

fn short_name(s: &str) -> &str {
    match s.find(':') {
        Some(index) => &s[0..index],
        None => s,
    }
}

fn plan_rel_type_case_name(obj: &substrait::PlanRel) -> &'static str {
    if let Some(oneof) = &obj.rel_type {
        match oneof {
            substrait::plan_rel::RelType::Rel(_) => "rel",
            substrait::plan_rel::RelType::Root(_) => "root",
        }
    } else {
        "unknown"
    }
}

impl InitialPlanVisitor {
    /// Create a new initial visitor with the given symbol table
    pub fn new(symbol_table: crate::textplan::symbol_table::SymbolTable) -> Self {
        Self {
            symbol_table,
            current_relation_scope: None,
            internal_location: ProtoLocation::default(),
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

impl PlanProtoVisitor for InitialPlanVisitor {
    fn current_location(&self) -> &ProtoLocation {
        &self.internal_location
    }

    fn set_location(&mut self, location: ProtoLocation) {
        self.internal_location = location;
    }

    fn post_process_simple_extension_uri(
        &mut self,
        obj: &substrait::extensions::SimpleExtensionUri,
    ) {
        self.symbol_table.define_symbol(
            obj.uri.clone(),
            self.current_location().field("uri"),
            SymbolType::ExtensionSpace,
            /* subtype */ None,
            Some(Arc::new(Mutex::new(ExtensionSpaceData::new(
                obj.extension_uri_anchor.clone(),
            ))) as Arc<Mutex<dyn Any + Send + Sync>>),
        );
    }

    fn post_process_simple_extension_declaration(
        &mut self,
        obj: &substrait::extensions::SimpleExtensionDeclaration,
    ) {
        if let Some(mapping_type) = &obj.mapping_type {
            match mapping_type {
                substrait::extensions::simple_extension_declaration::MappingType::ExtensionFunction(ef) => {
                    let unique_name = self.symbol_table.get_unique_name(short_name(&ef.name));

                    self.symbol_table.define_symbol(unique_name,
                                                    self.current_location().field("extension_function"),
                                                    SymbolType::Function,
                                                    /* subtype */ None,
                                                    Some(Arc::new(Mutex::new(FunctionData::new(
                                                        ef.name.clone(),
                                                        Some(ef.extension_uri_reference.clone()),
                                                        ef.function_anchor.clone()),
                                                    )) as Arc<Mutex<dyn Any + Send + Sync>>)
                    );
                }
                _ => {
                    panic!("Unknown mapping type case {:#?} encountered.",
                           &obj.mapping_type);
                }
            }
        }
    }

    fn pre_process_plan_rel(&mut self, obj: &substrait::PlanRel) {
        let name = plan_rel_type_case_name(obj);
        let unique_name = self.symbol_table.get_unique_name(name);

        self.symbol_table.define_symbol(
            unique_name,
            self.current_location().clone(),
            SymbolType::PlanRelation,
            /* subtype */ None,
            Some(Arc::new(Mutex::new(RelationData::new(
                ::substrait::proto::Rel::default(),
            ))) as Arc<Mutex<dyn Any + Send + Sync>>),
        );
    }

    fn post_process_expression(&mut self, expression: &substrait::Expression) {
        if let Some(rex_type) = &expression.rex_type {
            match rex_type {
                substrait::expression::RexType::Subquery(_) => {
                    /*
                       outerRelations_.push_back(currentRelationScope_);
                    auto resetRelationScope = finally([&]() { outerRelations_.pop_back(); });

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
                        errorListener_->addError("Subquery type not set.");
                        return result;
                    }
                    if (subqueryRelation == nullptr) {
                      errorListener_->addError("Unrecognized subquery type.");
                      return result;
                    }

                    const SymbolInfo* symbol = symbolTable_->lookupSymbolByLocationAndType(
                        PROTO_LOCATION(*subqueryRelation), SymbolType::kRelation);
                    symbolTable_->setParentQueryLocation(
                        *symbol, PROTO_LOCATION(*currentRelationScope_));

                    return result;
                                     */
                }
                _ => {}
            }
        }
    }
}
