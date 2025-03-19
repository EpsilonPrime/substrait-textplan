// SPDX-License-Identifier: Apache-2.0

//! Tool to generate visitor code for Substrait protocol buffers.
//! 
//! This tool parses Substrait protobuf definitions and generates Rust code
//! for a complete visitor implementation.

use std::collections::{HashMap, BTreeMap};
use std::fs::File;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::error::Error;
use std::fmt;

/// Error type for visitor generator operations
#[derive(Debug)]
pub enum GeneratorError {
    Io(io::Error),
    ProtoError(String),
    ParseError(String),
}

impl fmt::Display for GeneratorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GeneratorError::Io(err) => write!(f, "I/O error: {}", err),
            GeneratorError::ProtoError(msg) => write!(f, "Protobuf error: {}", msg),
            GeneratorError::ParseError(msg) => write!(f, "Parse error: {}", msg),
        }
    }
}

impl Error for GeneratorError {}

impl From<io::Error> for GeneratorError {
    fn from(err: io::Error) -> Self {
        GeneratorError::Io(err)
    }
}

/// Represents a protobuf message field
#[derive(Debug, Clone)]
struct ProtoField {
    name: String,
    field_type: String,
    is_message: bool,
    is_repeated: bool,
    is_optional: bool,
    is_enum: bool,
    namespace: Option<String>,
}

/// Represents a protobuf message
#[derive(Debug, Clone)]
struct ProtoMessage {
    name: String,
    fields: Vec<ProtoField>,
    nested_messages: Vec<ProtoMessage>,
    nested_enums: Vec<ProtoEnum>,
    package: String,

    // Classification flags for special message types
    is_rel_variant: bool, // Is this a specific relation variant (ReadRel, FilterRel, etc.)?
}

/// Represents a protobuf enum
#[derive(Debug, Clone)]
struct ProtoEnum {
    name: String,
    values: Vec<String>,
    package: String,
}

/// Represents a protobuf file
#[derive(Debug, Clone)]
struct ProtoFile {
    path: PathBuf,
    package: String,
    messages: Vec<ProtoMessage>,
    enums: Vec<ProtoEnum>,
    imports: Vec<String>,
}

/// The main visitor generator
pub struct VisitorGenerator {
    proto_dir: PathBuf,
    output_path: PathBuf,
    
    // Phase 1: Discovery state
    proto_files: Vec<ProtoFile>,
    messages_map: HashMap<String, ProtoMessage>,
    enums_map: HashMap<String, ProtoEnum>,
    rel_variants: Vec<String>,
    
    // Phase 2: Generation state
    method_dependencies: HashMap<String, Vec<String>>,
    visit_methods: BTreeMap<String, String>,  // Method name -> Method signature
    visit_implementations: BTreeMap<String, String>, // Method name -> Method implementation
}

impl VisitorGenerator {
    /// Create a new generator
    pub fn new(proto_dir: impl AsRef<Path>, output_path: impl AsRef<Path>) -> Self {
        Self {
            proto_dir: proto_dir.as_ref().to_path_buf(),
            output_path: output_path.as_ref().to_path_buf(),
            proto_files: Vec::new(),
            messages_map: HashMap::new(),
            enums_map: HashMap::new(),
            rel_variants: Vec::new(),
            method_dependencies: HashMap::new(),
            visit_methods: BTreeMap::new(),
            visit_implementations: BTreeMap::new(),
        }
    }

    /// Run the generator - main entry point
    pub fn run(&mut self) -> Result<(), GeneratorError> {
        // Phase 1: Discover all protobuf types and their relationships
        self.discover_proto_types()?;
        
        // Phase 2: Generate visitor code based on discovered types
        let visitor_code = self.generate_visitor_code()?;
        
        // Write the generated code to a file
        self.write_visitor_code(&visitor_code)?;
        
        Ok(())
    }

    //
    // PHASE 1: DISCOVERY
    //

    /// Discover all protobuf types and their relationships
    fn discover_proto_types(&mut self) -> Result<(), GeneratorError> {
        // 1. Parse all proto files in the directory
        self.parse_proto_files()?;
        
        // 2. Build maps of all message and enum types for quick lookup
        self.build_type_maps();
        
        // 3. Identify all relation variants
        self.identify_relation_variants();
        
        // 4. Build dependency graph for visitor methods
        self.build_method_dependencies();
        
        Ok(())
    }

