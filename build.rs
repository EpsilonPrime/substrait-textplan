// SPDX-License-Identifier: Apache-2.0

use std::env;
use std::path::{Path, PathBuf};
use std::fs;
use std::process::Command;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Tell Cargo to re-run this build script if the protos or grammars change
    println!("cargo:rerun-if-changed=third_party/substrait/proto");
    println!("cargo:rerun-if-changed=src/substrait/textplan/parser/grammar");
    println!("cargo:rerun-if-changed=src/textplan/converter/visitor_generator.rs");
    println!("cargo:rerun-if-changed=build.rs");
    
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    
    // Generate protobuf code
    generate_proto_code(&out_dir)?;
    
    // Configure parser
    // Using ANTLR4
    println!("cargo:rustc-cfg=use_antlr4");
    
    // Generate ANTLR code if requested
    // You can use this by setting the GENERATE_ANTLR env var:
    // GENERATE_ANTLR=true cargo build
    if env::var("GENERATE_ANTLR").is_ok() {
        match generate_antlr_code() {
            Ok(_) => println!("cargo:warning=ANTLR code generation completed successfully"),
            Err(e) => println!("cargo:warning=ANTLR code generation failed: {}", e),
        }
    }
    
    // Always generate visitor code
    match generate_visitor_code() {
        Ok(_) => println!("cargo:warning=Visitor code generation completed successfully"),
        Err(e) => println!("cargo:warning=Visitor code generation failed: {}", e),
    }
    
    Ok(())
}

fn generate_proto_code(out_dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
    // Use the actual Substrait protos
    let proto_dir = PathBuf::from("third_party/substrait/proto");
    
    // Define all proto files needed from Substrait
    let protos = [
        "substrait/plan.proto", 
        "substrait/algebra.proto",
        "substrait/type.proto",
        "substrait/function.proto",
        "substrait/capabilities.proto",
        "substrait/parameterized_types.proto",
        "substrait/extensions/extensions.proto",
    ];
    
    // Convert to full paths for compilation
    let proto_files: Vec<PathBuf> = protos.iter().map(|p| proto_dir.join(p)).collect();
    
    // Configure prost with the necessary options
    let mut prost_config = prost_build::Config::new();
    prost_config
        .out_dir(out_dir)
        //.bytes(["."])
        .disable_comments(["."])  // To avoid errors with comments
        .compile_well_known_types()
        .btree_map(["."])
        // Add serde attributes to all types for serialization/deserialization
        .type_attribute(".", "#[derive(serde::Serialize, serde::Deserialize)]");
    
    // Define simplified Empty and Any types directly in the generated code
    // without serde to avoid serialization issues
    prost_config.type_attribute(
        ".google.protobuf.Empty", 
        "#[derive(Clone, PartialEq, ::prost::Message, Default)] pub struct Empty {}"
    );

    prost_config.type_attribute(
        ".google.protobuf.Any", 
        r#"#[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Any {
            #[prost(string, tag = "1")]
            pub type_url: String,
            
            #[prost(bytes, tag = "2")]
            pub value: Vec<u8>,
        }"#
    );
    // Fix serialization for specific bytes fields
    prost_config
        .field_attribute("substrait.Expression.Literal.Decimal.value", "#[serde(with = \"serde_bytes\")]")
        .field_attribute("substrait.DerivationExpression.Rex.script", "#[serde(with = \"serde_bytes\")]")
        .field_attribute("substrait.ExtensionLeafRel.script", "#[serde(with = \"serde_bytes\")]")
        .field_attribute("substrait.UserDefinedAggregateFunction.script", "#[serde(with = \"serde_bytes\")]")
        .field_attribute("substrait.UserDefinedScalarFunction.script", "#[serde(with = \"serde_bytes\")]")
        .field_attribute("substrait.Expression.Literal.String.value", "#[serde(with = \"serde_bytes\")]")
        .field_attribute("substrait.Expression.Literal.Binary.value", "#[serde(with = \"serde_bytes\")]");

    // Map other well-known Google protobuf types to prost_types where supported
    prost_config.extern_path(".google.protobuf.Timestamp", "::prost_types::Timestamp")
                .extern_path(".google.protobuf.Duration", "::prost_types::Duration");

    // Compile the protos directly
    prost_config.compile_protos(&proto_files, &[proto_dir])?;
    
    Ok(())
}

