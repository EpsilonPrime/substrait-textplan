// SPDX-License-Identifier: Apache-2.0

//! Roundtrip tests: JSON → Binary → Text → Binary → JSON
//!
//! These tests verify that we can convert a plan from JSON to binary,
//! then to text, parse it back, convert to binary again, and get the same result.
//! This follows the pattern from the C++ implementation in RoundtripTest.cpp.

#[cfg(test)]
mod tests {
    use crate::textplan::converter::load_json;
    use crate::textplan::converter::process_plan_with_visitor;
    use crate::textplan::converter::save_binary::save_to_binary;
    use crate::textplan::parser::parse_text::parse_stream;

    /// Add line numbers to text for better error reporting
    fn add_line_numbers(text: &str) -> String {
        text.lines()
            .enumerate()
            .map(|(i, line)| format!("{:4}: {}", i + 1, line))
            .collect::<Vec<_>>()
            .join("\n")
    }

    /// Helper function for roundtrip tests: JSON → Plan → TextPlan → SymbolTable → Binary → Plan comparison.
    /// Like the C++ implementation, we ignore the version field when comparing plans.
    fn run_roundtrip_test(test_file: &str) {
        let file_path = format!("src/substrait/textplan/data/{}", test_file);
        println!("\n=== Roundtrip test for: {} ===", test_file);

        // Step 1: Load JSON → Plan
        let original_plan = match load_json::load_from_json_file(&file_path) {
            Ok(plan) => plan,
            Err(err) => {
                panic!("Failed to load test file {}: {}", file_path, err);
            }
        };

        // Step 2: Plan → Binary (to verify encoding works)
        let original_binary = match crate::proto::save_plan_to_binary(&original_plan) {
            Ok(binary) => binary,
            Err(err) => {
                panic!(
                    "Failed to serialize original plan to binary for {}: {}",
                    file_path, err
                );
            }
        };

        println!("Original binary size: {} bytes", original_binary.len());

        // Step 3: Binary → Plan (verify we can deserialize what we just serialized)
        let loaded_plan = match crate::proto::load_plan_from_binary(&original_binary) {
            Ok(plan) => plan,
            Err(err) => {
                panic!(
                    "Failed to deserialize original binary for {}: {}",
                    file_path, err
                );
            }
        };

        // Step 4: Plan → TextPlan
        let text_plan = match process_plan_with_visitor(&loaded_plan) {
            Ok(text) => text,
            Err(err) => {
                panic!(
                    "Failed to convert binary to text for {}: {}",
                    file_path, err
                );
            }
        };

        assert!(!text_plan.is_empty(), "Empty textplan from binary");
        println!("Generated textplan ({} bytes)", text_plan.len());

        // Step 5: TextPlan → Parse → Symbol Table
        let parse_result = parse_stream(&text_plan);

        if !parse_result.successful() {
            println!("Generated textplan that failed to parse:");
            println!("{}", add_line_numbers(&text_plan));
            panic!(
                "Failed to parse generated textplan for {}: {:?}",
                file_path,
                parse_result.all_errors()
            );
        }

        let symbol_table = parse_result.symbol_table();
        println!(
            "Parsed successfully, symbol table has {} symbols",
            symbol_table.len()
        );

        // Step 6: Symbol Table → Binary
        let roundtrip_binary = match save_to_binary(&symbol_table) {
            Ok(binary) => binary,
            Err(err) => {
                panic!(
                    "Failed to convert symbol table to binary for {}: {}",
                    file_path, err
                );
            }
        };

        println!("Roundtrip binary size: {} bytes", roundtrip_binary.len());

        // Step 7: Binary → Plan (deserialize roundtrip result)
        let mut roundtrip_plan = match crate::proto::load_plan_from_binary(&roundtrip_binary) {
            Ok(plan) => plan,
            Err(err) => {
                panic!(
                    "Failed to deserialize roundtrip binary for {}: {}",
                    file_path, err
                );
            }
        };

        // Step 8: Normalize both plans for comparison (ignore version field like C++ does)
        let mut normalized_original = original_plan.clone();
        normalized_original.version = None;
        roundtrip_plan.version = None;

        // Step 9: Compare normalized plans
        if normalized_original != roundtrip_plan {
            // Plans differ - convert both to JSON for better error reporting
            let original_json = crate::proto::save_plan_to_json(&original_plan)
                .unwrap_or_else(|_| "Failed to serialize original plan".to_string());
            let roundtrip_json = crate::proto::save_plan_to_json(&roundtrip_plan)
                .unwrap_or_else(|_| "Failed to serialize roundtrip plan".to_string());

            println!("\n=== Original Plan JSON ===\n{}\n", original_json);
            println!("\n=== Roundtrip Plan JSON ===\n{}\n", roundtrip_json);

            panic!(
                "Roundtrip plan does not match original for {}\n\
                 Original: {} bytes binary, {} bytes JSON\n\
                 Roundtrip: {} bytes binary, {} bytes JSON\n\
                 Note: Comparison ignores version field",
                file_path,
                original_binary.len(),
                original_json.len(),
                roundtrip_binary.len(),
                roundtrip_json.len()
            );
        }

        println!("✓ Roundtrip successful: Plans match for {}", file_path);
    }

    // Macro to generate individual test functions for each data file
    macro_rules! roundtrip_tests {
        ($($name:ident: $file:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    run_roundtrip_test($file);
                }
            )*
        }
    }

    // Generate a test for each JSON file in the test data directory
    roundtrip_tests! {
        test_roundtrip_set_comparison_any: "set-comparision-any.json",
        test_roundtrip_tpch_plan01: "tpch-plan01.json",
        test_roundtrip_tpch_plan02: "tpch-plan02.json",
        test_roundtrip_tpch_plan03: "tpch-plan03.json",
        test_roundtrip_tpch_plan04: "tpch-plan04.json",
        test_roundtrip_tpch_plan05: "tpch-plan05.json",
        test_roundtrip_tpch_plan06: "tpch-plan06.json",
        test_roundtrip_tpch_plan07: "tpch-plan07.json",
        test_roundtrip_tpch_plan09: "tpch-plan09.json",
        test_roundtrip_tpch_plan10: "tpch-plan10.json",
        test_roundtrip_tpch_plan11: "tpch-plan11.json",
        test_roundtrip_tpch_plan13: "tpch-plan13.json",
        test_roundtrip_tpch_plan14: "tpch-plan14.json",
        test_roundtrip_tpch_plan16: "tpch-plan16.json",
        test_roundtrip_tpch_plan17: "tpch-plan17.json",
        test_roundtrip_tpch_plan18: "tpch-plan18.json",
        test_roundtrip_tpch_plan19: "tpch-plan19.json",
        test_roundtrip_tpch_plan20: "tpch-plan20.json",
        test_roundtrip_tpch_plan21: "tpch-plan21.json",
        test_roundtrip_tpch_plan22: "tpch-plan22.json",
    }
}
