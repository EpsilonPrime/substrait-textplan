//! ANTLR4-generated code for parsing Substrait TextPlan files.
//! 
//! This module contains the generated Rust code from the ANTLR4 grammar files.
//! It provides the lexer, parser, and visitor implementations for parsing
//! Substrait TextPlan files.

// Re-export the generated code
pub mod substraitplanlexer;
pub mod substraitplanparser;
pub mod substraitplanparservisitor;
pub mod substraitplanparserlistener;

// Re-export the main types
pub use substraitplanlexer::SubstraitPlanLexer;
pub use substraitplanparser::SubstraitPlanParser;
pub use substraitplanparservisitor::SubstraitPlanParserVisitor;
pub use substraitplanparserlistener::SubstraitPlanParserListener;

// Common types from antlr-rust that are needed when using the parser
pub use antlr_rust::{
    common_token_stream::CommonTokenStream,
    input_stream::InputStream,
    token_factory::CommonTokenFactory,
    token::Token,
    token_factory::TokenFactory,
};