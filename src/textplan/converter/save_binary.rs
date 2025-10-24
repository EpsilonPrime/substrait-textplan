// SPDX-License-Identifier: Apache-2.0

//! Save a Substrait plan to binary format.

use crate::proto::{save_plan_to_binary, Plan, PlanRel};
use crate::textplan::common::error::TextPlanError;
use crate::textplan::common::structured_symbol_data::RelationData;
use crate::textplan::symbol_table::{SymbolInfo, SymbolTable, SymbolType};
use ::substrait::proto::{plan_rel, rel, Rel, RelRoot};
use std::collections::HashSet;
use std::sync::Arc;

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

    // Find root relations and recursively build nested trees.
    // Following the C++ implementation in SymbolTablePrinter::outputToBinaryPlan()
    println!("Iterating over symbols, total count: {}", symbol_table.symbols().len());
    for symbol in symbol_table.symbols() {
        println!("Checking symbol '{}' of type {:?}", symbol.name(), symbol.symbol_type());
        if symbol.symbol_type() == SymbolType::Relation {
            // Check if this is a pipeline terminal (end of pipeline with no continuing relation)
            let is_pipeline_terminal = {
                println!("  Checking if '{}' is pipeline_terminal (locking for check)", symbol.name());
                if let Some(blob_lock) = &symbol.blob {
                    if let Ok(blob_data) = blob_lock.lock() {
                        println!("    Successfully locked '{}' for pipeline_terminal check", symbol.name());
                        if let Some(relation_data) = blob_data.downcast_ref::<RelationData>() {
                            // A pipeline terminal (end of pipeline to output) has:
                            // - continuing_pipeline == None (nothing follows it in the pipeline)
                            // - pipeline_start != None (it's part of a pipeline, not orphaned)
                            // - pipeline_start does NOT point to self (it's not the data source leaf)
                            let is_not_pipeline_start = relation_data.pipeline_start.as_ref()
                                .map_or(true, |start| !Arc::ptr_eq(start, symbol));
                            let result = relation_data.continuing_pipeline.is_none()
                                && relation_data.pipeline_start.is_some()
                                && is_not_pipeline_start;
                            println!(
                                "Relation '{}': continuing_pipeline={:?}, pipeline_start={:?}, is_pipeline_terminal={}",
                                symbol.name(),
                                relation_data.continuing_pipeline.as_ref().map(|s| s.name()),
                                relation_data.pipeline_start.as_ref().map(|s| s.name()),
                                result
                            );
                            result
                        } else {
                            false
                        }
                    } else {
                        println!("    FAILED to lock '{}' for pipeline_terminal check", symbol.name());
                        false
                    }
                } else {
                    false
                }
            };
            println!("  Lock dropped for '{}' after pipeline_terminal check", symbol.name());
            // Lock is dropped here before we recurse

            if is_pipeline_terminal {
                // Check if this is a "root" symbol - wrap in Root plan relation
                if symbol.name() == "root" {
                    // Get the input from new_pipelines[0]
                    if let Some(blob_lock) = &symbol.blob {
                        if let Ok(blob_data) = blob_lock.lock() {
                            if let Some(relation_data) = blob_data.downcast_ref::<RelationData>() {
                                if !relation_data.new_pipelines.is_empty() {
                                    // Clone input_symbol to avoid borrow issues
                                    let input_symbol = relation_data.new_pipelines[0].clone();
                                    drop(blob_data); // Drop the lock early

                                    // Get the input's Rel and build the tree
                                    if let Some(input_blob) = &input_symbol.blob {
                                        if let Ok(input_data) = input_blob.lock() {
                                            if let Some(input_rel_data) = input_data.downcast_ref::<RelationData>() {
                                                let mut input_rel = input_rel_data.relation.clone();
                                                drop(input_data);

                                                let mut visited = HashSet::new();
                                                add_inputs_to_relation(
                                                    symbol_table,
                                                    &input_symbol,
                                                    &mut input_rel,
                                                    &mut visited,
                                                )?;

                                                // Wrap in Root plan relation
                                                plan.relations.push(PlanRel {
                                                    rel_type: Some(plan_rel::RelType::Root(RelRoot {
                                                        input: Some(input_rel),
                                                        names: Vec::new(), // TODO: Get names from root relation definition
                                                    })),
                                                });
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                } else {
                    // Not a root symbol - add as regular Rel
                    if let Some(blob_lock) = &symbol.blob {
                        if let Ok(blob_data) = blob_lock.lock() {
                            if let Some(relation_data) = blob_data.downcast_ref::<RelationData>() {
                                let mut root_rel = relation_data.relation.clone();
                                // Drop the lock before recursing
                                drop(blob_data);

                                let mut visited = HashSet::new();
                                add_inputs_to_relation(
                                    symbol_table,
                                    symbol,
                                    &mut root_rel,
                                    &mut visited,
                                )?;

                                // Add as a plan relation
                                plan.relations.push(PlanRel {
                                    rel_type: Some(plan_rel::RelType::Rel(root_rel)),
                                });
                            }
                        }
                    }
                }
            }
        }
    }

    Ok(plan)
}

/// Populates a ReadRel protobuf from symbol table references.
fn populate_read_rel(
    _symbol_table: &SymbolTable,
    source_symbol: &Option<Arc<SymbolInfo>>,
    schema_symbol: &Option<Arc<SymbolInfo>>,
    read_rel: &mut ::substrait::proto::ReadRel,
) -> Result<(), TextPlanError> {
    // Populate base_schema from schema symbol
    if let Some(schema_sym) = schema_symbol {
        // TODO: Build NamedStruct from schema symbol's field definitions
        println!("  TODO: Populate base_schema from schema '{}'", schema_sym.name());
    }

    // Populate namedTable from source symbol
    if let Some(source_sym) = source_symbol {
        // TODO: Extract table names from source symbol
        // For now, use the source symbol name as a placeholder
        read_rel.read_type = Some(::substrait::proto::read_rel::ReadType::NamedTable(
            ::substrait::proto::read_rel::NamedTable {
                names: vec![source_sym.name().to_string()],
                advanced_extension: None,
            }
        ));
        println!("  Populated namedTable with source '{}'", source_sym.name());
    }

    // Set common to direct emission (no projection)
    read_rel.common = Some(::substrait::proto::RelCommon {
        emit_kind: Some(::substrait::proto::rel_common::EmitKind::Direct(
            ::substrait::proto::rel_common::Direct {}
        )),
        ..Default::default()
    });

    Ok(())
}

/// Recursively adds inputs to a relation by following pipeline links.
/// Based on C++ SymbolTablePrinter::addInputsToRelation()
fn add_inputs_to_relation(
    symbol_table: &SymbolTable,
    symbol: &Arc<SymbolInfo>,
    rel: &mut Rel,
    visited: &mut HashSet<*const SymbolInfo>,
) -> Result<(), TextPlanError> {
    println!("add_inputs_to_relation: Entering for symbol '{}'", symbol.name());

    // Check for cycles
    let symbol_ptr = Arc::as_ptr(symbol);
    if visited.contains(&symbol_ptr) {
        println!("  Already visited '{}', returning", symbol.name());
        return Ok(()); // Already visited, stop recursion
    }
    visited.insert(symbol_ptr);
    println!("  Marked '{}' as visited", symbol.name());
    // Get the relation data
    let blob_lock = symbol.blob.as_ref().ok_or_else(|| {
        TextPlanError::ProtobufError(format!(
            "Relation symbol '{}' has no blob data",
            symbol.name()
        ))
    })?;

    // Clone ALL the data we need before recursing to avoid holding locks during recursion
    // This includes not just the Arc<SymbolInfo>, but also the Rel protobufs
    println!("  Attempting to lock blob for '{}'", symbol.name());
    let result = {
        let blob_data = blob_lock.lock().map_err(|_| {
            TextPlanError::ProtobufError(format!(
                "Failed to lock blob for relation '{}'",
                symbol.name()
            ))
        })?;

        println!("  Successfully locked blob for '{}'", symbol.name());

        let relation_data = blob_data.downcast_ref::<RelationData>().ok_or_else(|| {
            TextPlanError::ProtobufError(format!(
                "Relation symbol '{}' blob is not RelationData",
                symbol.name()
            ))
        })?;

        // Clone continuing_pipeline symbol and its Rel
        let continuing_pipeline = relation_data.continuing_pipeline.clone();
        let continuing_pipeline_rel = continuing_pipeline.as_ref().and_then(|next| {
            next.blob.as_ref().and_then(|next_blob| {
                next_blob.lock().ok().and_then(|next_data| {
                    next_data.downcast_ref::<RelationData>().map(|next_rel_data| {
                        next_rel_data.relation.clone()
                    })
                })
            })
        });

        // Clone new_pipelines symbols and their Rels
        let new_pipelines = relation_data.new_pipelines.clone();
        let new_pipelines_rels: Vec<Option<Rel>> = new_pipelines.iter().map(|pipeline_sym| {
            pipeline_sym.blob.as_ref().and_then(|blob| {
                blob.lock().ok().and_then(|data| {
                    data.downcast_ref::<RelationData>().map(|rel_data| {
                        rel_data.relation.clone()
                    })
                })
            })
        }).collect();

        // Clone source and schema symbols for building protobufs
        let source_symbol = relation_data.source.clone();
        let schema_symbol = relation_data.schema.clone();

        println!("  '{}' has continuing_pipeline={:?}, new_pipelines.len={}",
                 symbol.name(),
                 continuing_pipeline.as_ref().map(|s| s.name()),
                 new_pipelines.len());
        (continuing_pipeline, continuing_pipeline_rel, new_pipelines, new_pipelines_rels, source_symbol, schema_symbol)
    };
    // All locks are dropped here
    let (continuing_pipeline, continuing_pipeline_rel, new_pipelines, new_pipelines_rels, source_symbol, schema_symbol) = result;
    println!("  Lock dropped for '{}'", symbol.name());

    // Pattern match on relation type and recursively add inputs
    println!("  Matching on rel_type for '{}'", symbol.name());
    if let Some(rel_type) = &mut rel.rel_type {
        match rel_type {
            // Single-input relations
            rel::RelType::Read(read_rel) => {
                println!("    '{}' is Read (no inputs)", symbol.name());
                // Populate the ReadRel from symbol references
                populate_read_rel(symbol_table, &source_symbol, &schema_symbol, read_rel)?;
                // Read has no inputs
            }
            rel::RelType::Filter(filter_rel) => {
                println!("    '{}' is Filter", symbol.name());
                if let (Some(next), Some(next_rel)) = (&continuing_pipeline, &continuing_pipeline_rel) {
                    println!("      Filter '{}' has continuing_pipeline: '{}'", symbol.name(), next.name());
                    filter_rel.input = Some(Box::new(next_rel.clone()));
                    if let Some(input) = &mut filter_rel.input {
                        println!("      Recursing from Filter '{}' to '{}'", symbol.name(), next.name());
                        add_inputs_to_relation(symbol_table, next, input, visited)?;
                        println!("      Returned from recursion to '{}' from '{}'", next.name(), symbol.name());
                    }
                }
            }
            rel::RelType::Project(project_rel) => {
                if let (Some(next), Some(next_rel)) = (&continuing_pipeline, &continuing_pipeline_rel) {
                    project_rel.input = Some(Box::new(next_rel.clone()));
                    if let Some(input) = &mut project_rel.input {
                        add_inputs_to_relation(symbol_table, next, input, visited)?;
                    }
                }
            }
            rel::RelType::Aggregate(agg_rel) => {
                if let (Some(next), Some(next_rel)) = (&continuing_pipeline, &continuing_pipeline_rel) {
                    agg_rel.input = Some(Box::new(next_rel.clone()));
                    if let Some(input) = &mut agg_rel.input {
                        add_inputs_to_relation(symbol_table, next, input, visited)?;
                    }
                }
            }
            rel::RelType::Sort(sort_rel) => {
                if let (Some(next), Some(next_rel)) = (&continuing_pipeline, &continuing_pipeline_rel) {
                    sort_rel.input = Some(Box::new(next_rel.clone()));
                    if let Some(input) = &mut sort_rel.input {
                        add_inputs_to_relation(symbol_table, next, input, visited)?;
                    }
                }
            }
            // Binary relations (two inputs)
            rel::RelType::Join(join_rel) => {
                if new_pipelines.len() >= 2 && new_pipelines_rels.len() >= 2 {
                    // Left input
                    if let Some(left_rel) = &new_pipelines_rels[0] {
                        join_rel.left = Some(Box::new(left_rel.clone()));
                        if let Some(left) = &mut join_rel.left {
                            add_inputs_to_relation(
                                symbol_table,
                                &new_pipelines[0],
                                left,
                                visited,
                            )?;
                        }
                    }
                    // Right input
                    if let Some(right_rel) = &new_pipelines_rels[1] {
                        join_rel.right = Some(Box::new(right_rel.clone()));
                        if let Some(right) = &mut join_rel.right {
                            add_inputs_to_relation(
                                symbol_table,
                                &new_pipelines[1],
                                right,
                                visited,
                            )?;
                        }
                    }
                }
            }
            rel::RelType::Cross(cross_rel) => {
                if new_pipelines.len() >= 2 && new_pipelines_rels.len() >= 2 {
                    // Left input
                    if let Some(left_rel) = &new_pipelines_rels[0] {
                        cross_rel.left = Some(Box::new(left_rel.clone()));
                        if let Some(left) = &mut cross_rel.left {
                            add_inputs_to_relation(
                                symbol_table,
                                &new_pipelines[0],
                                left,
                                visited,
                            )?;
                        }
                    }
                    // Right input
                    if let Some(right_rel) = &new_pipelines_rels[1] {
                        cross_rel.right = Some(Box::new(right_rel.clone()));
                        if let Some(right) = &mut cross_rel.right {
                            add_inputs_to_relation(
                                symbol_table,
                                &new_pipelines[1],
                                right,
                                visited,
                            )?;
                        }
                    }
                }
            }
            // Other relation types can be added as needed
            _ => {}
        }
    }

    Ok(())
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
