# Substrait TextPlan Python Bindings

Python bindings for the [Substrait TextPlan](https://github.com/EpsilonPrime/substrait-textplan) library.

## Overview

Substrait TextPlan is a parser and converter for Substrait plans in textplan format. This Python package provides bindings to the Rust implementation, allowing you to:

- Parse Substrait textplan format and convert to binary protobuf
- Convert binary Substrait plans back to textplan format
- Validate and manipulate Substrait plans programmatically

## Installation

### From PyPI (when released)

```bash
pip install substrait-textplan
```

### From Source

```bash
# Clone the repository
git clone https://github.com/EpsilonPrime/substrait-textplan.git
cd substrait-textplan

# Build and install the Python package
cd python
pip install .
```

### Development Installation

For development, you can install in editable mode:

```bash
# Install in editable mode
pip install -e .

# Or with development dependencies
pip install -e ".[dev]"
```

## Requirements

- Python 3.8 or later
- Rust toolchain (for building from source)

## Usage

### Basic Usage

```python
import substrait_textplan

# Parse textplan format to binary protobuf
text = """
schema simple_schema {
    id i32;
    name string;
}
"""

binary_data = substrait_textplan.load_from_text(text)

# Convert binary back to textplan format
text_result = substrait_textplan.load_from_binary(binary_data)
print(text_result)
```

### Class-Based Interface

```python
from substrait_textplan import TextPlan

tp = TextPlan()

# Parse text to binary
text = """
schema my_schema {
    user_id i64;
    email string?;
}
"""

binary_data = tp.load_from_text(text)

# Convert binary to text
text_result = tp.load_from_binary(binary_data)
```

### Handling Errors

The functions return `None` if parsing fails:

```python
import substrait_textplan

result = substrait_textplan.load_from_text("invalid syntax")
if result is None:
    print("Failed to parse textplan")
```

## API Reference

### Functions

#### `load_from_text(text: str) -> Optional[bytes]`

Parse a textplan string and convert it to binary protobuf format.

**Parameters:**
- `text` (str): The textplan text to parse

**Returns:**
- `bytes`: Binary protobuf representation, or `None` if parsing failed

#### `load_from_binary(data: bytes) -> Optional[str]`

Convert a binary Substrait plan to textplan format.

**Parameters:**
- `data` (bytes): Binary protobuf data

**Returns:**
- `str`: Textplan representation, or `None` if conversion failed

### Classes

#### `TextPlan`

Main wrapper class for the library.

**Methods:**
- `load_from_text(text: str) -> Optional[bytes]`: Same as function version
- `load_from_binary(data: bytes) -> Optional[str]`: Same as function version

## Development

### Running Tests

```bash
# Run tests
python -m pytest tests/

# Or using unittest
python -m unittest discover tests/
```

### Building Wheels

```bash
# Install build dependencies
pip install build

# Build wheel
python -m build

# The wheel will be in dist/
```

## Architecture

This package uses ctypes to call into a Rust library built as a C dynamic library. The Rust library is automatically compiled and bundled when building the wheel using `setuptools-rust`.

## License

Apache-2.0

## Contributing

Contributions are welcome! Please see the main repository for contribution guidelines.

## Links

- [Main Repository](https://github.com/EpsilonPrime/substrait-textplan)
- [Substrait](https://substrait.io/)
- [Issue Tracker](https://github.com/EpsilonPrime/substrait-textplan/issues)
