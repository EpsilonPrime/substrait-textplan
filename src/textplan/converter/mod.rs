// SPDX-License-Identifier: Apache-2.0

//! Converter for binary and JSON Substrait plans.

pub mod load_binary;
pub mod save_binary;
pub mod load_json;
pub mod save_json;

// Re-export the main functions
pub use load_binary::load_from_binary;
pub use save_binary::save_to_binary;
pub use load_json::load_from_json;
pub use save_json::save_to_json;