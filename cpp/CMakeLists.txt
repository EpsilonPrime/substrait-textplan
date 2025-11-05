# SPDX-License-Identifier: Apache-2.0

cmake_minimum_required(VERSION 3.16)
project(substrait_textplan_cpp VERSION 0.1.0 LANGUAGES CXX)

# Set C++ standard
set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_EXTENSIONS OFF)

# Options
option(BUILD_EXAMPLES "Build example programs" ON)
option(BUILD_TESTS "Build test programs" ON)

# Find the Rust library
# First, check if SUBSTRAIT_TEXTPLAN_LIB_DIR is set
if(DEFINED ENV{SUBSTRAIT_TEXTPLAN_LIB_DIR})
  set(RUST_LIB_DIR $ENV{SUBSTRAIT_TEXTPLAN_LIB_DIR})
else()
  # Default to looking in the cargo build directories
  set(RUST_LIB_DIR "${CMAKE_CURRENT_SOURCE_DIR}/../target/release")
  if(NOT EXISTS "${RUST_LIB_DIR}")
    set(RUST_LIB_DIR "${CMAKE_CURRENT_SOURCE_DIR}/../target/debug")
  endif()
endif()

message(STATUS "Looking for Rust library in: ${RUST_LIB_DIR}")

# Find the shared library
find_library(SUBSTRAIT_TEXTPLAN_LIBRARY
  NAMES substrait_textplan libsubstrait_textplan
  PATHS ${RUST_LIB_DIR}
  NO_DEFAULT_PATH
  REQUIRED
)

message(STATUS "Found Substrait TextPlan library: ${SUBSTRAIT_TEXTPLAN_LIBRARY}")

# Create the C++ wrapper library
add_library(substrait_textplan_cpp
  src/substrait_textplan.cpp
)

target_include_directories(substrait_textplan_cpp
  PUBLIC
    $<BUILD_INTERFACE:${CMAKE_CURRENT_SOURCE_DIR}/include>
    $<INSTALL_INTERFACE:include>
)

target_link_libraries(substrait_textplan_cpp
  PUBLIC
    ${SUBSTRAIT_TEXTPLAN_LIBRARY}
)

# Set the rpath so the library can find the Rust shared library
if(APPLE)
  set_target_properties(substrait_textplan_cpp PROPERTIES
    INSTALL_RPATH "@loader_path;@loader_path/../lib;${RUST_LIB_DIR}"
    BUILD_RPATH "${RUST_LIB_DIR}"
  )
elseif(UNIX)
  set_target_properties(substrait_textplan_cpp PROPERTIES
    INSTALL_RPATH "$ORIGIN:$ORIGIN/../lib:${RUST_LIB_DIR}"
    BUILD_RPATH "${RUST_LIB_DIR}"
  )
endif()

# Build examples if requested
if(BUILD_EXAMPLES)
  add_subdirectory(examples)
endif()

# Build tests if requested
if(BUILD_TESTS)
  enable_testing()
  add_subdirectory(tests)
endif()

# Installation
include(GNUInstallDirs)

install(TARGETS substrait_textplan_cpp
  EXPORT substrait_textplan_cpp_targets
  LIBRARY DESTINATION ${CMAKE_INSTALL_LIBDIR}
  ARCHIVE DESTINATION ${CMAKE_INSTALL_LIBDIR}
  RUNTIME DESTINATION ${CMAKE_INSTALL_BINDIR}
)

install(DIRECTORY include/
  DESTINATION ${CMAKE_INSTALL_INCLUDEDIR}
)

install(EXPORT substrait_textplan_cpp_targets
  FILE substrait_textplan_cpp_targets.cmake
  NAMESPACE substrait::
  DESTINATION ${CMAKE_INSTALL_LIBDIR}/cmake/substrait_textplan_cpp
)
