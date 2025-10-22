// SPDX-License-Identifier: Apache-2.0

//! Symbol table for tracking entities in a textplan.

use std::any::Any;
use std::collections::{HashMap, HashSet};
use std::fmt;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::sync::{Arc, Mutex};

use crate::textplan::common::Location;
use crate::textplan::common::UnknownLocation;
use crate::textplan::TextLocation;

/// Types of symbols in the symbol table.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SymbolType {
    /// An extension space, such as "iceberg".
    ExtensionSpace,
    /// A function, such as "count".
    Function,
    /// A relation in a plan.
    PlanRelation,
    /// A relation, such as a join or a filter.
    Relation,
    /// A schema.
    Schema,
    /// A column in a schema.
    SchemaColumn,
    /// A source.
    Source,
    /// A detail about a source.
    SourceDetail,
    /// A field.
    Field,
    /// The root of a plan.
    Root,
    /// A table.
    Table,
    /// A measure.
    Measure,
    /// An unknown symbol type.
    Unknown,
}

impl fmt::Display for SymbolType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SymbolType::ExtensionSpace => write!(f, "ExtensionSpace"),
            SymbolType::Function => write!(f, "Function"),
            SymbolType::PlanRelation => write!(f, "PlanRelation"),
            SymbolType::Relation => write!(f, "Relation"),
            SymbolType::Schema => write!(f, "Schema"),
            SymbolType::SchemaColumn => write!(f, "SchemaColumn"),
            SymbolType::Source => write!(f, "Source"),
            SymbolType::SourceDetail => write!(f, "SourceDetail"),
            SymbolType::Field => write!(f, "Field"),
            SymbolType::Root => write!(f, "Root"),
            SymbolType::Table => write!(f, "Table"),
            SymbolType::Measure => write!(f, "Measure"),
            SymbolType::Unknown => write!(f, "Unknown"),
        }
    }
}

/// Types of relations in a plan.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RelationType {
    // Logical relations
    Unknown,
    Read,
    Project,
    Join,
    Cross,
    Fetch,
    Aggregate,
    Sort,
    Filter,
    Set,

    // Physical relations
    HashJoin,
    MergeJoin,

    // Write relations
    Exchange,
    Ddl,
    Write,

    // Extension relations
    ExtensionLeaf,
    ExtensionSingle,
    ExtensionMulti,
}

/// Types of relations in a plan.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SourceType {
    // Logical relations
    Unknown,
    LocalFiles,
    NamedTable,
    VirtualTable,
    ExtensionTable,
    IcebergTable,
}

/// Information about a symbol in the symbol table.
#[derive(Debug)]
pub struct SymbolInfo {
    /// The name of the symbol.
    name: String,
    /// An optional alias for the symbol.
    alias: Option<String>,
    /// The location of the symbol in the source text.
    source_location: Box<dyn Location>,
    /// A permanent location for the symbol (if different from source_location).
    /// TODO: This field is obsolete in the Rust implementation since locations are tracked
    /// through traversal. Consider removing this field and related methods.
    permanent_location: Box<dyn Location>,
    /// The location of the parent query, if this symbol is in a subquery.
    parent_query_location: Box<dyn Location>,
    /// The index of the parent query, if this symbol is in a subquery.
    parent_query_index: i32,
    /// The type of the symbol.
    symbol_type: SymbolType,
    /// Additional type information for the symbol.
    subtype: Option<Box<dyn Any + Send + Sync>>,
    /// Additional data for the symbol.
    pub blob: Option<Arc<Mutex<dyn Any + Send + Sync>>>,
    /// The schema associated with this symbol, if any.
    schema: Option<Arc<SymbolInfo>>,
}

impl SymbolInfo {
    /// Creates a new symbol info.
    pub fn new<L: Into<Box<dyn Location>>>(
        name: String,
        location: L,
        symbol_type: SymbolType,
        subtype: Option<Box<dyn Any + Send + Sync>>,
        blob: Option<Arc<Mutex<dyn Any + Send + Sync>>>,
    ) -> Self {
        Self {
            name,
            alias: None,
            source_location: location.into(),
            permanent_location: Box::new(TextLocation::UNKNOWN_LOCATION),
            parent_query_location: Box::new(TextLocation::UNKNOWN_LOCATION),
            parent_query_index: -1,
            symbol_type,
            subtype,
            blob,
            schema: None,
        }
    }

