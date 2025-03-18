// SPDX-License-Identifier: Apache-2.0

//! Grammar module for the ANTLR4 parser.

use crate::textplan::parser::antlr::substraitplanparser::SubstraitPlanParserContextType;
use std::path::Path;
use std::sync::Arc;
use std::rc::Rc;
use std::fs;
use crate::textplan::common::location::Location;
use crate::textplan::parser::error_listener::ErrorListener;
use antlr_rust::{
    common_token_stream::CommonTokenStream,
    input_stream::InputStream,
    token_factory::CommonTokenFactory,
    parser::Parser,
    DefaultErrorStrategy,
};
use crate::textplan::parser::antlr::{
    SubstraitPlanLexer,
    SubstraitPlanParser,
};

/// Options for the ANTLR parser.
pub struct AntlrParserOptions {
    /// The directory containing the grammar files.
    pub grammar_dir: String,
}

impl Default for AntlrParserOptions {
    fn default() -> Self {
        Self {
            grammar_dir: "src/substrait/textplan/parser/grammar".to_string(),
        }
    }
}

// Re-export PlanContext for simplicity
pub use crate::textplan::parser::antlr::substraitplanparser::PlanContext;
use crate::textplan::parser::error_listener::AntlrErrorListener;
use crate::textplan::parser::error_listener::create_boxed_error_listener;

/// Result from parsing a string using ANTLR.
/// This represents the parse tree from the ANTLR parser.
pub struct ParseResult {
    /// The root node of the parse tree, which is the plan context.
    /// This is what will be passed to our visitors for processing.
    pub plan_ctx: Option<Box<PlanContext<'static>>>,
    /// The error listener that captured any syntax errors.
    pub error_listener: Arc<ErrorListener>,
}

/// Parses a string using ANTLR.
///
/// Creates a lexer, parser, and runs the parsing process on the input text.
///
/// # Arguments
///
/// * `text` - The text to parse.
///
/// # Returns
///
/// A result containing either a parsed tree or an error message.
pub fn parse_string(text: &str) -> Result<ParseResult, String> {
    // Create an error listener
    let error_listener = Arc::new(ErrorListener::new());
    
    // Create a token factory - this needs to outlive the lexer
    let tf = CommonTokenFactory::default();
    
    // Create a lexer
    let lexer_result = create_lexer(text, error_listener.clone(), &tf);
    
    // If lexer creation fails, return the error
    let mut lexer = match lexer_result {
        Ok(mut lexer) => {
            // Create an ANTLR error listener
            let boxed_error_listener = create_boxed_error_listener(error_listener.clone());
            
            // Add the error listener to the lexer
            lexer.remove_error_listeners();
            lexer.add_error_listener(boxed_error_listener);
            
            lexer
        },
        Err(err) => return Err(err),
    };
    
    // Create a token stream from the lexer
    let token_stream = CommonTokenStream::new(lexer);
    
    // Create a parser with the token stream and default error strategy
    let error_strategy = DefaultErrorStrategy::new();
    let mut parser = SubstraitPlanParser::with_strategy(token_stream, error_strategy);
    
    // Create an ANTLR error listener for the parser
    let boxed_error_listener = create_boxed_error_listener(error_listener.clone());
    
    // Add the error listener to the parser
    parser.remove_error_listeners();
    parser.add_error_listener(boxed_error_listener);
    
    // Call the parser's plan() method to get the parse tree
    let plan_result = match parser.plan() {
        Ok(plan_ctx) => plan_ctx,
        Err(e) => {
            // If we get a parsing error, add it to our error listener
            error_listener.add_error(
                format!("Parser error: {}", e),
                Location::new(0, 0) // We don't have specific location for this error
            );
            // Return early with the error
            return Err(format!("Parser error: {}", e));
        }
    };
    
    // Check for errors collected by our custom error listener
    if error_listener.has_errors() {
        let error_messages = error_listener.format_errors();
        return Err(format!("Parsing errors: {}", error_messages.join(", ")));
    }
    
    // Return the parse result with the plan context
    Ok(ParseResult {
        // Convert the Rc<PlanContext> to a Box<PlanContext> for ownership transfer
        // We use unsafe here because we need to convert a reference-counted pointer to a Box
        // This works because we're the only owner of the Rc at this point
        plan_ctx: Some(unsafe {
            let rc_ptr = Rc::into_raw(plan_result);
            Box::from_raw(rc_ptr as *mut PlanContext)
        }),
        error_listener,
    })
}