/// Generate ANTLR code using the antlr4 tool with Rust support.
///
/// Note: You must download the special ANTLR4 JAR with Rust support from:
/// https://github.com/rrevenantt/antlr4rust/releases/download/antlr4-4.8-2-Rust-0.3.0-beta/antlr4-4.8-2-SNAPSHOT-complete.jar
///
/// Regular ANTLR4 JAR files DO NOT support generating Rust code!
fn generate_antlr_code() -> Result<(), Box<dyn std::error::Error>> {
    // Path to your ANTLR grammar files
    let grammar_dir = PathBuf::from("src/substrait/textplan/parser/grammar");
    
    // Path to output the generated code
    let output_dir = PathBuf::from("src/textplan/parser/antlr");
    
    // Create the output directory if it doesn't exist
    fs::create_dir_all(&output_dir)?;
    
    // Path to the ANTLR4 jar file with Rust support
    // This must be the special version from antlr4rust repo!
    let antlr_jar = env::var("ANTLR_JAR").ok().or_else(|| {
        // Try to find the JAR in the build-tools directory
        let build_tools_jar = PathBuf::from("build-tools/antlr4rust.jar");
        if build_tools_jar.exists() {
            println!("cargo:warning=Using ANTLR4 JAR found in build-tools directory");
            return Some(build_tools_jar.to_string_lossy().to_string());
        }
        None
    }).unwrap_or_else(|| {
        println!("cargo:warning=ANTLR_JAR environment variable not set!");
        println!("cargo:warning=You must download the special ANTLR4 JAR with Rust support from:");
        println!("cargo:warning=https://github.com/rrevenantt/antlr4rust/releases/download/antlr4-4.8-2-Rust-0.3.0-beta/antlr4-4.8-2-SNAPSHOT-complete.jar");
        println!("cargo:warning=Regular ANTLR4 JAR files DO NOT support generating Rust code!");
        println!("cargo:warning=Using default 'antlr4rust.jar' - make sure this is the correct version with Rust support");
        "antlr4rust.jar".to_string()
    });
    
    // Check if the JAR file exists
    if !PathBuf::from(&antlr_jar).exists() {
        println!("cargo:warning=Could not find ANTLR4 JAR file at '{}'", antlr_jar);
        println!("cargo:warning=Please download it from:");
        println!("cargo:warning=https://github.com/rrevenantt/antlr4rust/releases/download/antlr4-4.8-2-Rust-0.3.0-beta/antlr4-4.8-2-SNAPSHOT-complete.jar");
        println!("cargo:warning=And place it in build-tools/antlr4rust.jar or set ANTLR_JAR environment variable");
        return Err("ANTLR4 JAR file not found".into());
    }
    
    // Run the ANTLR tool to generate the parser code
    println!("cargo:warning=Generating ANTLR code from grammar files...");
    
    // Copy grammar files to a temporary directory for processing
    let temp_dir = env::temp_dir().join("substrait_antlr");
    fs::create_dir_all(&temp_dir)?;
    
    // Copy the grammar files
    let lexer_src = grammar_dir.join("SubstraitPlanLexer.g4");
    let parser_src = grammar_dir.join("SubstraitPlanParser.g4");
    let lexer_dst = temp_dir.join("SubstraitPlanLexer.g4");
    let parser_dst = temp_dir.join("SubstraitPlanParser.g4");
    
    fs::copy(&lexer_src, &lexer_dst)?;
    fs::copy(&parser_src, &parser_dst)?;
    
    println!("cargo:warning=Copied grammar files to {}", temp_dir.display());
    
    // First generate the lexer code
    println!("cargo:warning=Generating lexer code...");
    let lexer_cmd = format!(
        "java -jar {} -Dlanguage=Rust -visitor -o {} {}",
        antlr_jar, 
        output_dir.display(),
        lexer_dst.display()
    );
    println!("cargo:warning=Executing: {}", lexer_cmd);
    
    let lexer_output = Command::new("java")
        .arg("-jar")
        .arg(&antlr_jar)
        .arg("-Dlanguage=Rust")
        .arg("-visitor")
        .arg("-o")
        .arg(&output_dir)
        .arg(&lexer_dst)
        .output()?;
    
    if !lexer_output.status.success() {
        println!("cargo:warning=Lexer generation failed: {}", lexer_output.status);
        println!("cargo:warning=STDOUT: {}", String::from_utf8_lossy(&lexer_output.stdout));
        println!("cargo:warning=STDERR: {}", String::from_utf8_lossy(&lexer_output.stderr));
        return Err("ANTLR lexer generation failed".into());
    }
    
    println!("cargo:warning=Lexer generation succeeded");
    
    // Then generate the parser code
    println!("cargo:warning=Generating parser code...");
    let parser_cmd = format!(
        "java -jar {} -Dlanguage=Rust -visitor -o {} {}", 
        antlr_jar,
        output_dir.display(),
        parser_dst.display()
    );
    println!("cargo:warning=Executing: {}", parser_cmd);
    
    let parser_output = Command::new("java")
        .arg("-jar")
        .arg(&antlr_jar)
        .arg("-Dlanguage=Rust")
        .arg("-visitor")
        .arg("-o")
        .arg(&output_dir)
        .arg(&parser_dst)
        .output()?;
    
    if !parser_output.status.success() {
        println!("cargo:warning=Parser generation failed: {}", parser_output.status);
        println!("cargo:warning=STDOUT: {}", String::from_utf8_lossy(&parser_output.stdout));
        println!("cargo:warning=STDERR: {}", String::from_utf8_lossy(&parser_output.stderr));
        return Err("ANTLR parser generation failed".into());
    }
    
    println!("cargo:warning=Parser generation succeeded");
    println!("cargo:warning=ANTLR code generation completed successfully");
    
    // List the generated files
    println!("cargo:warning=Generated files:");
    if let Ok(entries) = fs::read_dir(&output_dir) {
        for entry in entries {
            if let Ok(entry) = entry {
                println!("cargo:warning=- {}", entry.path().display());
            }
        }
    }
    
    Ok(())
}

