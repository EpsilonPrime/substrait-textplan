// SPDX-License-Identifier: Apache-2.0

//! Save a Substrait plan to JSON format.

use std::fs;
use std::path::Path;

use crate::proto::{Plan, save_plan_to_json};
use crate::textplan::common::error::TextPlanError;
use crate::textplan::symbol_table::SymbolTable;

/// Saves a Substrait plan to JSON format.
///
/// # Arguments
///
/// * `plan` - The plan to save.
///
/// # Returns
///
/// The JSON representation of the plan.
pub fn save_to_json(plan: &Plan) -> Result<String, TextPlanError> {
    save_plan_to_json(plan)
}

/// Saves a Substrait plan to a JSON file.
///
/// # Arguments
///
/// * `plan` - The plan to save.
/// * `file_path` - The path to the JSON file.
///
/// # Returns
///
/// A result indicating success or failure.
pub fn save_to_json_file<P: AsRef<Path>>(plan: &Plan, file_path: P) -> Result<(), TextPlanError> {
    let json_str = save_to_json(plan)?;
    
    fs::write(&file_path, json_str)
        .map_err(|e| TextPlanError::IoError(e))
}

/// Saves a symbol table to JSON format by first converting it to a Plan.
///
/// # Arguments
///
/// * `symbol_table` - The symbol table to save.
///
/// # Returns
///
/// The JSON representation of the plan.
pub fn symbol_table_to_json(symbol_table: &SymbolTable) -> Result<String, TextPlanError> {
    // Convert the symbol table to a Plan
    let plan = crate::textplan::converter::save_binary::create_plan_from_symbol_table(symbol_table)?;
    
    // Convert the Plan to JSON
    save_to_json(&plan)
}