/// Creates a new lexer for the given input text.
///
/// # Arguments
///
/// * `text` - The text to tokenize.
/// * `error_listener` - The error listener to use.
/// * `tf` - The token factory to use for the lexer.
///
/// # Returns
///
/// A result containing either a lexer or an error message.
pub fn create_lexer<'input, 'tf>(
    text: &'input str, 
    error_listener: Arc<ErrorListener>,
    tf: &'tf CommonTokenFactory
) -> Result<SubstraitPlanLexer<'input, InputStream<&'input str>>, String> 
where
    'tf: 'input  // This constraint ensures that tf lives at least as long as input
{
    let input = InputStream::new(text);
    let mut lexer = SubstraitPlanLexer::new_with_token_factory(input, tf);
    
    // Create an ANTLR error listener that will forward errors to our ErrorListener
    let antlr_error_listener = AntlrErrorListener::new(error_listener.clone());
    
    // Add the error listener to the lexer
    lexer.add_error_listener(Box::new(antlr_error_listener));
    
    Ok(lexer)
}

/// Creates a new parser for the given input text.
/// 
/// # Arguments
///
/// * `token_stream` - The token stream to parse
/// * `error_listener` - The error listener to use.
///
/// # Returns
///
/// A result containing either a parser or an error message.
pub fn create_parser<'input>(
    token_stream: CommonTokenStream<'input, SubstraitPlanLexer<'input, InputStream<&'input str>>>,
    error_listener: Arc<ErrorListener>
) -> Result<SubstraitPlanParser<'input, CommonTokenStream<'input, SubstraitPlanLexer<'input, InputStream<&'input str>>>, DefaultErrorStrategy<'input, SubstraitPlanParserContextType>>, String> {
    // Create a parser with the token stream and default error strategy
    let error_strategy = DefaultErrorStrategy::new();
    let mut parser = SubstraitPlanParser::with_strategy(token_stream, error_strategy);
    
    // Create an ANTLR error listener for the parser
    let boxed_error_listener = create_boxed_error_listener(error_listener);
    
    // Add the error listener to the parser
    parser.remove_error_listeners();
    parser.add_error_listener(boxed_error_listener);
    
    Ok(parser)
}

/// Reads grammar files from the specified directory.
///
/// This function is useful for diagnostics and debugging.
///
/// # Arguments
///
/// * `options` - Options for the parser, including the grammar directory.
///
/// # Returns
///
/// A result containing either a tuple of (lexer_grammar, parser_grammar) or an error message.
pub fn read_grammar_files(options: &AntlrParserOptions) -> Result<(String, String), String> {
    let grammar_dir = Path::new(&options.grammar_dir);
    
    // Read lexer grammar
    let lexer_path = grammar_dir.join("SubstraitPlanLexer.g4");
    let lexer_grammar = fs::read_to_string(&lexer_path)
        .map_err(|e| format!("Failed to read lexer grammar: {}", e))?;
        
    // Read parser grammar
    let parser_path = grammar_dir.join("SubstraitPlanParser.g4");
    let parser_grammar = fs::read_to_string(&parser_path)
        .map_err(|e| format!("Failed to read parser grammar: {}", e))?;
        
    Ok((lexer_grammar, parser_grammar))
}

/// Checks if a parse tree has any syntax errors.
///
/// # Arguments
///
/// * `error_listener` - The error listener to check.
///
/// # Returns
///
/// `true` if the error listener has any errors, `false` otherwise.
pub fn has_errors(error_listener: &ErrorListener) -> bool {
    error_listener.has_errors()
}