    /// Returns the name of the symbol.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the alias of the symbol, if any.
    pub fn alias(&self) -> Option<&str> {
        self.alias.as_deref()
    }

    /// Returns the display name of the symbol (alias if present, otherwise name).
    pub fn display_name(&self) -> &str {
        self.alias.as_deref().unwrap_or(&self.name)
    }

    /// Returns a reference to the location of the symbol in the source text.
    pub fn source_location(&self) -> &dyn Location {
        self.source_location.as_ref()
    }

    /// Returns a reference to the permanent location of the symbol.
    pub fn permanent_location(&self) -> &dyn Location {
        self.permanent_location.as_ref()
    }

    /// Returns a reference to the location of the parent query, if this symbol is in a subquery.
    pub fn parent_query_location(&self) -> &dyn Location {
        self.parent_query_location.as_ref()
    }

    /// Returns the index of the parent query, if this symbol is in a subquery.
    pub fn parent_query_index(&self) -> i32 {
        self.parent_query_index
    }

    /// Returns the type of the symbol.
    pub fn symbol_type(&self) -> SymbolType {
        self.symbol_type
    }

    /// Sets the alias of the symbol.
    pub fn set_alias(&mut self, alias: String) {
        self.alias = Some(alias);
    }

    /// Sets the schema associated with this symbol.
    pub fn set_schema(&mut self, schema: Arc<SymbolInfo>) {
        self.schema = Some(schema);
    }

    /// Sets the permanent location of the symbol.
    pub fn set_permanent_location<L: Into<Box<dyn Location>>>(&mut self, location: L) {
        self.permanent_location = location.into();
    }

    /// Sets the location of the parent query.
    pub fn set_parent_query_location<L: Into<Box<dyn Location>>>(&mut self, location: L) {
        self.parent_query_location = location.into();
    }

    /// Sets the index of the parent query.
    pub fn set_parent_query_index(&mut self, index: i32) {
        self.parent_query_index = index;
    }

    /// Sets the subtype of the symbol.
    pub fn set_subtype(&mut self, subtype: Box<dyn Any + Send + Sync>) {
        self.subtype = Some(subtype);
    }

    /// Sets the blob of the symbol.
    pub fn set_blob(&mut self, blob: Box<dyn Any + Send + Sync>) {
        self.blob = Some(Arc::new(Mutex::new(blob)));
    }

    /// Returns the schema associated with this symbol, if any.
    pub fn schema(&self) -> Option<&Arc<SymbolInfo>> {
        self.schema.as_ref()
    }

    /// Returns the subtype of the symbol, if any.
    pub fn subtype<T: 'static>(&self) -> Option<&T> {
        self.subtype.as_ref().and_then(|s| s.downcast_ref::<T>())
    }

    /// Provides access to the blob of the symbol through the mutex, if any.
    pub fn with_blob<T: 'static, F, R>(&self, f: F) -> Option<R>
    where
        F: FnOnce(&mut T) -> R,
    {
        self.blob.as_ref().and_then(|b| {
            let mut guard = b.lock().ok()?;
            guard.downcast_mut::<T>().map(|value| f(value))
        })
    }
}

impl PartialEq for SymbolInfo {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
            && self.alias == other.alias
            && self.source_location.equals(other.source_location.as_ref())
            && self
                .permanent_location
                .equals(other.permanent_location.as_ref())
            && self
                .parent_query_location
                .equals(other.parent_query_location.as_ref())
            && self.parent_query_index == other.parent_query_index
            && self.symbol_type == other.symbol_type
    }
}

impl Eq for SymbolInfo {}

