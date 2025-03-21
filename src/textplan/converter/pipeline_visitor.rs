// SPDX-License-Identifier: Apache-2.0

//! Plan visitor implementation for traversing Substrait plans.
//!
//! This module provides implementations of visitors for processing Substrait plans.
//! It builds on the generated BasePlanProtoVisitor trait to provide specialized
//! visitors for different stages of plan processing.

use crate::textplan::converter::generated::BasePlanProtoVisitor;
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
    fn visit_plan(&mut self, plan: &substrait::Plan) -> impl std::any::Any {
        // Process each relation in the plan
        for relation in &plan.relations {
            if let Some(rel_type) = &relation.rel_type {
                match rel_type {
                    substrait::plan_rel::RelType::Rel(rel) => {
                        // Process regular relations
                        //self.visit_rel(rel)?;
                    }
                    substrait::plan_rel::RelType::Root(_) => {
                        // Root relations are handled by the initial visitor
                    }
                }
            }
        }
    }
}
