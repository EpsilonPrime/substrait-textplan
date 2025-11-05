# Substrait TextPlan - C++ Wrapper

This directory contains a C++ wrapper for the Substrait TextPlan library, which provides functionality for parsing and converting between text and binary Substrait plan formats.

## Overview

The C++ wrapper provides a clean, modern C++ API around the Rust-based Substrait TextPlan library. It uses RAII for automatic memory management and provides both static and instance methods for ease of use.

## Features

- Parse textplans into binary protobuf format
- Convert binary protobuf plans to textplans
- Modern C++17 API with RAII memory management
- No manual memory management required
- Static and instance methods for flexibility
- Thread-safe operations

## Requirements

- C++ compiler with C++17 support (GCC 7+, Clang 5+, MSVC 2017+)
- CMake 3.16 or later
- Rust toolchain (to build the underlying library)
- The Substrait TextPlan Rust library (built from the parent directory)

## Building

### Step 1: Build the Rust Library

First, build the Rust library from the parent directory:

```bash
cd ..
cargo build --release
```

This will create the shared library at `target/release/libsubstrait_textplan.so` (Linux), `target/release/libsubstrait_textplan.dylib` (macOS), or `target/release/substrait_textplan.dll` (Windows).

### Step 2: Build the C++ Wrapper

From this directory (`cpp/`), create a build directory and run CMake:

```bash
mkdir build
cd build
cmake ..
make
```

This will build:
- The C++ wrapper library (`libsubstrait_textplan_cpp`)
- Example programs (`simple_example` and `roundtrip_example`)

### Custom Library Location

If you want to use a different location for the Rust library, set the `SUBSTRAIT_TEXTPLAN_LIB_DIR` environment variable:

```bash
export SUBSTRAIT_TEXTPLAN_LIB_DIR=/path/to/rust/library
cmake ..
make
```

## Running Examples

After building, you can run the example programs:

```bash
# Simple example showing basic usage
./examples/simple_example

# Roundtrip example showing text->binary->text conversion
./examples/roundtrip_example
```

## Usage

### Basic Usage

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

    ROOT {
      NAMES = [simple_read]
    }
  )";

  // Convert textplan to binary using static method
  auto binary_plan = substrait::textplan::TextPlan::LoadFromText(text.c_str());

  if (!binary_plan.has_value()) {
    std::cerr << "Error parsing textplan" << std::endl;
    return 1;
  }

  std::cout << "Successfully parsed plan: " << binary_plan->size() << " bytes"
            << std::endl;

  return 0;
}
```

### Using Instance Methods

```cpp
#include "substrait_textplan.h"

int main() {
  substrait::textplan::TextPlan tp;

  // Load from text
  auto binary_plan = tp.LoadFromText("schema my_schema { id i32; }");

  if (binary_plan.has_value()) {
    // Convert back to text
    auto text_plan = tp.SaveToText(*binary_plan);

    if (text_plan.has_value()) {
      std::cout << *text_plan << std::endl;
    }
  }

  return 0;
}
```

### Roundtrip Conversion

```cpp
#include "substrait_textplan.h"

int main() {
  std::string original_text = "..."; // Your textplan

  // Text -> Binary
  auto binary = substrait::textplan::TextPlan::LoadFromText(original_text.c_str());

  if (binary.has_value()) {
    // Binary -> Text
    auto regenerated_text = substrait::textplan::TextPlan::SaveToText(*binary);

    if (regenerated_text.has_value()) {
      // Use the regenerated textplan
      std::cout << *regenerated_text << std::endl;
    }
  }

  return 0;
}
```

## API Reference

### Class: `substrait::textplan::TextPlan`

#### Static Methods

- `static std::optional<std::vector<uint8_t>> LoadFromText(const char* text)`
  - Converts a textplan string to binary protobuf format
  - Returns `std::nullopt` on error

- `static std::optional<std::string> SaveToText(const uint8_t* data, size_t size)`
  - Converts a binary protobuf plan to textplan format
  - Returns `std::nullopt` on error

#### Instance Methods

- `std::optional<std::vector<uint8_t>> LoadFromText(const std::string& text) const`
  - Converts a textplan string to binary protobuf format
  - Returns `std::nullopt` on error

- `std::optional<std::string> SaveToText(const std::vector<uint8_t>& data) const`
  - Converts a binary protobuf plan to textplan format
  - Returns `std::nullopt` on error

## Integrating into Your Project

### Using CMake

```cmake
# Add the substrait_textplan_cpp directory
add_subdirectory(path/to/substrait-textplan/cpp)

# Link against your target
target_link_libraries(your_target PRIVATE substrait_textplan_cpp)
```

### Manual Compilation

```bash
g++ -std=c++17 your_program.cpp \
    -I/path/to/substrait-textplan/cpp/include \
    -L/path/to/substrait-textplan/target/release \
    -lsubstrait_textplan \
    -o your_program
```

Make sure the shared library is in your library path when running:

```bash
# Linux
export LD_LIBRARY_PATH=/path/to/substrait-textplan/target/release:$LD_LIBRARY_PATH

# macOS
export DYLD_LIBRARY_PATH=/path/to/substrait-textplan/target/release:$DYLD_LIBRARY_PATH
```

## Error Handling

All methods return `std::optional` values. Check if the result has a value before using it:

```cpp
auto result = tp.LoadFromText(text);
if (result.has_value()) {
  // Success - use *result
  auto& binary_plan = *result;
} else {
  // Error occurred
  std::cerr << "Failed to parse textplan" << std::endl;
}
```

## Thread Safety

The `TextPlan` class is thread-safe for read operations. Multiple threads can safely call `LoadFromText` and `SaveToText` concurrently.

## Memory Management

The wrapper uses RAII (Resource Acquisition Is Initialization) for automatic memory management. All memory allocated by the Rust library is automatically freed when the returned `std::vector` or `std::string` goes out of scope. No manual memory management is required.

## License

Apache-2.0