/// A symbol table for tracking entities in a textplan.
///
/// The symbol table is used to track symbols defined in a textplan, such as
/// relations, schemas, functions, etc. It provides methods for defining symbols,
/// looking them up by name or location, and iterating over all symbols.
#[derive(Debug, Default, Clone)]
pub struct SymbolTable {
    /// A map from symbol name to index in the symbols vector.
    names: HashMap<String, usize>,
    /// The symbols in the table.
    symbols: Vec<Arc<SymbolInfo>>,
    /// A map from alias to index in the symbols vector.
    aliases: HashMap<String, usize>,
    /// A map from location hash to indices in the symbols vector.
    locations: HashMap<u64, Vec<usize>>,
}

impl SymbolTable {
    /// Creates a new empty symbol table.
    pub fn new() -> Self {
        Self {
            names: HashMap::new(),
            symbols: Vec::new(),
            aliases: HashMap::new(),
            locations: HashMap::new(),
        }
    }

    /// Returns a unique name for a symbol.
    ///
    /// If the given name is already used, a number is appended to make it unique.
    pub fn get_unique_name(&self, base_name: &str) -> String {
        if !self.names.contains_key(base_name) {
            return base_name.to_string();
        }

        let mut i = 1;
        loop {
            let name = format!("{}{}", base_name, i);
            if !self.names.contains_key(&name) {
                return name;
            }
            i += 1;
        }
    }

    /// Defines a new symbol in the symbol table.
    ///
    /// Returns a reference to the newly defined symbol.
    pub fn define_symbol<L: Into<Box<dyn Location>>>(
        &mut self,
        name: String,
        location: L,
        symbol_type: SymbolType,
        subtype: Option<Box<dyn Any + Send + Sync>>,
        blob: Option<Arc<Mutex<dyn Any + Send + Sync>>>,
    ) -> Arc<SymbolInfo> {
        let symbol = SymbolInfo::new(name.clone(), location, symbol_type, subtype, blob);

        // Add it to the symbols vector
        let index = self.symbols.len();
        let symbol = Arc::new(symbol);
        self.symbols.push(symbol.clone());

        // Add it to the names map
        self.names.insert(name, index);

        // Add it to the locations map using the location hash
        let location_hash = symbol.source_location().location_hash();

        self.locations
            .entry(location_hash)
            .or_insert_with(Vec::new)
            .push(index);

        symbol
    }

    /// Gets a mutable reference to a symbol in the symbol table.
    ///
    /// This is a convenience method for when you need to modify a symbol after it's been added.
    /// It should be used with care, as it requires interior mutability to work with Arc.
    ///
    /// # Arguments
    ///
    /// * `symbol` - A reference to the symbol to get a mutable reference to.
    ///
    /// # Returns
    ///
    /// A mutable reference to the symbol's internal SymbolInfo, or None if not found.
    pub fn get_mutable_symbol(&mut self, symbol: &Arc<SymbolInfo>) -> Option<&mut SymbolInfo> {
        // Try to find the symbol by name
        if let Some(&index) = self.names.get(symbol.name()) {
            // Check if this is the right symbol
            if Arc::ptr_eq(&self.symbols[index], symbol) {
                // Get a mutable reference to the symbol
                // This is safe because we're the only mutable reference to the SymbolTable
                let symbol_ref = Arc::get_mut(&mut self.symbols[index])?;
                return Some(symbol_ref);
            }
        }

        // Symbol wasn't found
        None
    }

    /// Defines a new symbol with a unique name.
    ///
    /// This is a convenience method that calls get_unique_name and then define_symbol.
    pub fn define_unique_symbol<L: Into<Box<dyn Location>>>(
        &mut self,
        base_name: &str,
        location: L,
        symbol_type: SymbolType,
        subtype: Option<Box<dyn Any + Send + Sync>>,
        blob: Option<Arc<Mutex<dyn Any + Send + Sync>>>,
    ) -> Arc<SymbolInfo> {
        let name = self.get_unique_name(base_name);
        self.define_symbol(name, location, symbol_type, subtype, blob)
    }

