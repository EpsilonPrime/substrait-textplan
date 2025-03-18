// SPDX-License-Identifier: Apache-2.0

/**
 * @file Substrait TextPlan grammar for tree-sitter
 * 
 * This grammar is based on the ANTLR grammar in the Substrait project.
 */

module.exports = grammar({
  name: 'substrait',

  extras: $ => [
    /\s|\r?\n/,
    $.comment,
  ],
  
  conflicts: $ => [
    [$.relation_detail, $.identifier],
    [$.expression, $.function_call],
    [$.expression, $.constant],
    [$.expression, $.column_reference],
    [$.pipeline, $.relation_ref],
    [$.constant, $.expression],
    [$.sort_field, $.identifier],
    [$.measure_detail, $.identifier],
    [$.expression, $.identifier],
    [$.source_reference, $.identifier],
    [$.relation_filter_behavior],
    [$.relation_filter_behavior, $.identifier],
    [$.constant, $.identifier],
    [$.type, $.map_type],
    [$.type, $.list_type],
    [$.type, $.struct_type],
    [$.map_literal, $.struct_literal],
    [$.function_call],
  ],

  rules: {
    // The entry point of the grammar
    plan: $ => seq(
      repeat($.plan_detail),
      optional(';')
    ),

    plan_detail: $ => choice(
      $.pipelines,
      $.relation,
      $.root_relation,
      $.schema_definition,
      $.source_definition,
      $.extension_space
    ),

    // Schema definition
    schema_definition: $ => seq(
      'schema',
      $.identifier,
      '{',
      repeat($.schema_item),
      '}'
    ),

    schema_item: $ => seq(
      $.identifier,
      $.type,
      optional(seq('NAMED', $.identifier)),
      ';'
    ),

    // Source definition
    source_definition: $ => seq(
      'source',
      $.read_type
    ),

    read_type: $ => choice(
      $.local_files,
      $.virtual_table,
      $.named_table,
      $.extension_table
    ),

    local_files: $ => seq(
      'LOCAL_FILES',
      $.identifier,
      '{',
      repeat($.local_files_detail),
      '}'
    ),

    local_files_detail: $ => choice(
      seq('ADVANCED_EXTENSION', $.identifier),
      seq('ITEMS', '=', '[', repeat(seq($.file, optional(','))), ']')
    ),

    file: $ => seq(
      '{',
      repeat($.file_detail),
      '}'
    ),

    file_detail: $ => choice(
      seq('PARTITION_INDEX', ':', $.number),
      seq('START', ':', $.number),
      seq('LENGTH', ':', $.number),
      seq('ORC', ':', '{', '}'),
      seq('PARQUET', ':', '{', '}'),
      $.file_location
    ),

    file_location: $ => choice(
      seq('URI_FILE', ':', $.string),
      seq('URI_PATH', ':', $.string),
      seq('URI_PATH_GLOB', ':', $.string),
      seq('URI_FOLDER', ':', $.string)
    ),

    virtual_table: $ => seq(
      'VIRTUAL_TABLE',
      $.identifier,
      '{',
      '}'
    ),

    named_table: $ => seq(
      'NAMED_TABLE',
      $.identifier,
      '{',
      repeat($.named_table_detail),
      '}'
    ),

    named_table_detail: $ => choice(
      seq('ADVANCED_EXTENSION', $.identifier),
      seq('NAMES', '=', '[', repeat(seq($.string, optional(','))), ']')
    ),

    extension_table: $ => seq(
      'EXTENSION_TABLE',
      $.identifier,
      '{',
      '}'
    ),

    // Relation definition
    relation: $ => seq(
      $.relation_type,
      'RELATION',
      $.relation_ref,
      '{',
      repeat($.relation_detail),
      '}'
    ),

    relation_type: $ => $.identifier,

    relation_ref: $ => seq(
      $.identifier,
      optional(seq('(', 'SCHEMA', $.identifier, ')'))
    ),

    relation_detail: $ => choice(
      seq('COMMON', ';'),
      seq('BASE_SCHEMA', $.identifier, ';'),
      seq(optional($.relation_filter_behavior), 'FILTER', $.expression, ';') ,
      seq('EXPRESSION', $.expression, optional(seq('NAMED', $.identifier)), ';'),
      seq('ADVANCED_EXTENSION', ';'),
      seq($.source_reference, ';'),
      seq('GROUPING', $.expression, ';'),
      seq('MEASURE', '{', repeat($.measure_detail), '}'),
      $.sort_field,
      seq('COUNT', $.number, ';'),
      seq('TYPE', $.identifier, ';'),
      seq('EMIT', $.column_name, ';')
    ),
    

    relation_filter_behavior: $ => choice(
      $.identifier,
      seq($.identifier, '-', $.identifier),
      seq($.identifier, $.identifier)
    ),

    measure_detail: $ => choice(
      seq('MEASURE', $.expression, optional(seq('->', $.type)), optional(seq('@', $.identifier)), optional(seq('NAMED', $.identifier)), ';'),
      seq('FILTER', $.expression, ';'),
      seq('INVOCATION', $.identifier, ';'),
      $.sort_field
    ),

    sort_field: $ => seq(
      'SORT',
      $.expression,
      optional(seq('BY', $.identifier)),
      ';'
    ),

    // Root relation
    root_relation: $ => seq(
      'ROOT',
      '{',
      'NAMES',
      '=',
      '[',
      $.identifier,
      repeat(seq(',', $.identifier)),
      optional(','),
      ']',
      '}'
    ),

    // Pipelines
    pipelines: $ => seq(
      'PIPELINES',
      '{',
      repeat(seq($.pipeline, ';')),
      '}'
    ),

    pipeline: $ => choice(
      seq($.pipeline, '->', $.relation_ref),
      $.relation_ref
    ),

    // Extension space
    extension_space: $ => seq(
      'EXTENSION_SPACE',
      optional($.uri),
      '{',
      repeat($.function),
      '}'
    ),

    function: $ => seq(
      'FUNCTION',
      $.name,
      optional(seq('AS', $.identifier)),
      ';'
    ),

    name: $ => seq(
      $.identifier,
      optional(seq(':', optional($.signature)))
    ),

    signature: $ => $.identifier,

    // Expressions
    expression: $ => choice(
      $.function_call,
      $.constant,
      $.column_reference,
      $.cast_expression,
      $.subquery_expression,
      $.in_predicate_subquery,
      $.set_predicate_subquery,
      $.set_comparison_subquery
    ),

    function_call: $ => seq(
      $.identifier,
      '(',
      optional(seq($.expression, repeat(seq(',', $.expression)), optional(','))),
      ')',
      optional(seq('->', $.type))
    ),

    constant: $ => choice(
      seq($.number, optional(seq('_', $.type))),
      seq($.string, optional(seq('_', $.type))),
      seq($.map_literal, optional(seq('_', $.type))),
      seq($.struct_literal, optional(seq('_', $.type))),
      seq('NULL', optional(seq('_', $.type))),
      seq('TRUE', optional(seq('_', $.type))),
      seq('FALSE', optional(seq('_', $.type)))
    ),

    map_literal: $ => choice(
      seq('{', '}'),
      seq('{', $.map_entry, repeat(seq(',', $.map_entry)), '}')
    ),

    map_entry: $ => seq(
      $.constant,
      ':',
      $.constant
    ),

    struct_literal: $ => choice(
      seq('{', '}'),
      seq('{', $.constant, repeat(seq(',', $.constant)), '}')
    ),

    column_reference: $ => $.column_name,

    column_name: $ => seq(
      optional(seq($.identifier, '.')),
      $.identifier
    ),

    cast_expression: $ => seq(
      $.expression,
      'AS',
      $.type
    ),

    subquery_expression: $ => seq(
      'SUBQUERY',
      $.relation_ref
    ),

    in_predicate_subquery: $ => seq(
      $.expression_list,
      'IN',
      'SUBQUERY',
      $.relation_ref
    ),

    expression_list: $ => seq(
      '(',
      $.expression,
      repeat(seq(',', $.expression)),
      ')'
    ),

    set_predicate_subquery: $ => seq(
      choice('UNIQUE', 'EXISTS'),
      'IN',
      'SUBQUERY',
      $.relation_ref
    ),

    set_comparison_subquery: $ => seq(
      $.expression,
      $.comparison_operator,
      choice('ALL', 'ANY'),
      'SUBQUERY',
      $.relation_ref
    ),

    comparison_operator: $ => choice('EQ', 'NE', 'LT', 'GT', 'LE', 'GE'),

    // Source reference
    source_reference: $ => seq(
      'SOURCE',
      $.identifier
    ),

    // Types
    type: $ => choice(
      $.simple_type,
      $.list_type,
      $.map_type,
      $.struct_type
    ),

    simple_type: $ => seq(
      $.identifier,
      optional('?'),
      optional($.type_specifier)
    ),

    list_type: $ => seq(
      'LIST',
      optional('?'),
      '<',
      optional($.type),
      '>'
    ),

    map_type: $ => seq(
      'MAP',
      optional('?'),
      '<',
      optional($.simple_type),
      optional(','),
      optional($.type),
      '>'
    ),

    struct_type: $ => seq(
      'STRUCT',
      optional('?'),
      '<',
      optional(seq(
        $.type,
        repeat(seq(',', $.type))
      )),
      '>'
    ),

    type_specifier: $ => seq(
      '<',
      $.number,
      repeat(seq(',', $.number)),
      '>'
    ),

    // Basic tokens
    identifier: $ => choice(
      /[A-Za-z][A-Za-z0-9$]*/,
      'FILTER',
      'ROOT',
      'SOURCE',
      'SCHEMA',
      'NULL',
      'SORT',
      'MEASURE',
      'GROUPING',
      'COUNT',
      'TYPE',
      'EMIT',
      'NAMED',
      'ALL',
      'ANY'
    ),

    uri: $ => choice(
      seq(/[A-Za-z]+/, ':', optional(seq('//', /[A-Za-z0-9-.]+/, '/')), /[A-Za-z0-9-._]+(\/[A-Za-z0-9-._]+)*/),
      seq(optional('/'), /[A-Za-z0-9-._]+(\/[A-Za-z0-9-._]+)*/)
    ),

    number: $ => /[-+]?[0-9]+(\.[0-9]+)?([eE][-+]?[0-9]+)?/,

    string: $ => choice(
      seq('"', repeat(choice(/[^"\\]/, /\\./)), '"'),
      seq('`', repeat(/[^`]/), '`'),
      seq('``', repeat(/./), '``'),
      seq('```', repeat(/./), '```')
    ),

    comment: $ => seq('//', /[^\r\n]*/, choice('\r\n', '\n', '\r'))
  }
});