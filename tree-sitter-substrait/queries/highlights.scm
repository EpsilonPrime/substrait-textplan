; SPDX-License-Identifier: Apache-2.0

; Keywords
[
  "schema"
  "source"
  "RELATION"
  "ROOT"
  "PIPELINES"
  "EXTENSION_SPACE"
  "FUNCTION"
  "AS"
  "NAMED"
  "SCHEMA"
  "SOURCE"
  "COMMON"
  "BASE_SCHEMA"
  "FILTER"
  "EXPRESSION"
  "ADVANCED_EXTENSION"
  "GROUPING"
  "MEASURE"
  "SORT"
  "COUNT"
  "TYPE"
  "EMIT"
  "LOCAL_FILES"
  "VIRTUAL_TABLE"
  "NAMED_TABLE"
  "EXTENSION_TABLE"
  "IN"
  "SUBQUERY"
  "UNIQUE"
  "EXISTS"
  "ALL"
  "ANY"
  "LIST"
  "MAP"
  "STRUCT"
] @keyword

; Comparison operators
[
  "EQ"
  "NE"
  "LT"
  "GT"
  "LE"
  "GE"
] @operator

; Constants
[
  "NULL"
  "TRUE"
  "FALSE"
] @constant.builtin

; Operators and delimiters
[
  "{"
  "}"
  "("
  ")"
  "["
  "]"
  "<"
  ">"
  ";"
  ","
  ":"
  "."
  "->"
  "_"
  "="
  "-"
] @punctuation.delimiter

; Comments
(comment) @comment

; Types
(simple_type (identifier) @type)
(list_type) @type
(map_type) @type
(struct_type) @type

; Variables
(identifier) @variable

; Functions
(function_call (identifier) @function)
(function (name (identifier) @function))

; Literals
(string) @string
(number) @number