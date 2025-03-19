// SPDX-License-Identifier: Apache-2.0

//! Load a binary Substrait plan and convert it to a textplan.

use prost::Message;
use crate::proto::substrait;
use crate::textplan::common::error::TextPlanError;
use crate::textplan::converter::visitor::PipelineVisitor;
use crate::textplan::converter::visitor::BasePlanProtoVisitor;
use crate::textplan::printer::plan_printer::{PlanPrinter, TextPlanFormat};
use crate::textplan::symbol_table::SymbolTable;

/// Loads a binary Substrait plan.
///
/// # Arguments
///
/// * `bytes` - The binary plan to load.
///
/// # Returns
///
/// The deserialized Substrait plan.
fn load_plan_from_binary(bytes: &[u8]) -> Result<substrait::Plan, TextPlanError> {
    // Deserialize the binary data to a Plan
    substrait::Plan::decode(bytes)
        .map_err(|e| TextPlanError::ProtobufError(format!("Failed to parse binary plan: {}", e)))
}

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
    let plan = load_plan_from_binary(bytes)?;
    
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
fn convert_plan_to_text(plan: &substrait::Plan) -> Result<String, TextPlanError> {
    // Start building the textplan string
    let mut textplan = String::new();
    
    // Add version information
    if let Some(version) = &plan.version {
        textplan.push_str(&format!("// Substrait plan version: {}.{}.{}\n",
            version.major_number, version.minor_number, version.patch_number));
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
                use substrait::extensions::simple_extension_declaration::MappingType;
                match mapping_type {
                    MappingType::ExtensionFunction(func) => {
                        textplan.push_str(&format!("// - URI Ref: {}, Function: {}, Name: {}\n", 
                            func.extension_uri_reference, func.function_anchor, func.name));
                    },
                    MappingType::ExtensionType(typ) => {
                        textplan.push_str(&format!("// - URI Ref: {}, Type: {}, Name: {}\n", 
                            typ.extension_uri_reference, typ.type_anchor, typ.name));
                    },
                    MappingType::ExtensionTypeVariation(var) => {
                        textplan.push_str(&format!("// - URI Ref: {}, Type Variation: {}, Name: {}\n", 
                            var.extension_uri_reference, var.type_variation_anchor, var.name));
                    },
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
fn process_plan_with_visitor(plan: &substrait::Plan) -> Result<String, TextPlanError> {
    // Create a symbol table for the plan
    let symbol_table = SymbolTable::new();
    
    // Create a pipeline visitor with the symbol table
    let mut visitor = PipelineVisitor::new(symbol_table);
    
    // Visit the plan to build the symbol table
    visitor.visit_plan(plan)?;
    
    // Get the populated symbol table from the visitor
    let symbol_table = visitor.symbol_table().clone();
    
    // Create a plan printer and use it to convert the symbol table to a textplan
    let mut printer = PlanPrinter::new(TextPlanFormat::Standard);
    let plan_text = printer.print_plan(&symbol_table)?;
    
    Ok(plan_text)
}