// SPDX-License-Identifier: Apache-2.0

//! Load a binary Substrait plan and convert it to a textplan.

use crate::proto;
use crate::textplan::common::error::TextPlanError;
use crate::textplan::converter::initial_plan_visitor::InitialPlanVisitor;
use crate::textplan::converter::pipeline_visitor::PipelineVisitor;
use crate::textplan::printer::plan_printer::{PlanPrinter, TextPlanFormat};
use crate::textplan::symbol_table::SymbolTable;
use crate::textplan::SymbolInfo;
use std::sync::Arc;

/// Loads a binary Substrait plan and converts it to a textplan.
///
/// # Arguments
///
/// * `bytes` - The binary plan to load.
///
/// # Returns
///
/// The textplan representation of the plan.
pub fn load_from_binary(bytes: &[u8]) -> Result<String, TextPlanError> {
    // Deserialize the binary data to a Plan
    let plan = proto::load_plan_from_binary(bytes)?;

    // Convert the plan to a textplan
    convert_plan_to_text(&plan)
}

/// Converts a Plan to a textplan.
///
/// # Arguments
///
/// * `plan` - The Substrait plan to convert.
///
/// # Returns
///
/// The textplan representation of the plan.
fn convert_plan_to_text(plan: &substrait::proto::Plan) -> Result<String, TextPlanError> {
    // Start building the textplan string
    let mut textplan = String::new();

    // Add version information
    if let Some(version) = &plan.version {
        textplan.push_str(&format!(
            "// Substrait plan version: {}.{}.{}\n",
            version.major_number, version.minor_number, version.patch_number
        ));
        if !version.producer.is_empty() {
            textplan.push_str(&format!("// Producer: {}\n", version.producer));
        }
    }
    textplan.push_str("\n");

    // Add extension URIs if present
    if !plan.extension_uris.is_empty() {
        textplan.push_str("// Extension URIs:\n");
        for uri in &plan.extension_uris {
            textplan.push_str(&format!("// - {}: {}\n", uri.extension_uri_anchor, uri.uri));
        }
        textplan.push_str("\n");
    }

    // Add extensions if present
    if !plan.extensions.is_empty() {
        textplan.push_str("// Extensions:\n");
        for ext in &plan.extensions {
            // Use the correct mapping_type oneof field
            if let Some(mapping_type) = &ext.mapping_type {
                use ::substrait::proto::extensions::simple_extension_declaration::MappingType;
                match mapping_type {
                    MappingType::ExtensionFunction(func) => {
                        textplan.push_str(&format!(
                            "// - URI Ref: {}, Function: {}, Name: {}\n",
                            func.extension_uri_reference, func.function_anchor, func.name
                        ));
                    }
                    MappingType::ExtensionType(typ) => {
                        textplan.push_str(&format!(
                            "// - URI Ref: {}, Type: {}, Name: {}\n",
                            typ.extension_uri_reference, typ.type_anchor, typ.name
                        ));
                    }
                    MappingType::ExtensionTypeVariation(var) => {
                        textplan.push_str(&format!(
                            "// - URI Ref: {}, Type Variation: {}, Name: {}\n",
                            var.extension_uri_reference, var.type_variation_anchor, var.name
                        ));
                    }
                }
            }
        }
        textplan.push_str("\n");
    }

    // Process the plan using the PipelineVisitor to build a symbol table
    let plan_body = process_plan_with_visitor(plan)?;

    // Append the plan body to the header comments
    textplan.push_str(&plan_body);

    Ok(textplan)
}

/// Processes a plan using the PipelineVisitor to extract structured information.
///
/// # Arguments
///
/// * `plan` - The Substrait plan to process.
///
/// # Returns
///
/// The textplan body generated from the symbol table
pub fn process_plan_with_visitor(plan: &substrait::proto::Plan) -> Result<String, TextPlanError> {
    // Create a symbol table for the plan
    let symbol_table = SymbolTable::new();

    let mut visitor1 = InitialPlanVisitor::new(symbol_table);
    visitor1.visit_plan(plan);
    // MEGAHACK -- Check for errors.

    println!(
        "DEBUG: After InitialPlanVisitor, symbol table has {} symbols",
        visitor1.symbol_table().len()
    );
    for symbol in visitor1.symbol_table().symbols() {
        if symbol.symbol_type() == crate::textplan::SymbolType::Relation {
            println!(
                "  - {} (type: {:?}, location: {:?})",
                symbol.name(),
                symbol.symbol_type(),
                symbol.source_location()
            );
        } else {
            println!("  - {} (type: {:?})", symbol.name(), symbol.symbol_type());
        }
    }

    // Create a pipeline visitor with the symbol table
    let mut visitor = PipelineVisitor::new(visitor1.symbol_table_mut().clone());

    // Visit the plan to build the symbol table
    visitor.visit_plan(plan);
    // MEGAHACK -- Check for errors.

    println!(
        "DEBUG: After PipelineVisitor, symbol table has {} symbols",
        visitor.symbol_table().len()
    );
    for symbol in visitor.symbol_table().symbols() {
        println!("  - {} (type: {:?})", symbol.name(), symbol.symbol_type());
    }

    // Populate sub_query_pipelines by finding subquery relations
    populate_subquery_pipelines(visitor.symbol_table_mut())?;

    // Get the populated symbol table from the visitor
    let symbol_table = visitor.symbol_table().clone();

    // Create a plan printer and use it to convert the symbol table to a textplan
    let mut printer = PlanPrinter::new(TextPlanFormat::Standard);
    let plan_text = printer.print_plan(&symbol_table)?;

    Ok(plan_text)
}

