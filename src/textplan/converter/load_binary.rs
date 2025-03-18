// SPDX-License-Identifier: Apache-2.0

//! Load a binary Substrait plan and convert it to a textplan.

use crate::proto::{load_plan_from_binary, relation_type_to_string};
use crate::textplan::common::error::TextPlanError;

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
                use crate::proto::substrait::extensions::simple_extension_declaration::MappingType;
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
    
    // Add each relation to the textplan
    // In the real implementation, we would recursively traverse the relation tree
    // But for a simple example, we'll just handle the top-level relations
    let mut root_names = Vec::new();
    
    for (idx, plan_rel) in plan.relations.iter().enumerate() {
        // The root relation is special
        if let Some(rel_type) = &plan_rel.rel_type {
            use crate::proto::substrait::plan_rel::RelType;
            
            match rel_type {
                RelType::Root(rel_root) => {
                    textplan.push_str("ROOT {\n");
                    if !rel_root.names.is_empty() {
                        textplan.push_str("    NAMES = [");
                        for (i, name) in rel_root.names.iter().enumerate() {
                            if i > 0 {
                                textplan.push_str(", ");
                            }
                            textplan.push_str(name);
                            root_names.push(name.clone());
                        }
                        textplan.push_str("]\n");
                    }
                    textplan.push_str("}\n\n");
                },
                RelType::Rel(rel) => {
                    // Regular relations
                    let relation_name = format!("relation_{}", idx);
                    let relation_type = relation_type_to_string(rel);
                    
                    textplan.push_str(&format!("{} RELATION {} {{\n", relation_type, relation_name));
                    
                    // In a more complete implementation, we would traverse the relation
                    // and extract and format all of its properties
                    
                    textplan.push_str("}\n\n");
                }
            }
        }
    }
    
    // If no ROOT was found, but we have relations, create a default ROOT
    if root_names.is_empty() && !plan.relations.is_empty() {
        textplan.push_str("ROOT {\n");
        textplan.push_str("    NAMES = [relation_0]\n");
        textplan.push_str("}\n");
    }
    
    Ok(textplan)
}