    /// Adds an alias for a symbol.
    pub fn add_alias(&mut self, alias: String, symbol: &Arc<SymbolInfo>) {
        // Find the index of the symbol
        if let Some(&index) = self.names.get(symbol.name()) {
            // Mutate the symbol to add the alias
            let mut_symbol = Arc::get_mut(self.symbols.get_mut(index).unwrap()).unwrap();
            mut_symbol.set_alias(alias.clone());

            // Add it to the aliases map
            self.aliases.insert(alias, index);
        }
    }

    /// Sets the permanent location for a symbol.
    pub fn add_permanent_location<L: Into<Box<dyn Location>>>(
        &mut self,
        symbol: &Arc<SymbolInfo>,
        location: L,
    ) {
        // Find the index of the symbol
        if let Some(&index) = self.names.get(symbol.name()) {
            // Mutate the symbol to set the permanent location
            let mut_symbol = Arc::get_mut(self.symbols.get_mut(index).unwrap()).unwrap();
            mut_symbol.set_permanent_location(location);

            // Add it to the locations map using the location hash
            let location_hash = mut_symbol.permanent_location().location_hash();

            self.locations
                .entry(location_hash)
                .or_insert_with(Vec::new)
                .push(index);
        }
    }

    /// Sets the parent query location for a symbol.
    pub fn set_parent_query_location<L: Into<Box<dyn Location>>>(
        &mut self,
        symbol: &Arc<SymbolInfo>,
        location: L,
    ) {
        // Find the index of the symbol
        if let Some(&index) = self.names.get(symbol.name()) {
            // Mutate the symbol to set the parent query location
            let mut_symbol = Arc::get_mut(self.symbols.get_mut(index).unwrap()).unwrap();
            mut_symbol.set_parent_query_location(location);

            // Note: We don't add an entry to the locations map for parent query locations
            // since they're not expected to be looked up directly, but through the parent query index
        }
    }

    /// Sets the parent query index for a symbol.
    pub fn set_parent_query_index(&mut self, symbol: &Arc<SymbolInfo>, index: i32) {
        // Find the index of the symbol
        if let Some(&symbol_index) = self.names.get(symbol.name()) {
            // Mutate the symbol to set the parent query index
            let mut_symbol = Arc::get_mut(self.symbols.get_mut(symbol_index).unwrap()).unwrap();
            mut_symbol.set_parent_query_index(index);
        }
    }

    /// Looks up a symbol by name.
    pub fn lookup_symbol_by_name(&self, name: &str) -> Option<Arc<SymbolInfo>> {
        self.names
            .get(name)
            .or_else(|| self.aliases.get(name))
            .map(|&index| self.symbols[index].clone())
    }

