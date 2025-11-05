// SPDX-License-Identifier: Apache-2.0

#include <iostream>
#include <string>

#include "substrait_textplan.h"

int main() {
  std::string original_text = R"(
    schema my_schema {
      id i32;
      name string;
      value fp64;
    }

    source LOCAL_FILES my_source {
      ITEMS = [
        {
          URI_FILE: "input.csv"
        }
      ]
    }

    read RELATION my_read {
      SOURCE my_source;
      BASE_SCHEMA my_schema;
    }

    ROOT {
      NAMES = [my_read]
    }
  )";

  std::cout << "Original textplan:" << std::endl;
  std::cout << original_text << std::endl;
  std::cout << std::string(60, '-') << std::endl;

  // Step 1: Convert text to binary
  std::cout << "\nStep 1: Converting textplan to binary..." << std::endl;
  auto binary_plan =
      substrait::textplan::TextPlan::LoadFromText(original_text.c_str());

  if (!binary_plan.has_value()) {
    std::cerr << "Error: Failed to convert textplan to binary" << std::endl;
    return 1;
  }

  std::cout << "Success! Binary plan size: " << binary_plan->size() << " bytes"
            << std::endl;

  // Step 2: Convert binary back to text
  std::cout << "\nStep 2: Converting binary back to textplan..." << std::endl;
  auto regenerated_text = substrait::textplan::TextPlan::SaveToText(
      binary_plan->data(), binary_plan->size());

  if (!regenerated_text.has_value()) {
    std::cerr << "Error: Failed to convert binary to textplan" << std::endl;
    return 1;
  }

  std::cout << "Success!" << std::endl;
  std::cout << "\nRegenerated textplan:" << std::endl;
  std::cout << *regenerated_text << std::endl;
  std::cout << std::string(60, '-') << std::endl;

  // Step 3: Verify roundtrip by converting back to binary again
  std::cout << "\nStep 3: Verifying roundtrip by converting to binary again..."
            << std::endl;
  auto binary_plan2 =
      substrait::textplan::TextPlan::LoadFromText(regenerated_text->c_str());

  if (!binary_plan2.has_value()) {
    std::cerr << "Error: Failed to parse regenerated textplan" << std::endl;
    return 1;
  }

  std::cout << "Success! Second binary plan size: " << binary_plan2->size()
            << " bytes" << std::endl;

  // Compare binary sizes (they should be the same)
  if (binary_plan->size() == binary_plan2->size()) {
    std::cout << "\nRoundtrip successful! Both binary plans have the same size."
              << std::endl;
  } else {
    std::cout << "\nWarning: Binary plan sizes differ after roundtrip."
              << std::endl;
  }

  return 0;
}
