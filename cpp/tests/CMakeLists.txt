# SPDX-License-Identifier: Apache-2.0

# Create test executable
add_executable(test_textplan test_textplan.cpp)
target_link_libraries(test_textplan PRIVATE substrait_textplan_cpp)

# Set rpath for the test
if(APPLE)
  set_target_properties(test_textplan PROPERTIES
    INSTALL_RPATH "@loader_path;@loader_path/../lib;${RUST_LIB_DIR}"
    BUILD_RPATH "${RUST_LIB_DIR}"
  )
elseif(UNIX)
  set_target_properties(test_textplan PROPERTIES
    INSTALL_RPATH "$ORIGIN:$ORIGIN/../lib:${RUST_LIB_DIR}"
    BUILD_RPATH "${RUST_LIB_DIR}"
  )
endif()

# Add the test to CTest
add_test(NAME TextPlanTests COMMAND test_textplan)