    /// Looks up symbols by location.
    pub fn lookup_symbols_by_location(&self, location: &dyn Location) -> Vec<Arc<SymbolInfo>> {
        // Calculate the location hash
        let location_hash = location.location_hash();

        self.locations
            .get(&location_hash)
            .map(|indices| {
                indices
                    .iter()
                    .map(|&index| self.symbols[index].clone())
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Looks up a symbol by location and type.
    pub fn lookup_symbol_by_location_and_type(
        &self,
        location: &dyn Location,
        symbol_type: SymbolType,
    ) -> Option<Arc<SymbolInfo>> {
        // Calculate the location hash
        let location_hash = location.location_hash();

        self.locations.get(&location_hash).and_then(|indices| {
            indices
                .iter()
                .map(|&index| self.symbols[index].clone())
                .find(|symbol| symbol.symbol_type() == symbol_type)
        })
    }

    /// Looks up a symbol by location and any of the given types.
    pub fn lookup_symbol_by_location_and_types(
        &self,
        location: &dyn Location,
        types: &HashSet<SymbolType>,
    ) -> Option<Arc<SymbolInfo>> {
        // Calculate the location hash
        let location_hash = location.location_hash();

        self.locations.get(&location_hash).and_then(|indices| {
            indices
                .iter()
                .map(|&index| self.symbols[index].clone())
                .find(|symbol| types.contains(&symbol.symbol_type()))
        })
    }

    /// Looks up a symbol by parent query location, index, and type.
    pub fn lookup_symbol_by_parent_query_and_type(
        &self,
        location: &dyn Location,
        index: i32,
        symbol_type: SymbolType,
    ) -> Option<Arc<SymbolInfo>> {
        // Calculate the location hash
        let location_hash = location.location_hash();

        self.symbols
            .iter()
            .find(|symbol| {
                symbol.parent_query_location().location_hash() == location_hash
                    && symbol.parent_query_index() == index
                    && symbol.symbol_type() == symbol_type
            })
            .cloned()
    }

    /// Returns the nth symbol of the given type.
    ///
    /// Panics if n is out of bounds.
    pub fn nth_symbol_by_type(&self, n: usize, symbol_type: SymbolType) -> Arc<SymbolInfo> {
        let mut count = 0;
        for symbol in &self.symbols {
            if symbol.symbol_type() == symbol_type {
                if count == n {
                    return symbol.clone();
                }
                count += 1;
            }
        }
        panic!("No symbol of type {} at index {}", symbol_type, n);
    }

    /// Returns all symbols.
    pub fn symbols(&self) -> &[Arc<SymbolInfo>] {
        &self.symbols
    }

    /// Returns the number of symbols.
    pub fn len(&self) -> usize {
        self.symbols.len()
    }

    /// Returns true if the symbol table is empty.
    pub fn is_empty(&self) -> bool {
        self.symbols.is_empty()
    }
}

impl fmt::Display for SymbolTable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{")?;
        let mut first = true;
        for symbol in &self.symbols {
            if !first {
                write!(f, ", ")?;
            }
            write!(f, "{}", symbol.name())?;
            first = false;
        }
        write!(f, "}}")
    }
}

// TODO -- Consider moving these into the converter.

// Extension methods for converter functionality
impl SymbolTable {
    /// Adds a root relation to the symbol table.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the root relation.
    ///
    /// # Returns
    ///
    /// A reference to the newly created symbol.
    pub fn add_root_relation(&mut self, name: &str) -> Arc<SymbolInfo> {
        let location = UnknownLocation::UNKNOWN;
        let rel_type = Box::new(RelationType::Unknown);
        self.define_symbol(
            name.to_string(),
            location,
            SymbolType::Root,
            Some(rel_type),
            None,
        )
    }

    /// Adds a relation to the symbol table.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the relation.
    ///
    /// # Returns
    ///
    /// A reference to the newly created symbol.
    pub fn add_relation(&mut self, name: &str) -> Arc<SymbolInfo> {
        let location = UnknownLocation::UNKNOWN;
        let rel_type = Box::new(RelationType::Unknown);
        self.define_symbol(
            name.to_string(),
            location,
            SymbolType::Relation,
            Some(rel_type),
            None,
        )
    }

    /// Adds a relation with a specific type to the symbol table.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the relation.
    /// * `rel_type` - The type of the relation.
    ///
    /// # Returns
    ///
    /// A reference to the newly created symbol.
    pub fn add_relation_with_type(
        &mut self,
        name: &str,
        rel_type: RelationType,
    ) -> Arc<SymbolInfo> {
        let location = UnknownLocation::UNKNOWN;
        let rel_type_box = Box::new(rel_type);
        self.define_symbol(
            name.to_string(),
            location,
            SymbolType::Relation,
            Some(rel_type_box),
            None,
        )
    }

    /// Adds a field mapping to the symbol table.
    ///
    /// # Arguments
    ///
    /// * `relation_name` - The name of the relation the field belongs to.
    /// * `field_index` - The index of the field in the relation.
    ///
    /// # Returns
    ///
    /// A reference to the newly created symbol, or None if the relation was not found.
    pub fn add_field_mapping(
        &mut self,
        relation_name: &str,
        field_index: i32,
    ) -> Option<Arc<SymbolInfo>> {
        // Look up the relation symbol
        let _relation = self.lookup_symbol_by_name(relation_name)?;

        // Create a field name
        let field_name = format!("{}.field_{}", relation_name, field_index);
        let location = UnknownLocation::UNKNOWN;

        // Define the field symbol
        let field = self.define_symbol(
            field_name,
            location,
            SymbolType::Field,
            None,
            Some(Arc::new(Mutex::new(field_index))),
        );

        Some(field)
    }

    /// Adds a named table to the symbol table.
    ///
    /// # Arguments
    ///
    /// * `relation_name` - The name of the relation the table belongs to.
    /// * `table_names` - The names of the table (usually catalog, schema, table).
    ///
    /// # Returns
    ///
    /// A reference to the newly created symbol, or None if the relation was not found.
    pub fn add_named_table(
        &mut self,
        relation_name: &str,
        table_names: &[String],
    ) -> Option<Arc<SymbolInfo>> {
        // Look up the relation symbol
        let _relation = self.lookup_symbol_by_name(relation_name)?;

        // Create a full table name
        let table_name = if table_names.is_empty() {
            format!("{}.table", relation_name)
        } else {
            format!("{}.{}", relation_name, table_names.join("."))
        };

        let location = UnknownLocation::UNKNOWN;

        // Define the table symbol
        let table = self.define_symbol(
            table_name,
            location,
            SymbolType::Table,
            None,
            Some(Arc::new(Mutex::new(table_names.to_vec()))),
        );

        Some(table)
    }

    /// Adds a file source to the symbol table.
    ///
    /// # Arguments
    ///
    /// * `relation_name` - The name of the relation the source belongs to.
    /// * `uri` - The URI of the file.
    ///
    /// # Returns
    ///
    /// A reference to the newly created symbol, or None if the relation was not found.
    pub fn add_file_source(&mut self, relation_name: &str, uri: &str) -> Option<Arc<SymbolInfo>> {
        // Look up the relation symbol
        let _relation = self.lookup_symbol_by_name(relation_name)?;

        // Create a source name
        let source_name = format!("{}.file", relation_name);
        let location = UnknownLocation::UNKNOWN;

        // Define the source symbol
        let source = self.define_symbol(
            source_name,
            location,
            SymbolType::Source,
            None,
            Some(Arc::new(Mutex::new(uri.to_string()))),
        );

        Some(source)
    }

    /// Adds a folder source to the symbol table.
    ///
    /// # Arguments
    ///
    /// * `relation_name` - The name of the relation the source belongs to.
    /// * `uri` - The URI of the folder.
    ///
    /// # Returns
    ///
    /// A reference to the newly created symbol, or None if the relation was not found.
    pub fn add_folder_source(&mut self, relation_name: &str, uri: &str) -> Option<Arc<SymbolInfo>> {
        // Look up the relation symbol
        let _relation = self.lookup_symbol_by_name(relation_name)?;

        // Create a source name
        let source_name = format!("{}.folder", relation_name);
        let location = UnknownLocation::UNKNOWN;

        // Define the source symbol
        let source = self.define_symbol(
            source_name,
            location,
            SymbolType::Source,
            None,
            Some(Arc::new(Mutex::new(uri.to_string()))),
        );

        Some(source)
    }

    /// Adds a string literal to the symbol table.
    ///
    /// # Arguments
    ///
    /// * `relation_name` - The name of the relation the literal belongs to.
    /// * `value` - The string value.
    ///
    /// # Returns
    ///
    /// A reference to the newly created symbol, or None if the relation was not found.
    pub fn add_string_literal(
        &mut self,
        relation_name: &str,
        value: &str,
    ) -> Option<Arc<SymbolInfo>> {
        // Look up the relation symbol
        let _relation = self.lookup_symbol_by_name(relation_name)?;

        // Create a literal name (use a hash of the value for uniqueness)
        let mut hasher = DefaultHasher::new();
        value.hash(&mut hasher);
        let hash = format!("{:x}", hasher.finish());
        let literal_name = format!("{}.string_{}", relation_name, hash);
        let location = UnknownLocation::UNKNOWN;

        // Define the literal symbol
        let literal = self.define_symbol(
            literal_name,
            location,
            SymbolType::Field,
            None,
            Some(Arc::new(Mutex::new(value.to_string()))),
        );

        Some(literal)
    }
}