    /// Parse all protobuf files in the directory
    fn parse_proto_files(&mut self) -> Result<(), GeneratorError> {
        // In a real implementation, this would recursively search the proto_dir
        // and parse all .proto files using a protobuf parser
        
        // For the sake of this example, we'll create a simplified version of
        // the Substrait schema with just a few key messages
        
        // Example implementation for a few key messages
        let plan_message = ProtoMessage {
            name: "Plan".to_string(),
            fields: vec![
                ProtoField {
                    name: "relations".to_string(),
                    field_type: "PlanRel".to_string(),
                    is_message: true,
                    is_repeated: true,
                    is_optional: false,
                    is_enum: false,
                    namespace: None,
                },
                ProtoField {
                    name: "extension_uris".to_string(),
                    field_type: "SimpleExtensionUri".to_string(),
                    is_message: true,
                    is_repeated: true,
                    is_optional: false,
                    is_enum: false,
                    namespace: Some("extensions".to_string()),
                },
                ProtoField {
                    name: "extensions".to_string(),
                    field_type: "SimpleExtensionDeclaration".to_string(),
                    is_message: true,
                    is_repeated: true,
                    is_optional: false,
                    is_enum: false,
                    namespace: Some("extensions".to_string()),
                },
                ProtoField {
                    name: "version".to_string(),
                    field_type: "Version".to_string(),
                    is_message: true,
                    is_repeated: false,
                    is_optional: true,
                    is_enum: false,
                    namespace: None,
                },
                ProtoField {
                    name: "advanced_extensions".to_string(),
                    field_type: "AdvancedExtension".to_string(),
                    is_message: true,
                    is_repeated: true,
                    is_optional: false,
                    is_enum: false,
                    namespace: Some("extensions".to_string()),
                },
            ],
            nested_messages: Vec::new(),
            nested_enums: Vec::new(),
            package: "substrait".to_string(),
            is_rel_variant: false,
        };
        
        let rel_message = ProtoMessage {
            name: "Rel".to_string(),
            fields: vec![
                ProtoField {
                    name: "rel_type".to_string(),
                    field_type: "RelType".to_string(),
                    is_message: true,
                    is_repeated: false,
                    is_optional: true,
                    is_enum: false,
                    namespace: None,
                },
            ],
            nested_messages: Vec::new(),
            nested_enums: Vec::new(),
            package: "substrait".to_string(),
            is_rel_variant: false,
        };

        // Define various relation variants
        let read_rel_message = ProtoMessage {
            name: "ReadRel".to_string(),
            fields: vec![
                ProtoField {
                    name: "common".to_string(),
                    field_type: "RelCommon".to_string(),
                    is_message: true,
                    is_repeated: false,
                    is_optional: true,
                    is_enum: false,
                    namespace: None,
                },
                ProtoField {
                    name: "base_schema".to_string(),
                    field_type: "NamedStruct".to_string(),
                    is_message: true,
                    is_repeated: false,
                    is_optional: true,
                    is_enum: false,
                    namespace: None,
                },
            ],
            nested_messages: Vec::new(),
            nested_enums: Vec::new(),
            package: "substrait".to_string(),
            is_rel_variant: true,  // Mark this as a relation variant
        };
        
        let filter_rel_message = ProtoMessage {
            name: "FilterRel".to_string(),
            fields: vec![
                ProtoField {
                    name: "common".to_string(),
                    field_type: "RelCommon".to_string(),
                    is_message: true,
                    is_repeated: false,
                    is_optional: true,
                    is_enum: false,
                    namespace: None,
                },
                ProtoField {
                    name: "input".to_string(),
                    field_type: "Rel".to_string(),
                    is_message: true,
                    is_repeated: false,
                    is_optional: true,
                    is_enum: false,
                    namespace: None,
                },
            ],
            nested_messages: Vec::new(),
            nested_enums: Vec::new(),
            package: "substrait".to_string(),
            is_rel_variant: true,  // Mark this as a relation variant
        };
        
        // Add these messages to our proto_files
        let proto_file = ProtoFile {
            path: PathBuf::from("substrait/plan.proto"),
            package: "substrait".to_string(),
            messages: vec![plan_message, rel_message, read_rel_message, filter_rel_message],
            enums: Vec::new(),
            imports: Vec::new(),
        };
        
        self.proto_files.push(proto_file);
        
        Ok(())
    }
    
