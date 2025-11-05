// SPDX-License-Identifier: Apache-2.0

#include <cassert>
#include <iostream>
#include <string>

#include "substrait_textplan.h"

// Helper macro for test assertions
#define TEST_ASSERT(condition, message)                                        \
  do {                                                                         \
    if (!(condition)) {                                                        \
      std::cerr << "Test failed: " << message << std::endl;                    \
      std::cerr << "  at " << __FILE__ << ":" << __LINE__ << std::endl;        \
      return 1;                                                                \
    }                                                                          \
  } while (0)

// Test 1: Basic text to binary conversion
int test_load_from_text_basic() {
  std::cout << "Running: test_load_from_text_basic" << std::endl;

  std::string text = R"(
pipelines {
  simple_read -> root;
}

schema simple_schema {
  id i32;
  name string;
}

source named_table simple_source {
  names = ["test_table"]
}

read relation simple_read {
  base_schema simple_schema;
  source simple_source;
}
  )";

  auto result = substrait::textplan::TextPlan::LoadFromText(text.c_str());

  TEST_ASSERT(result.has_value(), "LoadFromText should return a value");
  TEST_ASSERT(!result->empty(), "Binary plan should not be empty");

  std::cout << "  PASSED (binary size: " << result->size() << " bytes)"
            << std::endl;
  return 0;
}

// Test 2: Instance method usage
int test_instance_method() {
  std::cout << "Running: test_instance_method" << std::endl;

  substrait::textplan::TextPlan tp;

  std::string text = R"(
pipelines {
  test_read -> root;
}

schema test_schema {
  value i32;
}

source named_table test_source {
  names = ["test_data"]
}

read relation test_read {
  base_schema test_schema;
  source test_source;
}
  )";

  auto result = tp.LoadFromText(text);

  TEST_ASSERT(
      result.has_value(), "Instance LoadFromText should return a value");
  TEST_ASSERT(!result->empty(), "Binary plan should not be empty");

  std::cout << "  PASSED (binary size: " << result->size() << " bytes)"
            << std::endl;
  return 0;
}

// Test 3: Roundtrip conversion (text -> binary -> text)
int test_roundtrip() {
  std::cout << "Running: test_roundtrip" << std::endl;

  std::string original_text = R"(
pipelines {
  roundtrip_read -> root;
}

schema roundtrip_schema {
  id i32;
  value fp64;
}

source named_table roundtrip_source {
  names = ["roundtrip_table"]
}

read relation roundtrip_read {
  base_schema roundtrip_schema;
  source roundtrip_source;
}
  )";

  // Text -> Binary
  auto binary =
      substrait::textplan::TextPlan::LoadFromText(original_text.c_str());
  TEST_ASSERT(binary.has_value(), "LoadFromText should succeed");
  TEST_ASSERT(!binary->empty(), "Binary plan should not be empty");

  size_t first_binary_size = binary->size();

  // Binary -> Text
  auto regenerated_text =
      substrait::textplan::TextPlan::SaveToText(binary->data(), binary->size());
  TEST_ASSERT(regenerated_text.has_value(), "SaveToText should succeed");
  TEST_ASSERT(
      !regenerated_text->empty(), "Regenerated text should not be empty");

  // Text -> Binary again
  auto binary2 =
      substrait::textplan::TextPlan::LoadFromText(regenerated_text->c_str());
  TEST_ASSERT(binary2.has_value(), "Second LoadFromText should succeed");
  TEST_ASSERT(!binary2->empty(), "Second binary plan should not be empty");

  // Both binary plans should have the same size
  TEST_ASSERT(
      first_binary_size == binary2->size(),
      "Roundtrip should preserve binary size");

  std::cout << "  PASSED (binary sizes: " << first_binary_size << " -> "
            << binary2->size() << " bytes)" << std::endl;
  return 0;
}

// Test 4: Error handling - null input
int test_null_input() {
  std::cout << "Running: test_null_input" << std::endl;

  auto result = substrait::textplan::TextPlan::LoadFromText(nullptr);

  TEST_ASSERT(
      !result.has_value(), "LoadFromText should return nullopt for null input");

  std::cout << "  PASSED" << std::endl;
  return 0;
}

// Test 5: Error handling - invalid textplan
int test_invalid_textplan() {
  std::cout << "Running: test_invalid_textplan" << std::endl;

  std::string invalid_text = "This is not a valid textplan at all!";

  auto result =
      substrait::textplan::TextPlan::LoadFromText(invalid_text.c_str());

  TEST_ASSERT(
      !result.has_value(),
      "LoadFromText should return nullopt for invalid input");

  std::cout << "  PASSED" << std::endl;
  return 0;
}

// Test 6: Empty input
int test_empty_input() {
  std::cout << "Running: test_empty_input" << std::endl;

  auto result = substrait::textplan::TextPlan::LoadFromText("");

  // Empty input might be valid and parse to an empty plan
  // This is acceptable behavior - we just verify it doesn't crash
  if (result.has_value()) {
    std::cout << "  PASSED (empty input parsed to " << result->size()
              << " byte plan)" << std::endl;
  } else {
    std::cout << "  PASSED (empty input returned nullopt)" << std::endl;
  }

  return 0;
}

// Test 7: SaveToText with empty data
int test_save_to_text_empty() {
  std::cout << "Running: test_save_to_text_empty" << std::endl;

  std::vector<uint8_t> empty_data;
  auto result = substrait::textplan::TextPlan::SaveToText(
      empty_data.data(), empty_data.size());

  TEST_ASSERT(
      !result.has_value(), "SaveToText should return nullopt for empty data");

  std::cout << "  PASSED" << std::endl;
  return 0;
}

// Test 8: Move semantics
int test_move_semantics() {
  std::cout << "Running: test_move_semantics" << std::endl;

  substrait::textplan::TextPlan tp1;

  // Move constructor
  substrait::textplan::TextPlan tp2(std::move(tp1));

  std::string text = R"(
pipelines {
  move_read -> root;
}

schema move_schema {
  id i32;
}

source named_table move_source {
  names = ["move_table"]
}

read relation move_read {
  base_schema move_schema;
  source move_source;
}
  )";

  auto result = tp2.LoadFromText(text);
  TEST_ASSERT(result.has_value(), "Moved object should work");

  // Move assignment
  substrait::textplan::TextPlan tp3;
  tp3 = std::move(tp2);

  auto result2 = tp3.LoadFromText(text);
  TEST_ASSERT(result2.has_value(), "Move-assigned object should work");

  std::cout << "  PASSED" << std::endl;
  return 0;
}

int main() {
  std::cout << "=== Running C++ TextPlan Tests ===" << std::endl;
  std::cout << std::endl;

  int failures = 0;

  failures += test_load_from_text_basic();
  failures += test_instance_method();
  failures += test_roundtrip();
  failures += test_null_input();
  failures += test_invalid_textplan();
  failures += test_empty_input();
  failures += test_save_to_text_empty();
  failures += test_move_semantics();

  std::cout << std::endl;
  if (failures == 0) {
    std::cout << "=== All tests passed! ===" << std::endl;
    return 0;
  } else {
    std::cout << "=== " << failures << " test(s) failed ===" << std::endl;
    return 1;
  }
}
