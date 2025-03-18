// SPDX-License-Identifier: Apache-2.0

//! This crate provides Substrait TextPlan language support for the [tree-sitter][] parsing library.
//!
//! Typically, you will use the [language][language func] function to add this language to a
//! tree-sitter [Parser][], and then use the parser to parse some code:
//!
//! ```
//! let code = r#"
//! schema simple_schema {
//!     id i32;
//!     name string;
//! }
//! 
//! read RELATION simple_read {
//!     SOURCE simple_source;
//!     BASE_SCHEMA simple_schema;
//! }
//! "#;
//! let mut parser = tree_sitter::Parser::new();
//! parser.set_language(tree_sitter_substrait::language()).expect("Error loading Substrait TextPlan grammar");
//! let parsed = parser.parse(code, None).unwrap();
//! let root = parsed.root_node();
//! assert!(!root.has_error());
//! ```
//!
//! [Language]: https://docs.rs/tree-sitter/*/tree_sitter/struct.Language.html
//! [language func]: fn.language.html
//! [Parser]: https://docs.rs/tree-sitter/*/tree_sitter/struct.Parser.html
//! [tree-sitter]: https://tree-sitter.github.io/

use tree_sitter::Language;

extern "C" {
    fn tree_sitter_substrait() -> Language;
}

/// Get the tree-sitter [Language][] for this grammar.
///
/// [Language]: https://docs.rs/tree-sitter/*/tree_sitter/struct.Language.html
pub fn language() -> Language {
    unsafe { tree_sitter_substrait() }
}

/// The source of the Substrait TextPlan tree-sitter grammar.
pub const GRAMMAR: &str = include_str!("../../grammar.js");

/// The content of the [`node-types.json`][] file for this grammar.
///
/// [`node-types.json`]: https://tree-sitter.github.io/tree-sitter/using-parsers#static-node-types
pub const NODE_TYPES: &str = include_str!("../../src/node-types.json");

/// The syntax highlighting query for this language.
pub const HIGHLIGHTS_QUERY: &str = include_str!("../../queries/highlights.scm");

/// The local variable syntax highlighting query for this language.
pub const LOCALS_QUERY: &str = include_str!("../../queries/locals.scm");

#[cfg(test)]
mod tests {
    #[test]
    fn test_can_load_grammar() {
        let mut parser = tree_sitter::Parser::new();
        parser
            .set_language(super::language())
            .expect("Error loading Substrait TextPlan grammar");
    }
}