    /// Build maps of all message and enum types for quick lookup
    fn build_type_maps(&mut self) {
        // First collect all top-level messages and enums
        for file in &self.proto_files {
            for message in &file.messages {
                let full_name = format!("{}.{}", file.package, message.name);
                self.messages_map.insert(full_name, message.clone());
            }
            
            for enum_type in &file.enums {
                let full_name = format!("{}.{}", file.package, enum_type.name);
                self.enums_map.insert(full_name, enum_type.clone());
            }
        }
        
        // Now add nested messages and enums to the maps
        let mut messages_to_process = Vec::new();
        for (full_name, message) in self.messages_map.clone() {
            messages_to_process.push((message, full_name));
        }
        
        while let Some((message, parent_full_name)) = messages_to_process.pop() {
            // Process nested messages
            for nested in &message.nested_messages {
                let nested_full_name = format!("{}.{}", parent_full_name, nested.name);
                self.messages_map.insert(nested_full_name.clone(), nested.clone());
                messages_to_process.push((nested.clone(), nested_full_name));
            }
            
            // Process nested enums
            for enum_type in &message.nested_enums {
                let enum_full_name = format!("{}.{}", parent_full_name, enum_type.name);
                self.enums_map.insert(enum_full_name, enum_type.clone());
            }
        }
    }
    
    /// Identify all relation variants
    fn identify_relation_variants(&mut self) {
        for (full_name, message) in &self.messages_map {
            if message.is_rel_variant {
                self.rel_variants.push(full_name.clone());
            }
        }
    }
    
    /// Build dependency graph for visitor methods
    fn build_method_dependencies(&mut self) {
        // For each message, determine which other messages it depends on
        for (_full_name, message) in &self.messages_map {
            let method_name = format!("visit_{}", to_snake_case(&message.name));
            let mut depends_on = Vec::new();
            
            // Add dependencies from fields
            for field in &message.fields {
                if field.is_message && !field.is_enum {
                    let field_full_name = if let Some(namespace) = &field.namespace {
                        format!("{}.{}.{}", message.package, namespace, field.field_type)
                    } else {
                        format!("{}.{}", message.package, field.field_type)
                    };
                    
                    if self.messages_map.contains_key(&field_full_name) {
                        let field_method_name = format!("visit_{}", to_snake_case(&field.field_type));
                        depends_on.push(field_method_name);
                    }
                }
            }
            
            self.method_dependencies.insert(method_name, depends_on);
        }
        
        // Add special dependency for visit_plan on visit_rel
        self.method_dependencies.get_mut("visit_plan")
            .map(|deps| deps.push("visit_rel".to_string()));
        
        // Add dependencies for visit_rel on all rel variants
        let mut rel_variant_methods = Vec::new();
        for variant in &self.rel_variants {
            let variant_name = variant.split('.').last().unwrap_or("");
            let method_name = format!("visit_{}", to_snake_case(variant_name));
            rel_variant_methods.push(method_name);
        }
        
        self.method_dependencies.insert("visit_rel".to_string(), rel_variant_methods);
    }
    
    //
    // PHASE 2: GENERATION
    //
    
    /// Generate visitor code based on discovered types
    fn generate_visitor_code(&mut self) -> Result<String, GeneratorError> {
        let mut output = String::new();
        
        // Generate file header with imports
        self.generate_header(&mut output);
        
        // Generate trait definition with all visit methods
        self.generate_trait_definition(&mut output);
        
        // Generate default implementation
        self.generate_default_implementation(&mut output);
        
        Ok(output)
    }
    
    /// Generate file header with imports
    fn generate_header(&self, output: &mut String) {
        output.push_str("// SPDX-License-Identifier: Apache-2.0\n\n");
        output.push_str("//! GENERATED CODE - DO NOT MODIFY\n");
        output.push_str("//! Generated visitor for Substrait protocol buffers.\n\n");
        
        // Import declarations
        output.push_str("use crate::proto::substrait;\n");
        output.push_str("use crate::proto::substrait::extensions;\n");
        output.push_str("use crate::textplan::common::error::TextPlanError;\n\n");
    }
    
