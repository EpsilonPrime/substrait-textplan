// SPDX-License-Identifier: Apache-2.0

#include <iostream>
#include <string>

#include "substrait_textplan.h"

int main() {
  std::string text = R"(
pipelines {
  simple_read -> root;
}

schema simple_schema {
  id i32;
  name string;
  price fp64;
}

source named_table simple_source {
  names = ["test_table"]
}

read relation simple_read {
  base_schema simple_schema;
  source simple_source;
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
