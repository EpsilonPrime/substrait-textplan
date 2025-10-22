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
use crate::textplan::symbol_table::SourceType;
use crate::textplan::{ProtoLocation, SymbolInfo, SymbolType};
use ::substrait::proto as substrait;
use std::any::Any;
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::Mutex;

const ROOT_NAMES: &str = "root.names";

/// Initial visitor implementation that builds a symbol table from a Substrait plan.
///
/// This visitor is used to populate a symbol table with relationships between plan elements
/// during the first traversal of the plan.
pub struct InitialPlanVisitor {
    /// Symbol table for storing plan element information
    symbol_table: crate::textplan::symbol_table::SymbolTable,

    /// Current relation context for scope resolution
    current_relation_scope: Vec<Arc<String>>,

    internal_location: ProtoLocation,

    read_relation_sources: HashMap<String, Arc<SymbolInfo>>,
    read_relation_schemas: HashMap<String, Arc<SymbolInfo>>,
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

fn rel_type_case_name(relation: &substrait::Rel) -> &'static str {
    if let Some(oneof) = &relation.rel_type {
        match oneof {
            substrait::rel::RelType::Read(_) => "rel",
            substrait::rel::RelType::Filter(_) => "filter",
            substrait::rel::RelType::Fetch(_) => "root",
            substrait::rel::RelType::Aggregate(_) => "aggregate",
            substrait::rel::RelType::Sort(_) => "sort",
            substrait::rel::RelType::Join(_) => "join",
            substrait::rel::RelType::Project(_) => "project",
            substrait::rel::RelType::Set(_) => "set",
            substrait::rel::RelType::ExtensionSingle(_) => "extension_single",
            substrait::rel::RelType::ExtensionMulti(_) => "extension_multi",
            substrait::rel::RelType::ExtensionLeaf(_) => "extension_leaf",
            substrait::rel::RelType::Cross(_) => "cross",
            substrait::rel::RelType::Reference(_) => "reference",
            substrait::rel::RelType::Write(_) => "write",
            substrait::rel::RelType::Ddl(_) => "ddl",
            substrait::rel::RelType::Update(_) => "update",
            substrait::rel::RelType::HashJoin(_) => "hash_join",
            substrait::rel::RelType::MergeJoin(_) => "merge_join",
            substrait::rel::RelType::NestedLoopJoin(_) => "nested_loop_join",
            substrait::rel::RelType::Window(_) => "window",
            substrait::rel::RelType::Exchange(_) => "exchange",
            substrait::rel::RelType::Expand(_) => "expand",
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
            current_relation_scope: Vec::new(),
            internal_location: ProtoLocation::default(),
            read_relation_sources: HashMap::new(),
            read_relation_schemas: HashMap::new(),
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

    fn pre_process_rel(&mut self, obj: &substrait::Rel) {
        self.current_relation_scope
            .push(Arc::new(self.current_location().path_string()));
    }

    fn post_process_rel(&mut self, obj: &substrait::Rel) {
        let name = rel_type_case_name(obj);

        // Create relation data to store with the symbol.
        let mut relation_data = RelationData::new(obj.clone());

        // Update the relation data for long term use.
        // TODO: updateLocalSchema(relationData, relation, relationData->relation);

        if let Some(scope) = self.current_relation_scope.last() {
            let scope_str = scope.as_ref().clone();
            if let Some(source) = self.read_relation_sources.get(&scope_str) {
                relation_data.source = Some(source.clone());
            }
            if let Some(schema) = self.read_relation_schemas.get(&scope_str) {
                relation_data.schema = Some(schema.clone());
            }
        }

        // Finally create our entry in the symbol table.
        let unique_name = self.symbol_table.get_unique_name(name);
        self.symbol_table.define_symbol(
            unique_name,
            self.current_location().clone(),
            SymbolType::Relation,
            None,
            Some(Arc::new(Mutex::new(relation_data))),
        );

        self.current_relation_scope.pop();
    }

    fn post_process_rel_root(&mut self, obj: &substrait::RelRoot) {
        let mut names = Vec::new();
        names.extend(obj.names.iter().cloned());

        let unique_name = self.symbol_table.get_unique_name(ROOT_NAMES);
        self.symbol_table.define_symbol(
            unique_name,
            self.current_location().field("rel_root"),
            SymbolType::Root,
            Some(Box::new(SourceType::Unknown)),
            Some(Arc::new(Mutex::new(names))),
        );
    }

    fn pre_process_read_rel(&mut self, obj: &substrait::ReadRel) {
        if obj.base_schema.is_some() {
            let name = self.symbol_table.get_unique_name("schema");
            let symbol = self.symbol_table.define_symbol(
                name,
                self.current_location().field("base_schema"),
                SymbolType::Schema,
                None,
                Some(Arc::new(Mutex::new(obj.base_schema.clone().unwrap()))),
            );
            self.read_relation_schemas.insert(
                self.current_relation_scope.last().unwrap().to_string(),
                symbol,
            );
            //visit_named_struct(&obj.base_schema);
        }
    }
}
