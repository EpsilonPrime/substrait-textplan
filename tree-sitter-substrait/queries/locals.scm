; SPDX-License-Identifier: Apache-2.0

; Scopes
(schema_definition) @scope
(relation) @scope
(pipelines) @scope
(extension_space) @scope

; References
(relation_ref (identifier) @reference)
(column_name (identifier) @reference)
(source_reference (identifier) @reference)
(relation_detail (identifier) @reference)

; Definitions
(schema_definition (identifier) @definition.struct)
(schema_item (identifier) @definition.field)
(relation (relation_ref (identifier) @definition.function))
(source_definition . "source" . (_) (identifier) @definition.constant)
(function (name (identifier) @definition.function))