    /// Generate trait definition with all visit methods
    fn generate_trait_definition(&mut self, output: &mut String) {
        output.push_str("/// Base visitor trait for Substrait plans.\n");
        output.push_str("/// \n");
        output.push_str("/// This trait defines the visit methods for all protobuf message types in the Substrait schema.\n");
        output.push_str("/// It's intended to be implemented by concrete visitors that need to traverse and process\n");
        output.push_str("/// Substrait plans.\n");
        output.push_str("pub trait BasePlanProtoVisitor {\n");
        output.push_str("    /// The result type of the visitor\n");
        output.push_str("    type Result;\n\n");
        
        // First, add the main entry points in a specific order
        
        // 1. Plan (the top-level entry point)
        output.push_str("    /// Visit a plan (entry point)\n");
        output.push_str("    fn visit_plan(&mut self, plan: &substrait::Plan) -> Self::Result;\n\n");
        
        // 2. Rel (the base relation type)
        output.push_str("    /// Visit a relation directly\n");
        output.push_str("    fn visit_rel(&mut self, rel: &substrait::Rel) -> Self::Result;\n\n");
        
        // Then add all relation variants
        for variant_full_name in &self.rel_variants {
            let variant_name = variant_full_name.split('.').last().unwrap_or("");
            let snake_name = to_snake_case(variant_name);
            
            output.push_str(&format!("    /// Visit a {} message\n", variant_name));
            output.push_str(&format!("    fn visit_{}(&mut self, value: &substrait::{}) -> Self::Result;\n\n", 
                                   snake_name, variant_name));
        }
        
        // Now add all other message types
        for (full_name, message) in &self.messages_map {
            // Skip if already handled (Plan and Rel)
            if message.name == "Plan" || message.name == "Rel" || message.is_rel_variant {
                continue;
            }
            
            let snake_name = to_snake_case(&message.name);
            
            output.push_str(&format!("    /// Visit a {} message\n", message.name));
            
            // Construct the import path based on namespace
            let import_path = if full_name.contains("extensions.") {
                format!("extensions::{}", message.name)
            } else {
                format!("substrait::{}", message.name)
            };
            
            output.push_str(&format!("    fn visit_{}(&mut self, value: &{}) -> Self::Result;\n\n", 
                                   snake_name, import_path));
        }
        
        // Close the trait
        output.push_str("}\n\n");
    }
    
    /// Generate default implementation
    fn generate_default_implementation(&mut self, output: &mut String) {
        output.push_str("/// Default implementation that can be used as a starting point for concrete visitors.\n");
        output.push_str("/// \n");
        output.push_str("/// This implementation traverses the entire plan structure but does nothing with the values.\n");
        output.push_str("/// Implementations can override specific methods to process only the parts they care about.\n");
        output.push_str("pub struct DefaultPlanVisitor;\n\n");
        
        output.push_str("impl DefaultPlanVisitor {\n");
        output.push_str("    /// Create a new default visitor\n");
        output.push_str("    pub fn new() -> Self {\n");
        output.push_str("        Self\n");
        output.push_str("    }\n");
        output.push_str("}\n\n");
        
        output.push_str("impl BasePlanProtoVisitor for DefaultPlanVisitor {\n");
        output.push_str("    type Result = Result<(), TextPlanError>;\n\n");
        
        // Generate implementations, starting with the primary entry point (visit_plan)
        self.generate_visit_plan_implementation(output);
        
        // Generate visit_rel implementation
        self.generate_visit_rel_implementation(output);
        
        // Generate implementations for relation variants
        for variant_full_name in &self.rel_variants {
            let variant_name = variant_full_name.split('.').last().unwrap_or("");
            let snake_name = to_snake_case(variant_name);
            
            output.push_str(&format!("    fn visit_{}(&mut self, _value: &substrait::{}) -> Self::Result {{\n", 
                                   snake_name, variant_name));
            output.push_str(&format!("        // Default implementation for {}\n", variant_name));
            output.push_str("        Ok(())\n");
            output.push_str("    }\n\n");
        }
        
        // Generate implementations for all other message types
        for (full_name, message) in &self.messages_map {
            // Skip if already handled
            if message.name == "Plan" || message.name == "Rel" || message.is_rel_variant {
                continue;
            }
            
            let snake_name = to_snake_case(&message.name);
            
            // Construct the import path based on namespace
            let import_path = if full_name.contains("extensions.") {
                format!("extensions::{}", message.name)
            } else {
                format!("substrait::{}", message.name)
            };
            
            output.push_str(&format!("    fn visit_{}(&mut self, _value: &{}) -> Self::Result {{\n", 
                                   snake_name, import_path));
            output.push_str(&format!("        // Default implementation for {}\n", message.name));
            output.push_str("        Ok(())\n");
            output.push_str("    }\n\n");
        }
        
        // Close the implementation
        output.push_str("}\n");
    }
    