/// Populates sub_query_pipelines for relations containing subquery expressions.
///
/// This function finds all subquery relations referenced in expressions and adds
/// their pipeline starts to the parent relation's sub_query_pipelines vector.
fn populate_subquery_pipelines(symbol_table: &mut SymbolTable) -> Result<(), TextPlanError> {
    println!("DEBUG: Populating sub_query_pipelines");

    // Collect all relations with subquery expressions
    let mut relations_with_subqueries = Vec::new();

    for symbol in symbol_table.symbols() {
        if symbol.symbol_type() == crate::textplan::SymbolType::Relation {
            if let Some(blob_lock) = &symbol.blob {
                if let Ok(blob_data) = blob_lock.lock() {
                    if let Some(relation_data) = blob_data.downcast_ref::<crate::textplan::common::structured_symbol_data::RelationData>() {
                        // Check if this relation has any subquery expressions
                        let has_subquery = has_subquery_expression(&relation_data.relation);
                        if has_subquery {
                            println!("  Found relation '{}' with subquery expressions", symbol.name());
                            relations_with_subqueries.push(symbol.clone());
                        }
                    }
                }
            }
        }
    }

    // For each relation with subqueries, extract and add subquery pipeline starts
    for parent_symbol in relations_with_subqueries {
        extract_and_add_subquery_pipelines(symbol_table, &parent_symbol)?;
    }

    Ok(())
}

/// Checks if a Rel contains any subquery expressions.
fn has_subquery_expression(rel: &substrait::proto::Rel) -> bool {
    use substrait::proto::rel::RelType;

    match &rel.rel_type {
        Some(RelType::Filter(filter_rel)) => {
            if let Some(condition) = &filter_rel.condition {
                has_subquery_in_expression(condition)
            } else {
                false
            }
        }
        Some(RelType::Project(project_rel)) => project_rel
            .expressions
            .iter()
            .any(has_subquery_in_expression),
        Some(RelType::Join(join_rel)) => {
            if let Some(expr) = &join_rel.expression {
                has_subquery_in_expression(expr)
            } else {
                false
            }
        }
        // Add other relation types as needed
        _ => false,
    }
}

/// Checks if an Expression contains a subquery.
fn has_subquery_in_expression(expr: &substrait::proto::Expression) -> bool {
    use substrait::proto::expression::RexType;

    match &expr.rex_type {
        Some(RexType::Subquery(_)) => true,
        Some(RexType::ScalarFunction(func)) => func.arguments.iter().any(|arg| {
            if let Some(substrait::proto::function_argument::ArgType::Value(inner_expr)) =
                &arg.arg_type
            {
                has_subquery_in_expression(inner_expr)
            } else {
                false
            }
        }),
        // Add other expression types as needed
        _ => false,
    }
}

/// Extracts subquery relations from a parent relation and adds their pipeline starts.
fn extract_and_add_subquery_pipelines(
    symbol_table: &SymbolTable,
    parent_symbol: &Arc<SymbolInfo>,
) -> Result<(), TextPlanError> {
    // Get the relation data
    let blob_lock = parent_symbol.blob.as_ref().ok_or_else(|| {
        TextPlanError::ProtobufError(format!(
            "Parent relation '{}' has no blob data",
            parent_symbol.name()
        ))
    })?;

    let mut blob_data = blob_lock.lock().map_err(|_| {
        TextPlanError::ProtobufError(format!(
            "Failed to lock blob for parent relation '{}'",
            parent_symbol.name()
        ))
    })?;

    let relation_data = blob_data
        .downcast_mut::<crate::textplan::common::structured_symbol_data::RelationData>()
        .ok_or_else(|| {
            TextPlanError::ProtobufError(format!(
                "Parent relation '{}' blob is not RelationData",
                parent_symbol.name()
            ))
        })?;

    // Extract subquery pipeline starts
    let subquery_starts = extract_subquery_starts(symbol_table, &relation_data.relation)?;

    println!(
        "    Found {} subquery pipeline starts for '{}'",
        subquery_starts.len(),
        parent_symbol.name()
    );

    // Add to sub_query_pipelines
    relation_data.sub_query_pipelines.extend(subquery_starts);

    Ok(())
}