/// Copies the ANTLR grammar files to the build directory.
#[allow(dead_code)]
fn copy_grammar_files(out_dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let grammar_dir = PathBuf::from("src/substrait/textplan/parser/grammar");
    let out_grammar_dir = out_dir.join("grammar");
    
    // Create the output directory if it doesn't exist
    fs::create_dir_all(&out_grammar_dir)?;
    
    // Copy the lexer grammar
    let lexer_src = grammar_dir.join("SubstraitPlanLexer.g4");
    let lexer_dst = out_grammar_dir.join("SubstraitPlanLexer.g4");
    fs::copy(lexer_src, lexer_dst)?;
    
    // Copy the parser grammar
    let parser_src = grammar_dir.join("SubstraitPlanParser.g4");
    let parser_dst = out_grammar_dir.join("SubstraitPlanParser.g4");
    fs::copy(parser_src, parser_dst)?;
    
    Ok(())
}

// Include the visitor generator module
mod visitor_generator {
    include!("build-tools/visitor_generator.rs");
}

/// Generate visitor code using our generator
fn generate_visitor_code() -> Result<(), Box<dyn std::error::Error>> {
    // Path to the Substrait proto files
    let proto_dir = PathBuf::from("third_party/substrait/proto");
    
    // Path to the output file
    let output_dir = PathBuf::from("src/textplan/converter/generated");
    let output_path = output_dir.join("plan_visitor.rs");
    
    // Create the output directory if it doesn't exist
    fs::create_dir_all(&output_dir)?;
    
    println!("cargo:warning=Generating visitor code from Substrait protobuf schema");
    println!("cargo:warning=Proto directory: {}", proto_dir.display());
    println!("cargo:warning=Output path: {}", output_path.display());
    
    // Run the visitor generator directly
    visitor_generator::generate(&proto_dir, &output_path)?;
    
    println!("cargo:warning=Visitor code generation completed successfully");
    Ok(())
}