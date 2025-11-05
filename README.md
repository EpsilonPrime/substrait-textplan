# Substrait TextPlan - Rust Library

This is a Rust implementation of the Substrait TextPlan library, designed to be used both directly from Rust and from other languages through FFI bindings.

## Overview

The Substrait TextPlan library provides functionality for parsing, loading, and converting Substrait plans in both text and binary formats. This Rust implementation mirrors the C++ implementation in the main repository, with some adaptations to make it more idiomatic in Rust.

## Features

- Parse textplans into a symbol table representation
- Convert textplans to binary protobuf plans
- Convert binary protobuf plans to textplans
- FFI bindings for C/C++ and Python
- Uses the same ANTLR grammar as the C++ implementation

## Building

### Requirements

- Rust (latest stable)
- Cargo
- Java JRE (for ANTLR grammar processing)

### Build Steps

Build the library:

```bash
cargo build
```

Build in release mode:

```bash
cargo build --release
```

Run tests:

```bash
cargo test
```

## Using from Rust

```rust
use substrait_textplan::textplan::parser::load_from_text;

fn main() {
    let text = r#"
        schema simple_schema {
            id i32;
            name string;
            price fp64;
        }

        source LOCAL_FILES simple_source {
            ITEMS = [
                {
                    URI_FILE: "data.csv"
                }
            ]
        }

        read RELATION simple_read {
            SOURCE simple_source;
            BASE_SCHEMA simple_schema;
        }

        filter RELATION filtered_data {
            BASE_SCHEMA simple_schema;
            FILTER greater_than(price, 100.0_fp64);
        }

        simple_read -> filtered_data

        ROOT {
            NAMES = [filtered_data]
        }
    "#;

    match load_from_text(text) {
        Ok(binary_plan) => {
            println!("Successfully parsed plan: {} bytes", binary_plan.len());
        }
        Err(e) => {
            eprintln!("Error parsing plan: {}", e);
        }
    }
}
```

## Using from C++

Include the header file:

```cpp
#include "substrait_textplan.h"
```

Use the TextPlan class:

```cpp
#include <iostream>
#include "substrait_textplan.h"

int main() {
    std::string text = R"(
        schema simple_schema {
            id i32;
            name string;
            price fp64;
        }

        read RELATION simple_read {
            SOURCE simple_source;
            BASE_SCHEMA simple_schema;
        }
    )";

    auto binary_plan = substrait::textplan::TextPlan::LoadFromText(text.c_str());
    if (!binary_plan.has_value()) {
        std::cerr << "Error parsing plan" << std::endl;
        return 1;
    }

    std::cout << "Successfully parsed plan: " << binary_plan->size() << " bytes" << std::endl;

    // Convert back to text (binary_plan contains serialized substrait::Plan protobuf)
    auto text_plan = substrait::textplan::TextPlan::SaveToText(*binary_plan);
    if (text_plan.has_value()) {
        std::cout << "Textplan:\n" << *text_plan << std::endl;
    }

    return 0;
}
```

## Using from Python

```python
import substrait_textplan

text = """
    schema simple_schema {
        id i32;
        name string;
        price fp64;
    }

    source LOCAL_FILES simple_source {
        ITEMS = [
            {
                URI_FILE: "data.csv"
            }
        ]
    }

    read RELATION simple_read {
        SOURCE simple_source;
        BASE_SCHEMA simple_schema;
    }
"""

binary_plan = substrait_textplan.load_from_text(text)
if binary_plan is None:
    print("Error parsing plan")
else:
    print(f"Successfully parsed plan: {len(binary_plan)} bytes")
```

## ANTLR Grammar

This implementation is transitioning from tree-sitter to ANTLR4 for parsing, using the same ANTLR grammar as the C++ implementation. The grammar files are located in:

- `src/substrait/textplan/parser/grammar/SubstraitPlanLexer.g4`
- `src/substrait/textplan/parser/grammar/SubstraitPlanParser.g4`

### Generating ANTLR4 Code

To generate the ANTLR4 parser code:

1. Install Java
2. Download the special ANTLR4 tool with Rust support:
   ```bash
   curl -L -o antlr4rust.jar https://github.com/rrevenantt/antlr4rust/releases/download/antlr4-4.8-2-Rust-0.3.0-beta/antlr4-4.8-2-SNAPSHOT-complete.jar
   ```
   
   **Important:** You MUST use this specific JAR file with Rust support. The standard ANTLR4 JAR file does NOT support generating Rust code!

3. Set the ANTLR_JAR environment variable and run the code generation:
   ```bash
   export ANTLR_JAR=/path/to/antlr4rust.jar
   GENERATE_ANTLR=true cargo build
   ```

The generated code will be placed in `src/textplan/parser/antlr/` and will be used by the parser to parse textplans.

## Implementation Status

This is an implementation of the Substrait TextPlan library in Rust. The current status is:

- [x] Basic structure and FFI bindings
- [x] Symbol table implementation
- [x] Tree-sitter parser integration
- [ ] ANTLR4 parser integration (in progress)
  - [x] Setup for antlr-rust
  - [x] Grammar files from C++ implementation
  - [x] Build-time code generation setup
  - [ ] Visitor implementations
  - [ ] Full ANTLR4 parser integration
- [x] Protobuf integration
- [ ] Full roundtrip parsing and generation

## License

Apache-2.0