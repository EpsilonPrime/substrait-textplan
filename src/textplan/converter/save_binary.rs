// SPDX-License-Identifier: Apache-2.0

//! Save a Substrait plan to binary format.

use crate::proto::{save_plan_to_binary, Plan, PlanRel, Rel};
use crate::textplan::common::error::TextPlanError;
use crate::textplan::symbol_table::{RelationType as SymbolRelationType, SymbolTable, SymbolType};
use ::substrait::proto::{
    plan_rel, read_rel, AggregateRel, FilterRel, JoinRel, ProjectRel, ReadRel, RelRoot, SortRel,
};
use std::boxed::Box;

/// Creates a Plan protobuf from a symbol table.
///
/// # Arguments
///
/// * `symbol_table` - The symbol table to convert.
///
/// # Returns
///
/// The Plan protobuf representation.
pub fn create_plan_from_symbol_table(symbol_table: &SymbolTable) -> Result<Plan, TextPlanError> {
    // Create a plan with the appropriate version
    let mut plan = Plan {
        version: Some(::substrait::proto::Version {
            major_number: 0,
            minor_number: 1,
            patch_number: 0,
            git_hash: String::new(),
            producer: "Substrait TextPlan Rust".to_string(),
        }),
        extension_uris: Vec::new(),
        extension_urns: Vec::new(),
        extensions: Vec::new(),
        relations: Vec::new(),
        expected_type_urls: Vec::new(),
        advanced_extensions: None,
        parameter_bindings: Vec::new(),
        type_aliases: Vec::new(),
    };

    // Find the root symbol if present
    let mut root_names = Vec::new();
    for symbol in symbol_table.symbols() {
        if symbol.symbol_type() == SymbolType::Root {
            // Add roots to the root_names list
            root_names.push(symbol.name().to_string());
        }
    }

    // If we found root names, add a root relation
    if !root_names.is_empty() {
        plan.relations.push(PlanRel {
            rel_type: Some(plan_rel::RelType::Root(RelRoot {
                names: root_names,
                input: None,
            })),
        });
    }

    // Collect relations from the symbol table
    for symbol in symbol_table.symbols() {
        if symbol.symbol_type() == SymbolType::Relation {
            // Extract relation type
            if let Some(symbol_rel_type) = symbol.subtype::<SymbolRelationType>() {
                let rel_obj = match symbol_rel_type {
                    SymbolRelationType::Read => {
                        // Create a read relation
                        let read_rel = ReadRel {
                            common: None,
                            base_schema: None,
                            filter: None,
                            best_effort_filter: None,
                            projection: None,
                            advanced_extension: None,
                            read_type: Some(read_rel::ReadType::NamedTable(read_rel::NamedTable {
                                names: vec![symbol.name().to_string()],
                                advanced_extension: None,
                            })),
                        };
                        Rel {
                            rel_type: Some(::substrait::proto::rel::RelType::Read(Box::new(
                                read_rel,
                            ))),
                        }
                    }
                    SymbolRelationType::Filter => {
                        // Create a filter relation
                        let filter_rel = FilterRel {
                            input: None,
                            condition: None,
                            common: None,
                            advanced_extension: None,
                        };
                        Rel {
                            rel_type: Some(::substrait::proto::rel::RelType::Filter(Box::new(
                                filter_rel,
                            ))),
                        }
                    }
                    SymbolRelationType::Project => {
                        // Create a project relation
                        let project_rel = ProjectRel {
                            input: None,
                            expressions: Vec::new(),
                            common: None,
                            advanced_extension: None,
                        };
                        Rel {
                            rel_type: Some(::substrait::proto::rel::RelType::Project(Box::new(
                                project_rel,
                            ))),
                        }
                    }
                    SymbolRelationType::Join => {
                        // Create a join relation
                        let join_rel = JoinRel {
                            left: None,
                            right: None,
                            expression: None,
                            r#type: 0,
                            post_join_filter: None,
                            common: None,
                            advanced_extension: None,
                        };
                        Rel {
                            rel_type: Some(::substrait::proto::rel::RelType::Join(Box::new(
                                join_rel,
                            ))),
                        }
                    }
                    SymbolRelationType::Aggregate => {
                        // Create an aggregate relation
                        let agg_rel = AggregateRel {
                            input: None,
                            groupings: Vec::new(),
                            measures: Vec::new(),
                            common: None,
                            advanced_extension: None,
                            grouping_expressions: Vec::new(),
                        };
                        Rel {
                            rel_type: Some(::substrait::proto::rel::RelType::Aggregate(Box::new(
                                agg_rel,
                            ))),
                        }
                    }
                    SymbolRelationType::Sort => {
                        // Create a sort relation
                        let sort_rel = SortRel {
                            input: None,
                            sorts: Vec::new(),
                            common: None,
                            advanced_extension: None,
                        };
                        Rel {
                            rel_type: Some(::substrait::proto::rel::RelType::Sort(Box::new(
                                sort_rel,
                            ))),
                        }
                    }
                    _ => continue, // Skip unknown relation types
                };

                // Add the relation to the plan
                plan.relations.push(PlanRel {
                    rel_type: Some(plan_rel::RelType::Rel(rel_obj)),
                });
            }
        }
    }

    Ok(plan)
}

/// Saves a Substrait plan to binary format.
///
/// # Arguments
///
/// * `symbol_table` - The symbol table to save.
///
/// # Returns
///
/// The binary representation of the plan.
pub fn save_to_binary(symbol_table: &SymbolTable) -> Result<Vec<u8>, TextPlanError> {
    // Create the plan from the symbol table
    let plan = create_plan_from_symbol_table(symbol_table)?;

    // Serialize the plan to bytes
    serialize_plan_to_binary(&plan)
}

/// Serializes a Plan to binary protobuf using prost.
///
/// # Arguments
///
/// * `plan` - The plan to serialize.
///
/// # Returns
///
/// The binary protobuf representation of the plan.
fn serialize_plan_to_binary(plan: &Plan) -> Result<Vec<u8>, TextPlanError> {
    // Use the existing function if available, otherwise implement with prost
    save_plan_to_binary(plan)
}