    /// Generate implementation for visit_plan
    fn generate_visit_plan_implementation(&self, output: &mut String) {
        output.push_str("    fn visit_plan(&mut self, plan: &substrait::Plan) -> Self::Result {\n");
        output.push_str("        // Visit all relations in the plan\n");
        output.push_str("        for relation in &plan.relations {\n");
        output.push_str("            if let Some(rel_type) = &relation.rel_type {\n");
        output.push_str("                match rel_type {\n");
        output.push_str("                    substrait::plan_rel::RelType::Rel(rel) => {\n");
        output.push_str("                        self.visit_rel(rel)?;\n");
        output.push_str("                    }\n");
        output.push_str("                    substrait::plan_rel::RelType::Root(_) => {\n");
        output.push_str("                        // Handle root relation\n");
        output.push_str("                    }\n");
        output.push_str("                }\n");
        output.push_str("            }\n");
        output.push_str("        }\n");
        output.push_str("        Ok(())\n");
        output.push_str("    }\n\n");
    }
    
    /// Generate implementation for visit_rel
    fn generate_visit_rel_implementation(&self, output: &mut String) {
        output.push_str("    fn visit_rel(&mut self, rel: &substrait::Rel) -> Self::Result {\n");
        output.push_str("        // Visit based on relation type\n");
        output.push_str("        if let Some(rel_type) = &rel.rel_type {\n");
        output.push_str("            match rel_type {\n");
        
        // Generate cases for all relation variants we know about
        // The real implementation would include all rel variant types from protobuf
        output.push_str("                substrait::rel::RelType::Read(read_rel) => {\n");
        output.push_str("                    self.visit_read_rel(read_rel)?;\n");
        output.push_str("                }\n");
        
        output.push_str("                substrait::rel::RelType::Filter(filter_rel) => {\n");
        output.push_str("                    self.visit_filter_rel(filter_rel)?;\n");
        output.push_str("                }\n");
        
        // Default case for other relation types
        output.push_str("                _ => {}\n");
        
        output.push_str("            }\n");
        output.push_str("        }\n");
        output.push_str("        Ok(())\n");
        output.push_str("    }\n\n");
    }
    
    /// Write the generated code to the output file
    fn write_visitor_code(&self, code: &str) -> Result<(), GeneratorError> {
        let mut file = File::create(&self.output_path)?;
        file.write_all(code.as_bytes())?;
        Ok(())
    }
}

/// Convert CamelCase to snake_case
fn to_snake_case(s: &str) -> String {
    let mut result = String::new();
    let mut prev_is_lowercase = false;
    
    for (i, c) in s.char_indices() {
        if c.is_uppercase() {
            if i > 0 && prev_is_lowercase {
                result.push('_');
            }
            result.push(c.to_lowercase().next().unwrap());
            prev_is_lowercase = false;
        } else {
            result.push(c);
            prev_is_lowercase = true;
        }
    }
    
    result
}

/// Main function to run the visitor generator
pub fn main() -> Result<(), Box<dyn Error>> {
    // Parse command line arguments
    let args: Vec<String> = std::env::args().collect();
    
    if args.len() < 3 {
        eprintln!("Usage: {} <proto_dir> <output_path>", args[0]);
        return Ok(());
    }
    
    let proto_dir = &args[1];
    let output_path = &args[2];
    
    // Create and run the generator
    let mut generator = VisitorGenerator::new(proto_dir, output_path);
    generator.run()?;
    
    println!("Generated visitor code successfully at {}", output_path);
    
    Ok(())
}

/// Run the generator as part of the build.rs process
pub fn generate_visitor(proto_dir: &Path, output_path: &Path) -> Result<(), Box<dyn Error>> {
    println!("Generating visitor code for Substrait protobuf schema...");
    
    let mut generator = VisitorGenerator::new(proto_dir, output_path);
    generator.run()?;
    
    println!("Generated visitor code successfully at {}", output_path.display());
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_to_snake_case() {
        assert_eq!(to_snake_case("CamelCase"), "camel_case");
        assert_eq!(to_snake_case("snake_case"), "snake_case");
        assert_eq!(to_snake_case("ReadRel"), "read_rel");
        assert_eq!(to_snake_case("ReadRelation"), "read_relation");
    }
}