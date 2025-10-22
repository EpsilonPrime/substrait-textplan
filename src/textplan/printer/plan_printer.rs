// SPDX-License-Identifier: Apache-2.0

//! PlanPrinter for converting a Substrait Symbol Table to a textplan string.

use std::collections::{HashMap, HashSet};
use std::sync::Arc;

use crate::textplan::common::error::TextPlanError;
use crate::textplan::common::structured_symbol_data::RelationData;
use crate::textplan::printer::expression_printer::ExpressionPrinter;
use crate::textplan::symbol_table::{RelationType, SourceType, SymbolInfo, SymbolTable, SymbolType};

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

        // Output pipelines section first
        let pipelines_text = self.output_pipelines_section(symbol_table)?;
        if !pipelines_text.is_empty() {
            result.push_str(&pipelines_text);
            result.push('\n');
        }

        // Don't print ROOT relations - they're internal to the plan structure
        // self.process_root_relations(symbol_table, &mut result)?;

        // Process all other relations
        self.process_relations(symbol_table, &mut result)?;

        // Process schemas
        self.process_schemas(symbol_table, &mut result)?;

        // Process sources
        self.process_sources(symbol_table, &mut result)?;

        // Process extension space
        self.process_extension_space(symbol_table, &mut result)?;

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
        use crate::textplan::common::structured_symbol_data::RelationData;

        let mut result = String::new();

        // Get the relation type from the RelationData blob (which contains the protobuf)
        let rel_type = if let Some(blob_lock) = &relation.blob {
            if let Ok(blob_data) = blob_lock.lock() {
                if let Some(relation_data) = blob_data.downcast_ref::<RelationData>() {
                    Self::rel_type_from_proto(&relation_data.relation)
                } else {
                    RelationType::Unknown
                }
            } else {
                RelationType::Unknown
            }
        } else {
            RelationType::Unknown
        };

        // Convert the relation type to a string
        let rel_type_str = Self::rel_type_to_string(rel_type);

        // Start the relation definition
        result.push_str(&format!(
            "{} relation {} {{\n",
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
            RelationType::Project => {
                self.add_project_relation_properties(relation, symbol_table, &indent, &mut result)?;
            }
            RelationType::Aggregate => {
                self.add_aggregate_relation_properties(relation, symbol_table, &indent, &mut result)?;
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
        use ::substrait::proto::rel::RelType;

        // Extract the data we need (clone to avoid holding the lock)
        let (source_name, schema_name, filter_expr) = if let Some(blob_lock) = &relation.blob {
            if let Ok(blob_data) = blob_lock.lock() {
                if let Some(relation_data) = blob_data.downcast_ref::<RelationData>() {
                    // Extract the ReadRel from the protobuf
                    if let Some(RelType::Read(read_rel)) = &relation_data.relation.rel_type {
                        let source_name = relation_data.source.as_ref().map(|s| s.name().to_string());
                        let schema_name = relation_data.schema.as_ref().map(|s| s.name().to_string());
                        let filter_expr = read_rel.filter.clone();
                        (source_name, schema_name, filter_expr)
                    } else {
                        (None, None, None)
                    }
                } else {
                    (None, None, None)
                }
            } else {
                (None, None, None)
            }
        } else {
            (None, None, None)
        };

        // Now print the properties (lock is released)
        if let Some(source) = source_name {
            result.push_str(&format!("{}source {};\n", indent, source));
        }

        if let Some(schema) = schema_name {
            result.push_str(&format!("{}base_schema {};\n", indent, schema));
        }

        if let Some(filter) = filter_expr {
            let mut expr_printer = ExpressionPrinter::new(symbol_table, Some(relation));
            let filter_text = expr_printer.print_expression(&filter)?;
            result.push_str(&format!("{}filter {};\n", indent, filter_text));
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
        relation: &Arc<SymbolInfo>,
        symbol_table: &SymbolTable,
        indent: &str,
        result: &mut String,
    ) -> Result<(), TextPlanError> {
        use ::substrait::proto::rel::RelType;

        // Extract the condition expression (clone it to avoid holding the lock)
        let condition_expr = if let Some(blob_lock) = &relation.blob {
            if let Ok(blob_data) = blob_lock.lock() {
                if let Some(relation_data) = blob_data.downcast_ref::<RelationData>() {
                    // Extract the FilterRel from the protobuf
                    if let Some(RelType::Filter(filter_rel)) = &relation_data.relation.rel_type {
                        filter_rel.condition.clone()
                    } else {
                        None
                    }
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
        };

        // Now print the condition (lock is released)
        if let Some(condition) = condition_expr {
            let mut expr_printer = ExpressionPrinter::new(symbol_table, Some(relation));
            let condition_text = expr_printer.print_expression(&condition)?;
            result.push_str(&format!("{}filter {};\n", indent, condition_text));
        }

        Ok(())
    }

    /// Adds properties for a project relation.
    fn add_project_relation_properties(
        &self,
        relation: &Arc<SymbolInfo>,
        symbol_table: &SymbolTable,
        indent: &str,
        result: &mut String,
    ) -> Result<(), TextPlanError> {
        use ::substrait::proto::rel::RelType;

        // Extract the project expressions and common (clone to avoid holding the lock)
        let (expressions, common) = if let Some(blob_lock) = &relation.blob {
            if let Ok(blob_data) = blob_lock.lock() {
                if let Some(relation_data) = blob_data.downcast_ref::<RelationData>() {
                    if let Some(RelType::Project(project_rel)) = &relation_data.relation.rel_type {
                        (project_rel.expressions.clone(), project_rel.common.clone())
                    } else {
                        (Vec::new(), None)
                    }
                } else {
                    (Vec::new(), None)
                }
            } else {
                (Vec::new(), None)
            }
        } else {
            (Vec::new(), None)
        };

        // Print expressions (lock is released)
        let mut expr_printer = ExpressionPrinter::new(symbol_table, Some(relation));
        for expr in &expressions {
            let expr_text = expr_printer.print_expression(expr)?;
            result.push_str(&format!("{}expression {};\n", indent, expr_text));
        }

        // Print emit from common
        if let Some(common_val) = common {
            if let Some(::substrait::proto::rel_common::EmitKind::Emit(emit)) = &common_val.emit_kind {
                if !expressions.is_empty() && !emit.output_mapping.is_empty() {
                    result.push('\n');
                }
                for &field_idx in &emit.output_mapping {
                    // TODO: Look up field name from relation data
                    result.push_str(&format!("{}emit field#{};\n", indent, field_idx));
                }
            }
        }

        Ok(())
    }

    /// Adds properties for an aggregate relation.
    fn add_aggregate_relation_properties(
        &self,
        relation: &Arc<SymbolInfo>,
        symbol_table: &SymbolTable,
        indent: &str,
        result: &mut String,
    ) -> Result<(), TextPlanError> {
        use ::substrait::proto::rel::RelType;

        // Extract the measures (clone to avoid holding the lock)
        let measures = if let Some(blob_lock) = &relation.blob {
            if let Ok(blob_data) = blob_lock.lock() {
                if let Some(relation_data) = blob_data.downcast_ref::<RelationData>() {
                    if let Some(RelType::Aggregate(agg_rel)) = &relation_data.relation.rel_type {
                        agg_rel.measures.clone()
                    } else {
                        Vec::new()
                    }
                } else {
                    Vec::new()
                }
            } else {
                Vec::new()
            }
        } else {
            Vec::new()
        };

        // Print measures (lock is released)
        for measure in &measures {
            result.push_str(&format!("{}measure {{\n", indent));
            let measure_indent = format!("{}  ", indent);

            if let Some(agg_func) = &measure.measure {
                let mut expr_printer = ExpressionPrinter::new(symbol_table, Some(relation));
                // TODO: Print aggregate function properly - for now just placeholder
                result.push_str(&format!("{}measure AGGREGATE_FUNCTION_NOT_YET_IMPLEMENTED;\n", measure_indent));
            }

            result.push_str(&format!("{}}}\n", indent));
        }

        Ok(())
    }

    /// Builds a pipeline path by following the continuing_pipeline chain.
    ///
    /// # Arguments
    ///
    /// * `symbol_table` - The symbol table
    /// * `info` - Starting symbol for the pipeline
    ///
    /// # Returns
    ///
    /// Vector of relation names in the pipeline
    fn pipeline_to_path(
        &self,
        symbol_table: &SymbolTable,
        info: &Arc<SymbolInfo>,
    ) -> Vec<String> {
        use crate::textplan::common::structured_symbol_data::RelationData;

        let mut pipeline = Vec::new();

        // Get the relation data
        if let Some(relation_data_lock) = &info.blob {
            if let Ok(relation_data) = relation_data_lock.lock() {
                if let Some(relation_data) = relation_data.downcast_ref::<RelationData>() {
                    pipeline.push(info.name().to_string());

                    // Follow the continuing pipeline
                    if let Some(continuing) = &relation_data.continuing_pipeline {
                        let tail_pipe = self.pipeline_to_path(symbol_table, continuing);
                        pipeline.extend(tail_pipe);
                    }
                }
            }
        }

        pipeline
    }

    /// Converts a protobuf Rel to a RelationType enum.
    ///
    /// # Arguments
    ///
    /// * `rel` - The protobuf relation
    ///
    /// # Returns
    ///
    /// The RelationType enum value
    fn rel_type_from_proto(rel: &::substrait::proto::Rel) -> RelationType {
        use ::substrait::proto::rel::RelType;

        match &rel.rel_type {
            Some(RelType::Read(_)) => RelationType::Read,
            Some(RelType::Filter(_)) => RelationType::Filter,
            Some(RelType::Fetch(_)) => RelationType::Fetch,
            Some(RelType::Aggregate(_)) => RelationType::Aggregate,
            Some(RelType::Sort(_)) => RelationType::Sort,
            Some(RelType::Join(_)) => RelationType::Join,
            Some(RelType::Project(_)) => RelationType::Project,
            Some(RelType::Set(_)) => RelationType::Set,
            Some(RelType::ExtensionSingle(_)) => RelationType::ExtensionSingle,
            Some(RelType::ExtensionMulti(_)) => RelationType::ExtensionMulti,
            Some(RelType::ExtensionLeaf(_)) => RelationType::ExtensionLeaf,
            Some(RelType::Cross(_)) => RelationType::Cross,
            Some(RelType::Reference(_)) => RelationType::Unknown, // No specific type for Reference
            Some(RelType::Write(_)) => RelationType::Write,
            Some(RelType::Ddl(_)) => RelationType::Ddl,
            Some(RelType::HashJoin(_)) => RelationType::HashJoin,
            Some(RelType::MergeJoin(_)) => RelationType::MergeJoin,
            Some(RelType::NestedLoopJoin(_)) => RelationType::Join, // Map to generic Join
            Some(RelType::Window(_)) => RelationType::Unknown, // No specific Window type
            Some(RelType::Exchange(_)) => RelationType::Exchange,
            Some(RelType::Expand(_)) => RelationType::Unknown, // No specific Expand type
            Some(RelType::Update(_)) => RelationType::Unknown, // No specific Update type
            None => RelationType::Unknown,
        }
    }

    /// Converts a RelationType enum to a lowercase string.
    ///
    /// # Arguments
    ///
    /// * `rel_type` - The relation type
    ///
    /// # Returns
    ///
    /// The lowercase string representation
    fn rel_type_to_string(rel_type: RelationType) -> &'static str {
        match rel_type {
            RelationType::Unknown => "unknown",
            RelationType::Read => "read",
            RelationType::Project => "project",
            RelationType::Join => "join",
            RelationType::Cross => "cross",
            RelationType::Fetch => "fetch",
            RelationType::Aggregate => "aggregate",
            RelationType::Sort => "sort",
            RelationType::Filter => "filter",
            RelationType::Set => "set",
            RelationType::HashJoin => "hash_join",
            RelationType::MergeJoin => "merge_join",
            RelationType::Exchange => "exchange",
            RelationType::Ddl => "ddl",
            RelationType::Write => "write",
            RelationType::ExtensionLeaf => "extension_leaf",
            RelationType::ExtensionSingle => "extension_single",
            RelationType::ExtensionMulti => "extension_multi",
        }
    }

    /// Outputs the pipelines section of the textplan.
    ///
    /// # Arguments
    ///
    /// * `symbol_table` - The symbol table
    ///
    /// # Returns
    ///
    /// The pipelines section as a string
    fn output_pipelines_section(
        &self,
        symbol_table: &SymbolTable,
    ) -> Result<String, TextPlanError> {
        use crate::textplan::common::structured_symbol_data::RelationData;

        let mut text = String::new();
        let mut has_previous_text = false;

        for info in symbol_table.symbols() {
            // Only process PlanRelation and Relation symbols
            if info.symbol_type() != SymbolType::PlanRelation
                && info.symbol_type() != SymbolType::Relation
            {
                continue;
            }

            // Get the relation data
            if let Some(relation_data_lock) = &info.blob {
                if let Ok(relation_data) = relation_data_lock.lock() {
                    if let Some(relation_data) = relation_data.downcast_ref::<RelationData>() {
                        // Process new pipelines
                        for pipeline_start in &relation_data.new_pipelines {
                            let mut pipeline = self.pipeline_to_path(symbol_table, pipeline_start);
                            pipeline.insert(0, info.name().to_string());

                            // Output in reverse order (from leaf to root)
                            text.push_str("  ");
                            for (i, pipe_name) in pipeline.iter().rev().enumerate() {
                                if i > 0 {
                                    text.push_str(" -> ");
                                }
                                text.push_str(pipe_name);
                            }
                            text.push_str(";\n");
                            has_previous_text = true;
                        }

                        // Process subquery pipelines
                        for pipeline_start in &relation_data.sub_query_pipelines {
                            let pipeline = self.pipeline_to_path(symbol_table, pipeline_start);

                            // Output in reverse order
                            text.push_str("  ");
                            for (i, pipe_name) in pipeline.iter().rev().enumerate() {
                                if i > 0 {
                                    text.push_str(" -> ");
                                }
                                text.push_str(pipe_name);
                            }
                            text.push_str(";\n");
                            has_previous_text = true;
                        }
                    }
                }
            }
        }

        if has_previous_text {
            Ok(format!("pipelines {{\n{}}}", text))
        } else {
            Ok(String::new())
        }
    }

    /// Processes schema symbols in the symbol table.
    fn process_schemas(
        &self,
        symbol_table: &SymbolTable,
        result: &mut String,
    ) -> Result<(), TextPlanError> {
        let schemas: Vec<_> = symbol_table
            .symbols()
            .iter()
            .filter(|s| s.symbol_type() == SymbolType::Schema)
            .cloned()
            .collect();

        for schema in schemas {
            result.push_str(&format!("schema {} {{\n", schema.name()));

            // Get the schema fields from the blob (which contains a NamedStruct)
            if let Some(blob_lock) = &schema.blob {
                if let Ok(blob_data) = blob_lock.lock() {
                    if let Some(named_struct) = blob_data.downcast_ref::<::substrait::proto::expression::field_reference::RootType>() {
                        // TODO: Extract fields from the NamedStruct
                        // For now, we'll look for field symbols associated with this schema
                    }
                }
            }

            // Find field symbols that belong to this schema
            for field in symbol_table.symbols() {
                if field.symbol_type() == SymbolType::Field {
                    if let Some(field_schema) = field.schema() {
                        if field_schema.name() == schema.name() {
                            // Print field with its type
                            result.push_str(&format!("  {} ", field.name()));
                            // TODO: Get the actual type from the field's blob
                            result.push_str("UNKNOWN_TYPE;\n");
                        }
                    }
                }
            }

            result.push_str("}\n\n");
        }

        Ok(())
    }

    /// Processes source symbols in the symbol table.
    fn process_sources(
        &self,
        symbol_table: &SymbolTable,
        result: &mut String,
    ) -> Result<(), TextPlanError> {
        use ::substrait::proto::read_rel::{LocalFiles, NamedTable, VirtualTable, ExtensionTable};

        let sources: Vec<_> = symbol_table
            .symbols()
            .iter()
            .filter(|s| s.symbol_type() == SymbolType::Source)
            .cloned()
            .collect();

        for source in sources {
            // Determine source type from subtype field
            let source_type = match source.subtype::<SourceType>() {
                Some(st) => st,
                None => continue,
            };

            match source_type {
                SourceType::LocalFiles => {
                    // Extract LocalFiles from blob
                    if let Some(blob_lock) = &source.blob {
                        if let Ok(blob_data) = blob_lock.lock() {
                            if let Some(local_files) = blob_data.downcast_ref::<LocalFiles>() {
                                result.push_str(&format!("source local_files {} {{\n", source.name()));
                                result.push_str("  items = [\n");

                                for item in &local_files.items {
                                    result.push_str("    {");

                                    // Print the path type
                                    if let Some(path_type) = &item.path_type {
                                        use ::substrait::proto::read_rel::local_files::file_or_files::PathType;
                                        match path_type {
                                            PathType::UriFile(uri) => {
                                                result.push_str(&format!("uri_file: \"{}\"", uri));
                                            }
                                            PathType::UriPath(uri) => {
                                                result.push_str(&format!("uri_path: \"{}\"", uri));
                                            }
                                            PathType::UriPathGlob(uri) => {
                                                result.push_str(&format!("uri_path_glob: \"{}\"", uri));
                                            }
                                            PathType::UriFolder(uri) => {
                                                result.push_str(&format!("uri_folder: \"{}\"", uri));
                                            }
                                        }
                                    }

                                    // Print start if non-zero
                                    if item.start != 0 {
                                        result.push_str(&format!(" start: {}", item.start));
                                    }

                                    // Print length if non-zero
                                    if item.length != 0 {
                                        result.push_str(&format!(" length: {}", item.length));
                                    }

                                    // Print file format
                                    if let Some(file_format) = &item.file_format {
                                        use ::substrait::proto::read_rel::local_files::file_or_files::FileFormat;
                                        match file_format {
                                            FileFormat::Parquet(_) => result.push_str(" parquet: {}"),
                                            FileFormat::Arrow(_) => result.push_str(" arrow: {}"),
                                            FileFormat::Orc(_) => result.push_str(" orc: {}"),
                                            FileFormat::Dwrf(_) => result.push_str(" dwrf: {}"),
                                            FileFormat::Text(_) => result.push_str(" text: {}"),
                                            FileFormat::Extension(_) => result.push_str(" extension: {}"),
                                        }
                                    }

                                    result.push_str("}\n");
                                }

                                result.push_str("  ]\n");
                                result.push_str("}\n\n");
                            }
                        }
                    }
                }
                SourceType::NamedTable => {
                    if let Some(blob_lock) = &source.blob {
                        if let Ok(blob_data) = blob_lock.lock() {
                            if let Some(named_table) = blob_data.downcast_ref::<NamedTable>() {
                                result.push_str(&format!("source named_table {} {{\n", source.name()));
                                result.push_str("  names = [");
                                for (i, name) in named_table.names.iter().enumerate() {
                                    if i > 0 {
                                        result.push_str(", ");
                                    }
                                    result.push_str(&format!("\"{}\"", name));
                                }
                                result.push_str("]\n");
                                result.push_str("}\n\n");
                            }
                        }
                    }
                }
                SourceType::VirtualTable => {
                    if let Some(blob_lock) = &source.blob {
                        if let Ok(blob_data) = blob_lock.lock() {
                            if let Some(_virtual_table) = blob_data.downcast_ref::<VirtualTable>() {
                                result.push_str(&format!("source virtual_table {} {{\n", source.name()));
                                // TODO: Print virtual table values
                                result.push_str("  // Virtual table values not yet implemented\n");
                                result.push_str("}\n\n");
                            }
                        }
                    }
                }
                SourceType::ExtensionTable => {
                    if let Some(blob_lock) = &source.blob {
                        if let Ok(blob_data) = blob_lock.lock() {
                            if let Some(_extension_table) = blob_data.downcast_ref::<ExtensionTable>() {
                                result.push_str(&format!("source extension_table {} {{\n", source.name()));
                                result.push_str("  // Extension table details not yet implemented\n");
                                result.push_str("}\n\n");
                            }
                        }
                    }
                }
                _ => {}
            }
        }

        Ok(())
    }

    /// Processes extension space (functions).
    fn process_extension_space(
        &self,
        symbol_table: &SymbolTable,
        result: &mut String,
    ) -> Result<(), TextPlanError> {
        let functions: Vec<_> = symbol_table
            .symbols()
            .iter()
            .filter(|s| s.symbol_type() == SymbolType::Function)
            .cloned()
            .collect();

        if functions.is_empty() {
            return Ok(());
        }

        result.push_str("extension_space {\n");

        for func in functions {
            // Format: function name:signature as alias;
            result.push_str(&format!("  function {} as {};\n", func.name(), func.name()));
        }

        result.push_str("}\n");

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
