// SPDX-License-Identifier: Apache-2.0

//! PlanPrinter for converting a Substrait Symbol Table to a textplan string.

use std::collections::{HashMap, HashSet};
use std::sync::Arc;

use crate::textplan::common::error::TextPlanError;
use crate::textplan::symbol_table::{RelationType, SymbolInfo, SymbolTable, SymbolType};

/// Format options for the text plan output.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TextPlanFormat {
    /// Standard format with standard indentation
    Standard,
    /// Compact format with minimal whitespace
    Compact,
    /// Verbose format with additional details and comments
    Verbose,
}

/// A printer for converting a symbol table into a textplan.
///
/// The PlanPrinter takes a symbol table that has been populated either by
/// parsing a textplan or by converting a binary Substrait plan, and
/// generates a textplan string representation.
pub struct PlanPrinter {
    /// The format to use for the output
    format: TextPlanFormat,
    /// Whether to include comments in the output
    include_comments: bool,
    /// Indentation level (number of spaces per level)
    indent_size: usize,
    /// Maps relation symbols to their text representations
    relation_text_cache: HashMap<String, String>,
}

impl Default for PlanPrinter {
    fn default() -> Self {
        Self::new(TextPlanFormat::Standard)
    }
}

impl PlanPrinter {
    /// Creates a new PlanPrinter with the specified format.
    pub fn new(format: TextPlanFormat) -> Self {
        let (indent_size, include_comments) = match format {
            TextPlanFormat::Standard => (4, true),
            TextPlanFormat::Compact => (2, false),
            TextPlanFormat::Verbose => (4, true),
        };

        Self {
            format,
            include_comments,
            indent_size,
            relation_text_cache: HashMap::new(),
        }
    }

    /// Generates a textplan from a symbol table.
    ///
    /// # Arguments
    ///
    /// * `symbol_table` - The symbol table to convert
    ///
    /// # Returns
    ///
    /// The textplan as a string
    pub fn print_plan(&mut self, symbol_table: &SymbolTable) -> Result<String, TextPlanError> {
        let mut result = String::new();

        // First, clear any cached data from previous runs
        self.relation_text_cache.clear();

        // Add header information if verbose
        if self.format == TextPlanFormat::Verbose {
            result.push_str("// Substrait TextPlan\n");
            result.push_str(&format!(
                "// Generated symbol count: {}\n\n",
                symbol_table.len()
            ));
        }

        // Process ROOT relations first
        self.process_root_relations(symbol_table, &mut result)?;

        // Process all other relations
        self.process_relations(symbol_table, &mut result)?;

        Ok(result)
    }

    /// Processes ROOT relations in the symbol table.
    ///
    /// # Arguments
    ///
    /// * `symbol_table` - The symbol table to process
    /// * `result` - The string to append the result to
    ///
    /// # Returns
    ///
    /// Result indicating success or an error
    fn process_root_relations(
        &mut self,
        symbol_table: &SymbolTable,
        result: &mut String,
    ) -> Result<(), TextPlanError> {
        // Find all ROOT symbols
        let root_symbols: Vec<_> = symbol_table
            .symbols()
            .iter()
            .filter(|s| s.symbol_type() == SymbolType::Root)
            .cloned()
            .collect();

        if root_symbols.is_empty() {
            // If no root symbols, add a comment
            if self.include_comments {
                result.push_str("// No ROOT relations found\n\n");
            }
            return Ok(());
        }

        // Process each root symbol
        for root in root_symbols {
            self.print_root_relation(&root, symbol_table, result)?;
        }

        // Add a blank line after all roots
        result.push('\n');

        Ok(())
    }

    /// Prints a ROOT relation.
    ///
    /// # Arguments
    ///
    /// * `root` - The root symbol to print
    /// * `symbol_table` - The symbol table for looking up references
    /// * `result` - The string to append the result to
    ///
    /// # Returns
    ///
    /// Result indicating success or an error
    fn print_root_relation(
        &self,
        root: &Arc<SymbolInfo>,
        _symbol_table: &SymbolTable,
        result: &mut String,
    ) -> Result<(), TextPlanError> {
        // Start the ROOT block
        result.push_str("ROOT {\n");

        // Get the indentation for this level
        let indent = " ".repeat(self.indent_size);

        // Add the names of the root relations
        result.push_str(&format!("{}NAMES = [", indent));

        // For a real implementation, we would look up the relations referenced by this root
        // and add their names. For now, just use the root's name as a placeholder.
        result.push_str(&format!("\"{}\"", root.display_name()));

        result.push_str("]\n");

        // End the ROOT block
        result.push_str("}\n");

        Ok(())
    }

