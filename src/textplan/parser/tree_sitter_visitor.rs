// SPDX-License-Identifier: Apache-2.0

//! Visitor for tree-sitter parse trees.

use tree_sitter::Node;

use crate::textplan::common::location::Location;
use crate::textplan::parser::plan_visitor::{NodeType, ParseNode};
use crate::textplan::symbol_table::{SymbolTable, SymbolType};

/// Converts a tree-sitter node to our ParseNode structure.
pub fn convert_tree_sitter_node(node: &Node, source: &str) -> Result<ParseNode, String> {
    let node_type = map_node_type(node.kind());
    
    // Extract the node text
    let node_text = match node.utf8_text(source.as_bytes()) {
        Ok(text) => text.to_string(),
        Err(_) => return Err(format!("Failed to extract text for node: {}", node.kind())),
    };
    
    // Convert tree-sitter position to our location format (position and length)
    // We use the byte offset as the position and calculate the length
    let position = node.start_byte() as i32;
    let length = (node.end_byte() - node.start_byte()) as i32;
    let location = Location::new(position, length);
    
    // Create the parse node
    let mut parse_node = ParseNode::new(node_type, node_text, location);
    
    // Process the node's children recursively
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        // Skip unnamed nodes (like punctuation)
        if child.is_named() {
            // Special handling for nested nodes based on ANTLR grammar structure
            match child.kind() {
                // Skip nodes that are just structural and don't need representation
                "comment" => continue,
                _ => {
                    if let Ok(child_node) = convert_tree_sitter_node(&child, source) {
                        parse_node.add_child(child_node);
                    }
                }
            }
        }
    }
    
    Ok(parse_node)
}

/// Maps a tree-sitter node type to our NodeType.
fn map_node_type(ts_type: &str) -> NodeType {
    match ts_type {
        "plan" => NodeType::Plan,
        "schema_definition" => NodeType::Schema,
        "source_definition" => NodeType::Source,
        "relation" => NodeType::Relation,
        "root_relation" => NodeType::Root,
        "extension_space" => NodeType::ExtensionSpace,
        "function" => NodeType::Function,
        "expression" => NodeType::Expression,
        "constant" => NodeType::Constant,
        "identifier" => NodeType::Identifier,
        // Map other types to Other with the type name
        _ => NodeType::Other(ts_type.to_string()),
    }
}

/// Extracts identifiers from a tree-sitter node.
pub fn extract_identifiers(node: &Node, source: &str) -> Vec<String> {
    let mut identifiers = Vec::new();
    
    // If this node is an identifier, add it
    if node.kind() == "identifier" {
        if let Ok(text) = node.utf8_text(source.as_bytes()) {
            identifiers.push(text.to_string());
        }
    }
    
    // Check all children
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        identifiers.extend(extract_identifiers(&child, source));
    }
    
    identifiers
}

/// Builds a symbol table from a tree-sitter parse tree.
pub fn build_symbol_table(root: &Node, source: &str) -> SymbolTable {
    let mut symbol_table = SymbolTable::new();
    
    // Helper to process each node type
    fn process_node(node: &Node, source: &str, symbol_table: &mut SymbolTable) {
        match node.kind() {
            "schema_definition" => {
                // Find the identifier child
                let mut cursor = node.walk();
                for child in node.children(&mut cursor) {
                    if child.kind() == "identifier" {
                        if let Ok(name) = child.utf8_text(source.as_bytes()) {
                            // Create a schema symbol
                            symbol_table.define_symbol(
                                name.to_string(),
                                Location::default(),
                                SymbolType::Schema,
                                None,
                                None
                            );
                        }
                        break;
                    }
                }
            }
            "source_definition" => {
                // Process read_type children to find source names
                let mut cursor = node.walk();
                for child in node.children(&mut cursor) {
                    if child.kind() == "local_files" || 
                       child.kind() == "virtual_table" || 
                       child.kind() == "named_table" || 
                       child.kind() == "extension_table" {
                        // Find the identifier (name) for this source
                        let mut source_cursor = child.walk();
                        for source_child in child.children(&mut source_cursor) {
                            if source_child.kind() == "identifier" {
                                if let Ok(name) = source_child.utf8_text(source.as_bytes()) {
                                    // Create a source symbol
                                    symbol_table.define_symbol(
                                        name.to_string(),
                                        Location::default(),
                                        SymbolType::Source,
                                        None,
                                        None
                                    );
                                }
                                break;
                            }
                        }
                    }
                }
            }
            "relation" => {
                // Find the relation_ref to get the relation name
                let mut cursor = node.walk();
                for child in node.children(&mut cursor) {
                    if child.kind() == "relation_ref" {
                        let mut ref_cursor = child.walk();
                        for ref_child in child.children(&mut ref_cursor) {
                            if ref_child.kind() == "identifier" {
                                if let Ok(name) = ref_child.utf8_text(source.as_bytes()) {
                                    // Create a relation symbol
                                    symbol_table.define_symbol(
                                        name.to_string(),
                                        Location::default(),
                                        SymbolType::Relation,
                                        None,
                                        None
                                    );
                                }
                                break;
                            }
                        }
                        break;
                    }
                }
            }
            "root_relation" => {
                // Extract relation names from the root declaration
                let mut cursor = node.walk();
                for child in node.children(&mut cursor) {
                    if child.kind() == "identifier" {
                        if let Ok(name) = child.utf8_text(source.as_bytes()) {
                            // Create a root symbol
                            symbol_table.define_symbol(
                                name.to_string(),
                                Location::default(),
                                SymbolType::Root,
                                None,
                                None
                            );
                        }
                    }
                }
            }
            _ => {}
        }
        
        // Process all children recursively
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            process_node(&child, source, symbol_table);
        }
    }
    
    // Start processing from the root
    process_node(root, source, &mut symbol_table);
    
    symbol_table
}