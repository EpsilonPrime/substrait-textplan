# SPDX-License-Identifier: Apache-2.0

# Simple example demonstrating basic usage
add_executable(simple_example simple_example.cpp)
target_link_libraries(simple_example PRIVATE substrait_textplan_cpp)

# Set rpath for the example
if(APPLE)
  set_target_properties(simple_example PROPERTIES
    INSTALL_RPATH "@loader_path;@loader_path/../lib;${RUST_LIB_DIR}"
    BUILD_RPATH "${RUST_LIB_DIR}"
  )
elseif(UNIX)
  set_target_properties(simple_example PROPERTIES
    INSTALL_RPATH "$ORIGIN:$ORIGIN/../lib:${RUST_LIB_DIR}"
    BUILD_RPATH "${RUST_LIB_DIR}"
  )
endif()

# Roundtrip example demonstrating text->binary->text conversion
add_executable(roundtrip_example roundtrip_example.cpp)
target_link_libraries(roundtrip_example PRIVATE substrait_textplan_cpp)

# Set rpath for the example
if(APPLE)
  set_target_properties(roundtrip_example PROPERTIES
    INSTALL_RPATH "@loader_path;@loader_path/../lib;${RUST_LIB_DIR}"
    BUILD_RPATH "${RUST_LIB_DIR}"
  )
elseif(UNIX)
  set_target_properties(roundtrip_example PROPERTIES
    INSTALL_RPATH "$ORIGIN:$ORIGIN/../lib:${RUST_LIB_DIR}"
    BUILD_RPATH "${RUST_LIB_DIR}"
  )
endif()
