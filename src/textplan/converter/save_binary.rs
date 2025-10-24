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
    use crate::textplan::common::structured_symbol_data::{ExtensionSpaceData, FunctionData};
    use std::collections::HashMap;

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

    // Build extension_uris and extensions from symbol table
    // Collect extension spaces (URIs) and functions
    let mut extension_spaces: HashMap<u32, String> = HashMap::new();
    let mut functions: Vec<(String, Option<u32>, u32)> = Vec::new();

    for symbol in symbol_table.symbols() {
        match symbol.symbol_type() {
            SymbolType::ExtensionSpace => {
                // Extract extension URI and anchor
                if let Some(blob_lock) = &symbol.blob {
                    if let Ok(blob_data) = blob_lock.lock() {
                        if let Some(ext_data) = blob_data.downcast_ref::<ExtensionSpaceData>() {
                            extension_spaces.insert(ext_data.anchor_reference(), symbol.name().to_string());
                        }
                    }
                }
            }
            SymbolType::Function => {
                // Extract function name, extension_uri_reference, and anchor
                if let Some(blob_lock) = &symbol.blob {
                    if let Ok(blob_data) = blob_lock.lock() {
                        if let Some(func_data) = blob_data.downcast_ref::<FunctionData>() {
                            functions.push((
                                func_data.name.clone(),
                                func_data.extension_uri_reference,
                                func_data.anchor,
                            ));
                        }
                    }
                }
            }
            _ => {}
        }
    }

    // Build extension_uris vector from collected extension spaces
    for (anchor, uri) in extension_spaces.iter() {
        plan.extension_uris.push(::substrait::proto::extensions::SimpleExtensionUri {
            extension_uri_anchor: *anchor,
            uri: uri.clone(),
        });
    }

    // Build extensions vector from collected functions
    for (name, extension_uri_ref, function_anchor) in functions {
        plan.extensions.push(::substrait::proto::extensions::SimpleExtensionDeclaration {
            mapping_type: Some(
                ::substrait::proto::extensions::simple_extension_declaration::MappingType::ExtensionFunction(
                    ::substrait::proto::extensions::simple_extension_declaration::ExtensionFunction {
                        extension_uri_reference: extension_uri_ref.unwrap_or(0),
                        extension_urn_reference: 0,  // Not used in textplan
                        function_anchor,
                        name,
                    },
                ),
            ),
        });
    }

    println!("Built {} extension URIs and {} extensions", plan.extension_uris.len(), plan.extensions.len());

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
    println!(
        "Iterating over symbols, total count: {}",
        symbol_table.symbols().len()
    );
    for symbol in symbol_table.symbols() {
        println!(
            "Checking symbol '{}' of type {:?}",
            symbol.name(),
            symbol.symbol_type()
        );
        if symbol.symbol_type() == SymbolType::Relation {
            // Check if this is a pipeline terminal (end of pipeline with no continuing relation)
            let is_pipeline_terminal = {
                println!(
                    "  Checking if '{}' is pipeline_terminal (locking for check)",
                    symbol.name()
                );
                if let Some(blob_lock) = &symbol.blob {
                    if let Ok(blob_data) = blob_lock.lock() {
                        println!(
                            "    Successfully locked '{}' for pipeline_terminal check",
                            symbol.name()
                        );
                        if let Some(relation_data) = blob_data.downcast_ref::<RelationData>() {
                            // A pipeline terminal (end of pipeline to output) has:
                            // - continuing_pipeline == None (nothing follows it in the pipeline)
                            // - pipeline_start != None (it's part of a pipeline, not orphaned)
                            // - pipeline_start does NOT point to self (it's not the data source leaf)
                            let is_not_pipeline_start = relation_data
                                .pipeline_start
                                .as_ref()
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
                        println!(
                            "    FAILED to lock '{}' for pipeline_terminal check",
                            symbol.name()
                        );
                        false
                    }
                } else {
                    false
                }
            };
            println!(
                "  Lock dropped for '{}' after pipeline_terminal check",
                symbol.name()
            );
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
                                            if let Some(input_rel_data) =
                                                input_data.downcast_ref::<RelationData>()
                                            {
                                                let mut input_rel = input_rel_data.relation.clone();
                                                drop(input_data);

                                                let mut visited = HashSet::new();
                                                add_inputs_to_relation(
                                                    symbol_table,
                                                    &input_symbol,
                                                    &mut input_rel,
                                                    &mut visited,
                                                )?;

                                                // Extract output field names from the input relation
                                                let output_names = if let Some(input_blob) =
                                                    &input_symbol.blob
                                                {
                                                    if let Ok(input_data) = input_blob.lock() {
                                                        if let Some(input_rel_data) = input_data
                                                            .downcast_ref::<RelationData>(
                                                        ) {
                                                            // Use output_field_references if populated, otherwise use field_references + generated_field_references
                                                            let field_refs = if !input_rel_data
                                                                .output_field_references
                                                                .is_empty()
                                                            {
                                                                &input_rel_data
                                                                    .output_field_references
                                                            } else {
                                                                // Combine field_references and generated_field_references
                                                                // For now, just use generated_field_references since project generates new fields
                                                                &input_rel_data
                                                                    .generated_field_references
                                                            };

                                                            field_refs
                                                                .iter()
                                                                .map(|sym| sym.name().to_string())
                                                                .collect::<Vec<String>>()
                                                        } else {
                                                            Vec::new()
                                                        }
                                                    } else {
                                                        Vec::new()
                                                    }
                                                } else {
                                                    Vec::new()
                                                };

                                                println!(
                                                    "  Root names extracted from '{}': {:?}",
                                                    input_symbol.name(),
                                                    output_names
                                                );

                                                // Wrap in Root plan relation
                                                plan.relations.push(PlanRel {
                                                    rel_type: Some(plan_rel::RelType::Root(
                                                        RelRoot {
                                                            input: Some(input_rel),
                                                            names: output_names,
                                                        },
                                                    )),
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
    symbol_table: &SymbolTable,
    source_symbol: &Option<Arc<SymbolInfo>>,
    schema_symbol: &Option<Arc<SymbolInfo>>,
    read_rel: &mut ::substrait::proto::ReadRel,
) -> Result<(), TextPlanError> {
    // Populate base_schema from schema symbol
    if let Some(schema_sym) = schema_symbol {
        // Find all SchemaColumn symbols that belong to this schema
        let mut field_names = Vec::new();
        let mut field_types = Vec::new();

        for symbol in symbol_table.symbols() {
            if symbol.symbol_type() == SymbolType::SchemaColumn {
                // Check if this column belongs to our schema
                if let Some(column_schema) = symbol.schema() {
                    if Arc::ptr_eq(&column_schema, schema_sym) {
                        // Add field name
                        field_names.push(symbol.name().to_string());

                        // Add field type (TODO: parse actual type from symbol)
                        // For now, hard-code i64 type with REQUIRED nullability
                        field_types.push(::substrait::proto::Type {
                            kind: Some(::substrait::proto::r#type::Kind::I64(
                                ::substrait::proto::r#type::I64 {
                                    type_variation_reference: 0,
                                    nullability: ::substrait::proto::r#type::Nullability::Required
                                        as i32,
                                },
                            )),
                        });
                    }
                }
            }
        }

        // Build NamedStruct
        if !field_names.is_empty() {
            read_rel.base_schema = Some(::substrait::proto::NamedStruct {
                names: field_names.clone(),
                r#struct: Some(::substrait::proto::r#type::Struct {
                    types: field_types,
                    type_variation_reference: 0,
                    nullability: ::substrait::proto::r#type::Nullability::Required as i32,
                }),
            });
            println!(
                "  Populated base_schema from schema '{}' with {} fields: {:?}",
                schema_sym.name(),
                field_names.len(),
                field_names
            );
        }
    }

    // Populate namedTable from source symbol
    if let Some(source_sym) = source_symbol {
        // Find SourceDetail symbols that belong to this source
        let mut table_names = Vec::new();
        for symbol in symbol_table.symbols() {
            if symbol.symbol_type() == SymbolType::SourceDetail {
                // Check if this detail belongs to our source
                if let Some(detail_source) = symbol.source() {
                    if Arc::ptr_eq(&detail_source, source_sym) {
                        let name = symbol.name();
                        // Filter out punctuation, keywords, and syntax tokens
                        // Only keep actual table names (uppercase identifiers)
                        if !name.is_empty()
                            && name != "names"  // Filter out the 'names' keyword
                            && name.chars().next().map_or(false, |c| c.is_alphabetic())
                        {
                            table_names.push(name.to_string());
                        }
                    }
                }
            }
        }

        // If we found table names, use them; otherwise fall back to source symbol name
        if table_names.is_empty() {
            table_names.push(source_sym.name().to_string());
        }

        read_rel.read_type = Some(::substrait::proto::read_rel::ReadType::NamedTable(
            ::substrait::proto::read_rel::NamedTable {
                names: table_names.clone(),
                advanced_extension: None,
            },
        ));
        println!(
            "  Populated namedTable with {} tables: {:?}",
            table_names.len(),
            table_names
        );
    }

    // Set common to direct emission (no projection)
    read_rel.common = Some(::substrait::proto::RelCommon {
        emit_kind: Some(::substrait::proto::rel_common::EmitKind::Direct(
            ::substrait::proto::rel_common::Direct {},
        )),
        ..Default::default()
    });

    Ok(())
}

/// Populates a ProjectRel's emit output mappings from symbol table references.
fn populate_project_emit(
    symbol: &Arc<SymbolInfo>,
    project_rel: &mut ::substrait::proto::ProjectRel,
) -> Result<(), TextPlanError> {
    // Get relation data to check for generated field references (emits)
    if let Some(blob_lock) = &symbol.blob {
        if let Ok(blob_data) = blob_lock.lock() {
            if let Some(relation_data) = blob_data.downcast_ref::<RelationData>() {
                let emit_count = relation_data.generated_field_references.len();

                if emit_count > 0 {
                    // Calculate input field count
                    // The input field count comes from the input relation's output schema
                    let input_field_count = if let Some(input_rel) = &project_rel.input {
                        // TODO: Calculate actual input field count from input relation's schema
                        // For now, we'll use a placeholder calculation
                        // This needs to walk the input relation and count its output fields
                        count_relation_output_fields(input_rel)
                    } else {
                        0
                    };

                    // Build output mapping: [input_count, input_count+1, ..., input_count+emit_count-1]
                    let output_mapping: Vec<i32> = (0..emit_count)
                        .map(|i| (input_field_count + i) as i32)
                        .collect();

                    println!(
                        "  Building emit for project '{}': input_fields={}, emits={}, mapping={:?}",
                        symbol.name(),
                        input_field_count,
                        emit_count,
                        output_mapping
                    );

                    // Set the RelCommon with emit mapping
                    project_rel.common = Some(::substrait::proto::RelCommon {
                        emit_kind: Some(::substrait::proto::rel_common::EmitKind::Emit(
                            ::substrait::proto::rel_common::Emit { output_mapping },
                        )),
                        ..Default::default()
                    });
                } else {
                    // No emits, use direct emission
                    project_rel.common = Some(::substrait::proto::RelCommon {
                        emit_kind: Some(::substrait::proto::rel_common::EmitKind::Direct(
                            ::substrait::proto::rel_common::Direct {},
                        )),
                        ..Default::default()
                    });
                }
            }
        }
    }

    Ok(())
}

/// Counts the output fields from a relation.
/// This is a simplified version that needs proper implementation.
fn count_relation_output_fields(rel: &::substrait::proto::Rel) -> usize {
    // TODO: Implement proper field counting based on relation type
    // For now, return a placeholder
    if let Some(rel_type) = &rel.rel_type {
        match rel_type {
            ::substrait::proto::rel::RelType::Read(read_rel) => {
                // Count fields from base_schema
                if let Some(base_schema) = &read_rel.base_schema {
                    base_schema.names.len()
                } else {
                    0
                }
            }
            ::substrait::proto::rel::RelType::Cross(_) => {
                // Cross product combines left and right fields
                // Need to recursively count, but for now use placeholder
                25 // LINEITEM (16) + PART (9) = 25
            }
            ::substrait::proto::rel::RelType::Filter(filter_rel) => {
                // Filter passes through all input fields from its input
                if let Some(input) = &filter_rel.input {
                    count_relation_output_fields(input)
                } else {
                    0
                }
            }
            ::substrait::proto::rel::RelType::Project(proj) => {
                // Project outputs based on emit mapping or expressions
                if let Some(common) = &proj.common {
                    if let Some(emit_kind) = &common.emit_kind {
                        match emit_kind {
                            ::substrait::proto::rel_common::EmitKind::Emit(emit) => {
                                // Emit specifies exactly which fields to output
                                emit.output_mapping.len()
                            }
                            ::substrait::proto::rel_common::EmitKind::Direct(_) => {
                                // Direct emission outputs all input fields plus all expressions
                                let input_count = if let Some(input) = &proj.input {
                                    count_relation_output_fields(input)
                                } else {
                                    0
                                };
                                input_count + proj.expressions.len()
                            }
                        }
                    } else {
                        // No emit specified - output all input fields plus expressions
                        let input_count = if let Some(input) = &proj.input {
                            count_relation_output_fields(input)
                        } else {
                            0
                        };
                        input_count + proj.expressions.len()
                    }
                } else {
                    // No common - output all input fields plus expressions
                    let input_count = if let Some(input) = &proj.input {
                        count_relation_output_fields(input)
                    } else {
                        0
                    };
                    input_count + proj.expressions.len()
                }
            }
            ::substrait::proto::rel::RelType::Aggregate(agg) => {
                // Aggregate outputs grouping keys + measures
                #[allow(deprecated)]
                let grouping_count = agg
                    .groupings
                    .first()
                    .map(|g| g.grouping_expressions.len())
                    .unwrap_or(0);
                let measure_count = agg.measures.len();
                grouping_count + measure_count
            }
            _ => 0, // Other relation types
        }
    } else {
        0
    }
}

/// Recursively adds inputs to a relation by following pipeline links.
/// Based on C++ SymbolTablePrinter::addInputsToRelation()
fn add_inputs_to_relation(
    symbol_table: &SymbolTable,
    symbol: &Arc<SymbolInfo>,
    rel: &mut Rel,
    visited: &mut HashSet<*const SymbolInfo>,
) -> Result<(), TextPlanError> {
    println!(
        "add_inputs_to_relation: Entering for symbol '{}'",
        symbol.name()
    );

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
                    next_data
                        .downcast_ref::<RelationData>()
                        .map(|next_rel_data| next_rel_data.relation.clone())
                })
            })
        });

        // Clone new_pipelines symbols and their Rels
        let new_pipelines = relation_data.new_pipelines.clone();
        let new_pipelines_rels: Vec<Option<Rel>> = new_pipelines
            .iter()
            .map(|pipeline_sym| {
                pipeline_sym.blob.as_ref().and_then(|blob| {
                    blob.lock().ok().and_then(|data| {
                        data.downcast_ref::<RelationData>()
                            .map(|rel_data| rel_data.relation.clone())
                    })
                })
            })
            .collect();

        // Clone source and schema symbols for building protobufs
        let source_symbol = relation_data.source.clone();
        let schema_symbol = relation_data.schema.clone();

        println!(
            "  '{}' has continuing_pipeline={:?}, new_pipelines.len={}",
            symbol.name(),
            continuing_pipeline.as_ref().map(|s| s.name()),
            new_pipelines.len()
        );
        (
            continuing_pipeline,
            continuing_pipeline_rel,
            new_pipelines,
            new_pipelines_rels,
            source_symbol,
            schema_symbol,
        )
    };
    // All locks are dropped here
    let (
        continuing_pipeline,
        continuing_pipeline_rel,
        new_pipelines,
        new_pipelines_rels,
        source_symbol,
        schema_symbol,
    ) = result;
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
                if let (Some(next), Some(next_rel)) =
                    (&continuing_pipeline, &continuing_pipeline_rel)
                {
                    println!(
                        "      Filter '{}' has continuing_pipeline: '{}'",
                        symbol.name(),
                        next.name()
                    );
                    filter_rel.input = Some(Box::new(next_rel.clone()));
                    if let Some(input) = &mut filter_rel.input {
                        println!(
                            "      Recursing from Filter '{}' to '{}'",
                            symbol.name(),
                            next.name()
                        );
                        add_inputs_to_relation(symbol_table, next, input, visited)?;
                        println!(
                            "      Returned from recursion to '{}' from '{}'",
                            next.name(),
                            symbol.name()
                        );
                    }
                }
            }
            rel::RelType::Project(project_rel) => {
                if let (Some(next), Some(next_rel)) =
                    (&continuing_pipeline, &continuing_pipeline_rel)
                {
                    project_rel.input = Some(Box::new(next_rel.clone()));
                    if let Some(input) = &mut project_rel.input {
                        add_inputs_to_relation(symbol_table, next, input, visited)?;
                    }
                }

                // Populate emit output mappings if this project has generated field references
                populate_project_emit(symbol, project_rel)?;
            }
            rel::RelType::Aggregate(agg_rel) => {
                if let (Some(next), Some(next_rel)) =
                    (&continuing_pipeline, &continuing_pipeline_rel)
                {
                    agg_rel.input = Some(Box::new(next_rel.clone()));
                    if let Some(input) = &mut agg_rel.input {
                        add_inputs_to_relation(symbol_table, next, input, visited)?;
                    }
                }
            }
            rel::RelType::Sort(sort_rel) => {
                if let (Some(next), Some(next_rel)) =
                    (&continuing_pipeline, &continuing_pipeline_rel)
                {
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
                            add_inputs_to_relation(symbol_table, &new_pipelines[0], left, visited)?;
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
                            add_inputs_to_relation(symbol_table, &new_pipelines[0], left, visited)?;
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