    /// Processes all non-ROOT relations in the symbol table.
    ///
    /// # Arguments
    ///
    /// * `symbol_table` - The symbol table to process
    /// * `result` - The string to append the result to
    ///
    /// # Returns
    ///
    /// Result indicating success or an error
    fn process_relations(
        &mut self,
        symbol_table: &SymbolTable,
        result: &mut String,
    ) -> Result<(), TextPlanError> {
        // Find all relation symbols
        let relation_symbols: Vec<_> = symbol_table
            .symbols()
            .iter()
            .filter(|s| s.symbol_type() == SymbolType::Relation)
            .cloned()
            .collect();

        if relation_symbols.is_empty() {
            // If no relation symbols, add a comment
            if self.include_comments {
                result.push_str("// No relations found\n");
            }
            return Ok(());
        }

        // First pass: generate text for all relations and cache it
        for relation in &relation_symbols {
            let relation_text = self.generate_relation_text(relation, symbol_table)?;
            self.relation_text_cache
                .insert(relation.name().to_string(), relation_text);
        }

        // Second pass: append all relation texts to the result in a meaningful order
        let mut processed = HashSet::new();

        // Process relations in dependency order (a more complex implementation would perform
        // topological sorting here)
        for relation in relation_symbols {
            if !processed.contains(relation.name()) {
                if let Some(text) = self.relation_text_cache.get(relation.name()) {
                    result.push_str(text);
                    result.push('\n');
                    processed.insert(relation.name().to_string());
                }
            }
        }

        Ok(())
    }

    /// Generates text representation for a relation.
    ///
    /// # Arguments
    ///
    /// * `relation` - The relation symbol to process
    /// * `symbol_table` - The symbol table for looking up references
    ///
    /// # Returns
    ///
    /// The relation text or an error
    fn generate_relation_text(
        &self,
        relation: &Arc<SymbolInfo>,
        symbol_table: &SymbolTable,
    ) -> Result<String, TextPlanError> {
        let mut result = String::new();

        // Get the relation type
        let rel_type = relation
            .subtype::<RelationType>()
            .cloned()
            .unwrap_or(RelationType::Unknown);

        // Convert the relation type to a string
        let rel_type_str = match rel_type {
            RelationType::Unknown => "UNKNOWN",
            RelationType::Read => "READ",
            RelationType::Project => "PROJECT",
            RelationType::Join => "JOIN",
            RelationType::Cross => "CROSS",
            RelationType::Fetch => "FETCH",
            RelationType::Aggregate => "AGGREGATE",
            RelationType::Sort => "SORT",
            RelationType::Filter => "FILTER",
            RelationType::Set => "SET",
            RelationType::HashJoin => "HASH_JOIN",
            RelationType::MergeJoin => "MERGE_JOIN",
            RelationType::Exchange => "EXCHANGE",
            RelationType::Ddl => "DDL",
            RelationType::Write => "WRITE",
            RelationType::ExtensionLeaf => "EXTENSION_LEAF",
            RelationType::ExtensionSingle => "EXTENSION_SINGLE",
            RelationType::ExtensionMulti => "EXTENSION_MULTI",
        };

        // Start the relation definition
        result.push_str(&format!(
            "{} RELATION {} {{\n",
            rel_type_str,
            relation.display_name()
        ));

        // Get the indentation for this level
        let indent = " ".repeat(self.indent_size);

        // Add relation properties based on relation type
        match rel_type {
            RelationType::Read => {
                self.add_read_relation_properties(relation, symbol_table, &indent, &mut result)?;
            }
            RelationType::Filter => {
                self.add_filter_relation_properties(relation, symbol_table, &indent, &mut result)?;
            }
            // Add cases for other relation types as needed
            _ => {
                // Default case: add a comment for unimplemented relation types
                if self.include_comments {
                    result.push_str(&format!(
                        "{}// Properties for this relation type are not yet fully implemented\n",
                        indent
                    ));
                }
            }
        }

        // End the relation definition
        result.push_str("}\n");

        Ok(result)
    }

