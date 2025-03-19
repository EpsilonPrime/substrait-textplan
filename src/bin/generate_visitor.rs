// SPDX-License-Identifier: Apache-2.0

//! Command-line tool to generate BasePlanProtoVisitor code from Substrait protobuf schema.
//! 
//! Usage: generate_visitor <proto_dir> <output_path>
//! 
//! This tool parses Substrait protobuf schema and generates a Rust implementation
//! of BasePlanProtoVisitor trait that can be used to traverse and process Substrait plans.
//! 
//! The tool uses a two-phase approach:
//! 1. Discovery phase: Parse all protobuf files to build a type inventory
//! 2. Generation phase: Generate visitor code based on the discovered types

use std::env;
use std::error::Error;
use std::path::PathBuf;
use substrait_textplan::textplan::converter::visitor_generator;

fn main() -> Result<(), Box<dyn Error>> {
    // Parse command line arguments
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 3 {
        eprintln!("Usage: {} <proto_dir> <output_path>", args[0]);
        eprintln!("");
        eprintln!("Example:");
        eprintln!("  {} third_party/substrait/proto src/textplan/converter/generated/plan_visitor.rs", args[0]);
        eprintln!("");
        eprintln!("This tool generates a visitor implementation for Substrait protocol buffers.");
        eprintln!("It follows a two-phase approach:");
        eprintln!("1. Discovery: Parse proto files and build a type inventory");
        eprintln!("2. Generation: Create visitor code based on discovered types");
        return Ok(());
    }
    
    let proto_dir = PathBuf::from(&args[1]);
    let output_path = PathBuf::from(&args[2]);
    
    if !proto_dir.exists() || !proto_dir.is_dir() {
        eprintln!("Error: Proto directory '{}' does not exist or is not a directory", 
            proto_dir.display());
        return Ok(());
    }
    
    // Make sure the output path's parent directory exists
    if let Some(parent) = output_path.parent() {
        if !parent.exists() {
            eprintln!("Error: Output directory '{}' does not exist", parent.display());
            return Ok(());
        }
    }
    
    println!("Generating visitor code from Substrait protobuf schema");
    println!("Proto directory: {}", proto_dir.display());
    println!("Output path: {}", output_path.display());
    
    // Create and run the visitor generator
    let mut generator = visitor_generator::VisitorGenerator::new(&proto_dir, &output_path);
    
    // Run the generator with our new two-phase approach
    println!("Phase 1: Discovering protobuf types...");
    generator.run().map_err(|e| Box::<dyn Error>::from(format!("Error generating visitor: {}", e)))?;
    
    println!("Visitor code generation complete");
    
    Ok(())
}