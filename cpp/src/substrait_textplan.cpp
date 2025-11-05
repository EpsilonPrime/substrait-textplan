// SPDX-License-Identifier: Apache-2.0

#include "substrait_textplan.h"

#include <cstdint>
#include <cstdlib>
#include <cstring>

namespace substrait {
namespace textplan {

// Declare external C functions from the Rust library
extern "C" {
void* load_from_text(const char* text);
void free_plan_bytes(void* ptr);
char* save_to_text(const uint8_t* bytes, size_t bytes_len);
void free_text_plan(char* text_ptr);
}

// Pimpl implementation
class TextPlan::Impl {
 public:
  Impl() = default;
  ~Impl() = default;

  // Delete copy constructor and assignment operator
  Impl(const Impl&) = delete;
  Impl& operator=(const Impl&) = delete;

  // Allow move semantics
  Impl(Impl&&) noexcept = default;
  Impl& operator=(Impl&&) noexcept = default;
};

TextPlan::TextPlan() : impl_(std::make_unique<Impl>()) {}

TextPlan::~TextPlan() = default;

TextPlan::TextPlan(TextPlan&&) noexcept = default;

TextPlan& TextPlan::operator=(TextPlan&&) noexcept = default;

std::optional<std::vector<uint8_t>> TextPlan::LoadFromText(
    const std::string& text) const {
  return LoadFromText(text.c_str());
}

std::optional<std::string> TextPlan::SaveToText(
    const std::vector<uint8_t>& data) const {
  return SaveToText(data.data(), data.size());
}

std::optional<std::vector<uint8_t>> TextPlan::LoadFromText(const char* text) {
  if (text == nullptr) {
    return std::nullopt;
  }

  void* ptr = load_from_text(text);
  if (ptr == nullptr) {
    return std::nullopt;
  }

  // First sizeof(size_t) bytes contain the length
  auto* len_ptr = static_cast<size_t*>(ptr);
  size_t length = *len_ptr;

  // Rest is the data
  auto* data_ptr =
      reinterpret_cast<uint8_t*>(reinterpret_cast<char*>(ptr) + sizeof(size_t));

  // Copy the data to a vector
  std::vector<uint8_t> result(data_ptr, data_ptr + length);

  // Free the C memory
  free_plan_bytes(ptr);

  return result;
}

std::optional<std::string> TextPlan::SaveToText(
    const uint8_t* data,
    size_t size) {
  if (data == nullptr || size == 0) {
    return std::nullopt;
  }

  char* text_ptr = save_to_text(data, size);
  if (text_ptr == nullptr) {
    return std::nullopt;
  }

  // Copy the string
  std::string result(text_ptr);

  // Free the C memory
  free_text_plan(text_ptr);

  return result;
}

} // namespace textplan
} // namespace substrait
