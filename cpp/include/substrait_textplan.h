// SPDX-License-Identifier: Apache-2.0

#ifndef SUBSTRAIT_TEXTPLAN_H
#define SUBSTRAIT_TEXTPLAN_H

#include <cstdint>
#include <memory>
#include <optional>
#include <string>
#include <vector>

namespace substrait {
namespace textplan {

/**
 * @brief C++ wrapper for the Substrait TextPlan library
 *
 * This class provides a C++ interface to the Rust-based Substrait TextPlan
 * library, which allows parsing and converting between text and binary
 * Substrait plan formats.
 */
class TextPlan {
 public:
  /**
   * @brief Construct a new TextPlan object
   */
  TextPlan();

  /**
   * @brief Destroy the TextPlan object
   */
  ~TextPlan();

  // Delete copy constructor and assignment operator
  TextPlan(const TextPlan&) = delete;
  TextPlan& operator=(const TextPlan&) = delete;

  // Allow move semantics
  TextPlan(TextPlan&&) noexcept;
  TextPlan& operator=(TextPlan&&) noexcept;

  /**
   * @brief Load a textplan from a string and convert it to binary protobuf
   *
   * @param text The textplan string to parse
   * @return std::optional<std::vector<uint8_t>> The binary protobuf
   * representation of the plan, or std::nullopt if an error occurred
   */
  std::optional<std::vector<uint8_t>> LoadFromText(
      const std::string& text) const;

  /**
   * @brief Save a binary plan to textplan format
   *
   * @param data The binary plan to convert
   * @return std::optional<std::string> The textplan representation of the plan,
   * or std::nullopt if an error occurred
   */
  std::optional<std::string> SaveToText(const std::vector<uint8_t>& data) const;

  /**
   * @brief Static helper: Load a textplan from a string and convert it to
   * binary protobuf
   *
   * @param text The textplan string to parse
   * @return std::optional<std::vector<uint8_t>> The binary protobuf
   * representation of the plan, or std::nullopt if an error occurred
   */
  static std::optional<std::vector<uint8_t>> LoadFromText(const char* text);

  /**
   * @brief Static helper: Save a binary plan to textplan format
   *
   * @param data Pointer to the binary plan data
   * @param size Size of the binary plan data in bytes
   * @return std::optional<std::string> The textplan representation of the plan,
   * or std::nullopt if an error occurred
   */
  static std::optional<std::string> SaveToText(
      const uint8_t* data,
      size_t size);

 private:
  class Impl;
  std::unique_ptr<Impl> impl_;
};

} // namespace textplan
} // namespace substrait

#endif // SUBSTRAIT_TEXTPLAN_H
