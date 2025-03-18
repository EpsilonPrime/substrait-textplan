// SPDX-License-Identifier: Apache-2.0

//! The textplan module contains code for parsing and generating Substrait plans.

pub mod common;
pub mod converter;
pub mod parser;
pub mod symbol_table;

#[cfg(test)]
mod tests;

// Re-export common types
pub use common::location::Location;
pub use common::parse_result::ParseResult;
pub use symbol_table::{SymbolInfo, SymbolTable, SymbolType};