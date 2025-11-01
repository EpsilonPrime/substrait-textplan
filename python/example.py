#!/usr/bin/env python3
# SPDX-License-Identifier: Apache-2.0

"""
Example usage of the substrait-textplan Python package.
"""

import substrait_textplan

# Define a simple schema in textplan format
textplan = """
schema employee_schema {
    employee_id i64;
    first_name string;
    last_name string;
    email string?;
    salary decimal<10,2>;
    hire_date date;
}

source named_table employees_source {
    names = ["employees"]
}

read relation employees_scan {
    base_schema employee_schema;
    source employees_source;
}
"""

print("Original textplan:")
print(textplan)
print("\n" + "="*60 + "\n")

# Convert textplan to binary protobuf
binary_data = substrait_textplan.load_from_text(textplan)

if binary_data is None:
    print("ERROR: Failed to parse textplan")
    exit(1)

print(f"Converted to binary protobuf ({len(binary_data)} bytes)")
print("\n" + "="*60 + "\n")

# Convert binary back to textplan
result_text = substrait_textplan.load_from_binary(binary_data)

if result_text is None:
    print("ERROR: Failed to convert binary to textplan")
    exit(1)

print("Converted back to textplan:")
print(result_text)

# Verify roundtrip
print("\n" + "="*60 + "\n")
print("Roundtrip successful!")