    /// Adds properties for a read relation.
    ///
    /// # Arguments
    ///
    /// * `relation` - The relation symbol
    /// * `symbol_table` - The symbol table for looking up references
    /// * `indent` - The indentation string
    /// * `result` - The string to append the result to
    ///
    /// # Returns
    ///
    /// Result indicating success or an error
    fn add_read_relation_properties(
        &self,
        relation: &Arc<SymbolInfo>,
        symbol_table: &SymbolTable,
        indent: &str,
        result: &mut String,
    ) -> Result<(), TextPlanError> {
        // Look for table information
        let table_symbols: Vec<_> = symbol_table
            .symbols()
            .iter()
            .filter(|s| {
                s.symbol_type() == SymbolType::Table
                    && s.name().starts_with(&format!("{}.", relation.name()))
            })
            .cloned()
            .collect();

        if !table_symbols.is_empty() {
            // Add BASE_SCHEMA if available
            if let Some(_schema) = relation.schema() {
                result.push_str(&format!("{}BASE_SCHEMA = {{\n", indent));

                let schema_indent = " ".repeat(indent.len() + self.indent_size);

                // Add schema names if available
                result.push_str(&format!("{}NAMES = [", schema_indent));
                // In a real implementation, we would get the column names from the schema
                result.push_str("\"column1\", \"column2\"");
                result.push_str("]\n");

                result.push_str(&format!("{}}}\n", indent));
            }

            // Process each table
            for table in table_symbols {
                // Get table names from the blob
                table.with_blob::<Vec<String>, _, _>(|names| {
                    result.push_str(&format!("{}SOURCE = NAMED_TABLE {{\n", indent));

                    let source_indent = " ".repeat(indent.len() + self.indent_size);

                    // Add table names
                    result.push_str(&format!("{}NAMES = [", source_indent));
                    for (i, name) in names.iter().enumerate() {
                        if i > 0 {
                            result.push_str(", ");
                        }
                        result.push_str(&format!("\"{}\"", name));
                    }
                    result.push_str("]\n");

                    result.push_str(&format!("{}}}\n", indent));
                });
            }
        }

        // Look for file/folder sources
        let source_symbols: Vec<_> = symbol_table
            .symbols()
            .iter()
            .filter(|s| {
                s.symbol_type() == SymbolType::Source
                    && s.name().starts_with(&format!("{}.", relation.name()))
            })
            .cloned()
            .collect();

        if !source_symbols.is_empty() {
            for source in source_symbols {
                if source.name().contains(".file") {
                    // Process file source
                    source.with_blob::<String, _, _>(|uri| {
                        result.push_str(&format!("{}SOURCE = LOCAL_FILES {{\n", indent));
                        let source_indent = " ".repeat(indent.len() + self.indent_size);
                        result.push_str(&format!("{}ITEMS = [\n", source_indent));
                        let item_indent = " ".repeat(indent.len() + 2 * self.indent_size);
                        result.push_str(&format!("{}{{ URI_FILE = \"{}\" }},\n", item_indent, uri));
                        result.push_str(&format!("{}]\n", source_indent));
                        result.push_str(&format!("{}}}\n", indent));
                    });
                } else if source.name().contains(".folder") {
                    // Process folder source
                    source.with_blob::<String, _, _>(|uri| {
                        result.push_str(&format!("{}SOURCE = LOCAL_FILES {{\n", indent));
                        let source_indent = " ".repeat(indent.len() + self.indent_size);
                        result.push_str(&format!("{}ITEMS = [\n", source_indent));
                        let item_indent = " ".repeat(indent.len() + 2 * self.indent_size);
                        result
                            .push_str(&format!("{}{{ URI_FOLDER = \"{}\" }},\n", item_indent, uri));
                        result.push_str(&format!("{}]\n", source_indent));
                        result.push_str(&format!("{}}}\n", indent));
                    });
                }
            }
        }

        Ok(())
    }

    /// Adds properties for a filter relation.
    ///
    /// # Arguments
    ///
    /// * `relation` - The relation symbol
    /// * `symbol_table` - The symbol table for looking up references
    /// * `indent` - The indentation string
    /// * `result` - The string to append the result to
    ///
    /// # Returns
    ///
    /// Result indicating success or an error
    fn add_filter_relation_properties(
        &self,
        _relation: &Arc<SymbolInfo>,
        _symbol_table: &SymbolTable,
        indent: &str,
        result: &mut String,
    ) -> Result<(), TextPlanError> {
        // In a real implementation, we would add the filter condition and input relation

        // Add a placeholder for the condition
        result.push_str(&format!("{}CONDITION = /* expression */\n", indent));

        // Add a placeholder for the input relation
        result.push_str(&format!("{}INPUT = /* relation reference */\n", indent));

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::textplan::common::unknown_location::UnknownLocation;

    #[test]
    fn test_print_simple_plan() {
        // Create a simple symbol table
        let mut symbol_table = SymbolTable::new();

        // Add a root relation
        symbol_table.add_root_relation("root1");

        // Add a regular relation
        let rel_type = Box::new(RelationType::Read);
        symbol_table.define_symbol(
            "read1".to_string(),
            UnknownLocation::UNKNOWN,
            SymbolType::Relation,
            Some(rel_type),
            None,
        );

        // Add a named table for the read relation
        let table_names = vec![
            "catalog1".to_string(),
            "schema1".to_string(),
            "table1".to_string(),
        ];
        symbol_table.add_named_table("read1", &table_names);

        // Create a printer and generate the plan
        let mut printer = PlanPrinter::new(TextPlanFormat::Standard);
        let plan = printer.print_plan(&symbol_table).unwrap();

        // Verify the plan contains the expected elements
        assert!(plan.contains("ROOT {"));
        assert!(plan.contains("NAMES = [\"root1\"]"));
        assert!(plan.contains("READ RELATION read1 {"));
        assert!(plan.contains("SOURCE = NAMED_TABLE {"));
        assert!(plan.contains("NAMES = [\"catalog1\", \"schema1\", \"table1\"]"));
    }
}
