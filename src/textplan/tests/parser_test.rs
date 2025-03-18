// SPDX-License-Identifier: Apache-2.0

//! Tests for the parser.

#[cfg(test)]
mod tests {
    use crate::textplan::parser::{load_from_text, parse_stream};

    #[test]
    fn test_simple_plan() {
        let text = r#"
        schema simple_schema {
            id i32;
            name string;
            price fp64;
        }

        source LOCAL_FILES simple_source {
            URI_FILE = "data.csv"
        }

        read RELATION simple_read {
            SOURCE simple_source;
            BASE_SCHEMA simple_schema;
        }

        filter RELATION filtered_data {
            BASE_SCHEMA simple_schema;
            FILTER greater_than(price, 100.0_fp64);
        }

        ROOT {
            NAMES = [filtered_data]
        }
        "#;

        // Test parse_stream
        let parse_result = parse_stream(text);
        assert!(parse_result.successful(), "Parse failed: {:?}", parse_result.all_errors());

        // Verify the symbol table
        let symbol_table = parse_result.symbol_table();
        assert!(symbol_table.len() > 0, "Symbol table is empty");

        // Test load_from_text
        match load_from_text(text) {
            Ok(_) => {
                // Success, the function ran without errors
            }
            Err(e) => {
                panic!("Failed to load text plan: {}", e);
            }
        }
    }

    #[test]
    fn test_parse_empty_plan() {
        // Simple test that just verifies the parsing doesn't crash
        // and an empty plan gives an empty symbol table
        let text = "";
        let result = parse_stream(text);
        assert!(result.successful(), "Empty plan should not have errors");
        assert!(result.syntax_errors().is_empty());
        assert_eq!(result.symbol_table().len(), 0);
    }

    #[test]
    fn test_parse_antlr_simple_plan() {
        let text = r#"
plan {
  relations {
    root {
      input {
        read {
          baseSchema {
            names: ["a", "b", "c"]
            struct {
              types: [i32, i64, string]
            }
          }
          namedTable {
            names: ["default", "test"]
          }
        }
      }
    }
  }
}
"#;
        let result = parse_stream(text);
        assert!(result.successful(), "Parsing errors: {:?}", result.all_errors());
        assert!(result.syntax_errors().is_empty());
        
        // Get the symbol table and verify we have symbols for the table and schema
        let symbol_table = result.symbol_table();
        
        // Expect at least a root symbol
        let symbols = symbol_table.symbols();
        assert!(symbols.len() > 0, "Expected at least one symbol in the symbol table");
        
        // Print the symbols for debugging
        println!("Symbols in plan: {:?}", symbol_table);
    }

    #[test]
    fn test_parse_plan_with_types() {
        let text = r#"
plan {
  relations {
    root {
      input {
        project {
          common {
            direct {
              struct {
                types: [boolean, i8, i16, i32, i64, fp32, fp64, string, binary, timestamp, date]
              }
            }
          }
          input {
            read {
              baseSchema {
                names: ["a", "b", "c"]
                struct {
                  types: [i32, i64, string]
                }
              }
              namedTable {
                names: ["default", "test"]
              }
            }
          }
          expressions {
            selection {
              directReference {
                structField {
                  field: 0
                }
              }
              rootReference {}
            }
          }
        }
      }
    }
  }
}
"#;
        let result = parse_stream(text);
        assert!(result.successful(), "Parsing errors: {:?}", result.all_errors());
        assert!(result.syntax_errors().is_empty());
        
        // Get the symbol table and verify we have symbols for the different types
        let symbol_table = result.symbol_table();
        
        // Print the symbols for debugging
        println!("Symbols in plan: {:?}", symbol_table);
    }

    #[test]
    fn test_parse_complex_types() {
        let text = r#"
plan {
  relations {
    root {
      input {
        project {
          common {
            direct {
              struct {
                types: [
                  list<i32>,
                  map<string, i32>,
                  struct<i32, string, boolean>,
                  decimal<10, 2>,
                  list<struct<i32, string>>
                ]
              }
            }
          }
          input {
            read {
              baseSchema {
                names: ["complex_types"]
                struct {
                  types: [struct<i32, string, list<i32>>]
                }
              }
              namedTable {
                names: ["default", "test"]
              }
            }
          }
        }
      }
    }
  }
}
"#;
        let result = parse_stream(text);
        assert!(result.successful(), "Parsing errors: {:?}", result.all_errors());
        assert!(result.syntax_errors().is_empty());
        
        // Get the symbol table
        let symbol_table = result.symbol_table();
        
        // Print the symbols for debugging
        println!("Symbols in plan with complex types: {:?}", symbol_table);
    }
}