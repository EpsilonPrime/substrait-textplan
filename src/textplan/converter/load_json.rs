// SPDX-License-Identifier: Apache-2.0

//! Load a JSON Substrait plan and convert it to a textplan.

use std::fs;
use std::path::Path;

use crate::proto::{Plan, load_plan_from_json};
use crate::textplan::common::error::TextPlanError;
use crate::textplan::converter::load_binary::load_from_binary;

/// Loads a JSON Substrait plan and converts it to a Plan protobuf.
///
/// # Arguments
///
/// * `json_str` - The JSON string containing the plan.
///
/// # Returns
///
/// The Plan protobuf representation of the JSON.
pub fn load_plan_from_json_str(json_str: &str) -> Result<Plan, TextPlanError> {
    load_plan_from_json(json_str)
}

/// Loads a JSON Substrait plan from a file and converts it to a Plan protobuf.
///
/// # Arguments
///
/// * `file_path` - The path to the JSON file.
///
/// # Returns
///
/// The Plan protobuf representation of the JSON file.
pub fn load_from_json_file<P: AsRef<Path>>(file_path: P) -> Result<Plan, TextPlanError> {
    // Read the file
    let json_str = fs::read_to_string(&file_path)
        .map_err(|e| TextPlanError::IoError(e))?;
    
    // Parse the JSON
    load_plan_from_json_str(&json_str)
}

/// Loads a JSON Substrait plan and converts it to a textplan.
///
/// # Arguments
///
/// * `json` - The JSON plan to load.
///
/// # Returns
///
/// The textplan representation of the plan.
pub fn load_from_json(json: &str) -> Result<String, TextPlanError> {
    // Deserialize the JSON data to a Plan
    let plan = load_plan_from_json_str(json)?;
    
    // Convert the plan to binary
    let binary = crate::proto::save_plan_to_binary(&plan)?;
    
    // Use the binary to textplan converter
    load_from_binary(&binary)
}