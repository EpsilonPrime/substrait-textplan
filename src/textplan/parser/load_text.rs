// SPDX-License-Identifier: Apache-2.0

//! Loads a textplan from a string and converts it to a binary protobuf.

use crate::textplan::common::error::TextPlanError;
use crate::textplan::converter::save_binary::save_to_binary;
use crate::textplan::parser::parse_text::parse_stream;

/// Validates textplan syntax and provides helpful error messages for common mistakes.
///
/// # Arguments
///
/// * `text` - The textplan text to validate.
///
/// # Returns
///
/// An optional error message with suggestions if validation fails.
fn validate_textplan_syntax(text: &str) -> Option<String> {
    let trimmed = text.trim();

    // Check for empty input
    if trimmed.is_empty() {
        return None; // Empty input is allowed
    }

    let mut suggestions = Vec::new();

    // Convert to lowercase for case-insensitive checking
    let lower = trimmed.to_lowercase();

    // Check if this looks like printer-generated output (has ROOT but not invalid)
    // The printer generates "ROOT { NAMES = [...] }" which is valid legacy syntax
    let has_root_block = trimmed.contains("ROOT {");
    let has_pipelines = lower.contains("pipelines");

    // If it has ROOT syntax but also has proper structure, it's likely printer output
    // Only warn about ROOT if it seems malformed (e.g., user trying to use old syntax incorrectly)
    let is_likely_printer_output =
        has_root_block && lower.contains("schema") && lower.contains("relation");

    // Check for common user errors (but not printer-generated patterns)
    if !is_likely_printer_output {
        // Check for uppercase keywords combined with missing pipelines (common user error)
        let has_uppercase_keywords = trimmed.contains("RELATION")
            || trimmed.contains("SOURCE")
            || trimmed.contains("SCHEMA")
            || trimmed.contains("PIPELINES");

        if has_uppercase_keywords && !has_pipelines && !has_root_block {
            suggestions.push("Keywords should be lowercase".to_string());
        }

        // Check for missing pipelines section ONLY when there are relations
        // (schema-only or type-only textplans don't need pipelines)
        let has_relation = lower.contains("relation");

        if has_relation && !has_pipelines && !has_root_block {
            suggestions.push("Relations require 'pipelines { }' or 'ROOT { }' section".to_string());
        }

        // Check for LOCAL_FILES (old syntax that printer doesn't use)
        if trimmed.contains("LOCAL_FILES") {
            suggestions.push("Use 'source named_table' instead of 'LOCAL_FILES'".to_string());
        }

        // Check for ITEMS (old syntax that printer doesn't use)
        if trimmed.contains("ITEMS =") {
            suggestions.push("Use 'names = [...]' instead of 'ITEMS = [...]'".to_string());
        }
    }

    // Return suggestions if any were found
    if !suggestions.is_empty() {
        return Some(suggestions.join("; "));
    }

    None
}

/// Loads a textplan from a string and converts it to a binary protobuf.
///
/// # Arguments
///
/// * `text` - The textplan to load.
///
/// # Returns
///
/// The binary protobuf representation of the plan.
pub fn load_from_text(text: &str) -> Result<Vec<u8>, TextPlanError> {
    // Skip validation for textplans that contain ROOT (printer-generated output)
    // The printer may generate syntax that our validation would flag but the parser accepts
    if !text.contains("ROOT {") {
        // Only validate user-written textplans
        if let Some(validation_error) = validate_textplan_syntax(text) {
            return Err(TextPlanError::ParseError(validation_error));
        }
    }

    let parse_result = parse_stream(text);

    if !parse_result.successful() {
        let errors = parse_result.all_errors();
        let error_msg = errors.join("; ");
        return Err(TextPlanError::ParseError(error_msg));
    }

    // Convert the symbol table to a protobuf plan
    save_to_binary(parse_result.symbol_table())
}