/// Extracts subquery pipeline starts from a Rel.
fn extract_subquery_starts(
    symbol_table: &SymbolTable,
    rel: &substrait::proto::Rel,
) -> Result<Vec<Arc<SymbolInfo>>, TextPlanError> {
    use substrait::proto::rel::RelType;

    let mut starts = Vec::new();

    match &rel.rel_type {
        Some(RelType::Filter(filter_rel)) => {
            if let Some(condition) = &filter_rel.condition {
                starts.extend(extract_subquery_starts_from_expression(
                    symbol_table,
                    condition,
                )?);
            }
        }
        Some(RelType::Project(project_rel)) => {
            for expr in &project_rel.expressions {
                starts.extend(extract_subquery_starts_from_expression(symbol_table, expr)?);
            }
        }
        Some(RelType::Join(join_rel)) => {
            if let Some(expr) = &join_rel.expression {
                starts.extend(extract_subquery_starts_from_expression(symbol_table, expr)?);
            }
        }
        _ => {}
    }

    Ok(starts)
}

/// Extracts subquery pipeline starts from an Expression.
fn extract_subquery_starts_from_expression(
    symbol_table: &SymbolTable,
    expr: &substrait::proto::Expression,
) -> Result<Vec<Arc<SymbolInfo>>, TextPlanError> {
    use substrait::proto::expression::{subquery::SubqueryType, RexType};

    let mut starts = Vec::new();

    match &expr.rex_type {
        Some(RexType::Subquery(_subquery)) => {
            // The subquery relation should already have parent_query_location set by InitialPlanVisitor
            // We can find it by looking for relations with matching parent_query info
            // However, for now, let's just find all relations that are part of a subquery
            // by checking which relations have parent_query_index >= 0

            // For each relation in the symbol table, check if it's a subquery relation
            for symbol in symbol_table.symbols() {
                if symbol.symbol_type() == crate::textplan::SymbolType::Relation {
                    // Check if this relation has parent_query info (indicates it's a subquery)
                    if symbol.parent_query_index() >= 0 {
                        // Find the pipeline start for this relation
                        if let Some(start) = find_pipeline_start(symbol_table, &symbol)? {
                            println!("      Found subquery pipeline start: '{}'", start.name());
                            if !starts.iter().any(|s| Arc::ptr_eq(s, &start)) {
                                starts.push(start);
                            }
                        }
                    }
                }
            }
        }
        Some(RexType::ScalarFunction(func)) => {
            // Recursively check arguments
            for arg in &func.arguments {
                if let Some(substrait::proto::function_argument::ArgType::Value(inner_expr)) =
                    &arg.arg_type
                {
                    starts.extend(extract_subquery_starts_from_expression(
                        symbol_table,
                        inner_expr,
                    )?);
                }
            }
        }
        _ => {}
    }

    Ok(starts)
}

/// Finds the pipeline start for a given relation by following continuing_pipeline backwards.
fn find_pipeline_start(
    symbol_table: &SymbolTable,
    rel_symbol: &Arc<SymbolInfo>,
) -> Result<Option<Arc<SymbolInfo>>, TextPlanError> {
    let blob_lock = rel_symbol.blob.as_ref().ok_or_else(|| {
        TextPlanError::ProtobufError(format!("Relation '{}' has no blob data", rel_symbol.name()))
    })?;

    let blob_data = blob_lock.lock().map_err(|_| {
        TextPlanError::ProtobufError(format!(
            "Failed to lock blob for relation '{}'",
            rel_symbol.name()
        ))
    })?;

    let relation_data = blob_data
        .downcast_ref::<crate::textplan::common::structured_symbol_data::RelationData>()
        .ok_or_else(|| {
            TextPlanError::ProtobufError(format!(
                "Relation '{}' blob is not RelationData",
                rel_symbol.name()
            ))
        })?;

    // If this relation has a pipeline_start, return it
    if let Some(start) = &relation_data.pipeline_start {
        Ok(Some(start.clone()))
    } else {
        // This relation IS the pipeline start
        Ok(Some(rel_symbol.clone()))
    }
}
