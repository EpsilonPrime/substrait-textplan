// SPDX-License-Identifier: Apache-2.0

#include <iostream>
#include <string>

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

    filter RELATION filtered_data {
      INPUT simple_read;
      CONDITION greater_than(price, 100.0_fp64);
    }

    ROOT {
      NAMES = [filtered_data]
    }
  )";

  std::cout << "Parsing textplan..." << std::endl;

  // Using static method
  auto binary_plan = substrait::textplan::TextPlan::LoadFromText(text.c_str());

  if (!binary_plan.has_value()) {
    std::cerr << "Error: Failed to parse textplan" << std::endl;
    return 1;
  }

  std::cout << "Successfully parsed textplan!" << std::endl;
  std::cout << "Binary plan size: " << binary_plan->size() << " bytes"
            << std::endl;

  // Can also use instance method
  substrait::textplan::TextPlan tp;
  auto binary_plan2 = tp.LoadFromText(text);

  if (!binary_plan2.has_value()) {
    std::cerr << "Error: Failed to parse textplan (instance method)"
              << std::endl;
    return 1;
  }

  std::cout << "Successfully parsed textplan using instance method!"
            << std::endl;
  std::cout << "Binary plan size: " << binary_plan2->size() << " bytes"
            << std::endl;

  return 0;
}
