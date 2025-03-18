#![allow(nonstandard_style)]
// Generated from /var/folders/8v/grx9b3v9755c2rmx1dqnw7nr0000gn/T/substrait_antlr/SubstraitPlanParser.g4 by ANTLR 4.8
use antlr_rust::tree::ParseTreeListener;
use super::substraitplanparser::*;

pub trait SubstraitPlanParserListener<'input> : ParseTreeListener<'input,SubstraitPlanParserContextType>{
/**
 * Enter a parse tree produced by {@link SubstraitPlanParser#plan}.
 * @param ctx the parse tree
 */
fn enter_plan(&mut self, _ctx: &PlanContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SubstraitPlanParser#plan}.
 * @param ctx the parse tree
 */
fn exit_plan(&mut self, _ctx: &PlanContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SubstraitPlanParser#plan_detail}.
 * @param ctx the parse tree
 */
fn enter_plan_detail(&mut self, _ctx: &Plan_detailContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SubstraitPlanParser#plan_detail}.
 * @param ctx the parse tree
 */
fn exit_plan_detail(&mut self, _ctx: &Plan_detailContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SubstraitPlanParser#pipelines}.
 * @param ctx the parse tree
 */
fn enter_pipelines(&mut self, _ctx: &PipelinesContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SubstraitPlanParser#pipelines}.
 * @param ctx the parse tree
 */
fn exit_pipelines(&mut self, _ctx: &PipelinesContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SubstraitPlanParser#pipeline}.
 * @param ctx the parse tree
 */
fn enter_pipeline(&mut self, _ctx: &PipelineContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SubstraitPlanParser#pipeline}.
 * @param ctx the parse tree
 */
fn exit_pipeline(&mut self, _ctx: &PipelineContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SubstraitPlanParser#relation}.
 * @param ctx the parse tree
 */
fn enter_relation(&mut self, _ctx: &RelationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SubstraitPlanParser#relation}.
 * @param ctx the parse tree
 */
fn exit_relation(&mut self, _ctx: &RelationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SubstraitPlanParser#root_relation}.
 * @param ctx the parse tree
 */
fn enter_root_relation(&mut self, _ctx: &Root_relationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SubstraitPlanParser#root_relation}.
 * @param ctx the parse tree
 */
fn exit_root_relation(&mut self, _ctx: &Root_relationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SubstraitPlanParser#relation_type}.
 * @param ctx the parse tree
 */
fn enter_relation_type(&mut self, _ctx: &Relation_typeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SubstraitPlanParser#relation_type}.
 * @param ctx the parse tree
 */
fn exit_relation_type(&mut self, _ctx: &Relation_typeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SubstraitPlanParser#relation_ref}.
 * @param ctx the parse tree
 */
fn enter_relation_ref(&mut self, _ctx: &Relation_refContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SubstraitPlanParser#relation_ref}.
 * @param ctx the parse tree
 */
fn exit_relation_ref(&mut self, _ctx: &Relation_refContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SubstraitPlanParser#relation_filter_behavior}.
 * @param ctx the parse tree
 */
fn enter_relation_filter_behavior(&mut self, _ctx: &Relation_filter_behaviorContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SubstraitPlanParser#relation_filter_behavior}.
 * @param ctx the parse tree
 */
fn exit_relation_filter_behavior(&mut self, _ctx: &Relation_filter_behaviorContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SubstraitPlanParser#measure_detail}.
 * @param ctx the parse tree
 */
fn enter_measure_detail(&mut self, _ctx: &Measure_detailContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SubstraitPlanParser#measure_detail}.
 * @param ctx the parse tree
 */
fn exit_measure_detail(&mut self, _ctx: &Measure_detailContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code relationCommon}
 * labeled alternative in {@link SubstraitPlanParser#relation_detail}.
 * @param ctx the parse tree
 */
fn enter_relationCommon(&mut self, _ctx: &RelationCommonContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code relationCommon}
 * labeled alternative in {@link SubstraitPlanParser#relation_detail}.
 * @param ctx the parse tree
 */
fn exit_relationCommon(&mut self, _ctx: &RelationCommonContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code relationUsesSchema}
 * labeled alternative in {@link SubstraitPlanParser#relation_detail}.
 * @param ctx the parse tree
 */
fn enter_relationUsesSchema(&mut self, _ctx: &RelationUsesSchemaContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code relationUsesSchema}
 * labeled alternative in {@link SubstraitPlanParser#relation_detail}.
 * @param ctx the parse tree
 */
fn exit_relationUsesSchema(&mut self, _ctx: &RelationUsesSchemaContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code relationFilter}
 * labeled alternative in {@link SubstraitPlanParser#relation_detail}.
 * @param ctx the parse tree
 */
fn enter_relationFilter(&mut self, _ctx: &RelationFilterContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code relationFilter}
 * labeled alternative in {@link SubstraitPlanParser#relation_detail}.
 * @param ctx the parse tree
 */
fn exit_relationFilter(&mut self, _ctx: &RelationFilterContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code relationExpression}
 * labeled alternative in {@link SubstraitPlanParser#relation_detail}.
 * @param ctx the parse tree
 */
fn enter_relationExpression(&mut self, _ctx: &RelationExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code relationExpression}
 * labeled alternative in {@link SubstraitPlanParser#relation_detail}.
 * @param ctx the parse tree
 */
fn exit_relationExpression(&mut self, _ctx: &RelationExpressionContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code relationAdvancedExtension}
 * labeled alternative in {@link SubstraitPlanParser#relation_detail}.
 * @param ctx the parse tree
 */
fn enter_relationAdvancedExtension(&mut self, _ctx: &RelationAdvancedExtensionContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code relationAdvancedExtension}
 * labeled alternative in {@link SubstraitPlanParser#relation_detail}.
 * @param ctx the parse tree
 */
fn exit_relationAdvancedExtension(&mut self, _ctx: &RelationAdvancedExtensionContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code relationSourceReference}
 * labeled alternative in {@link SubstraitPlanParser#relation_detail}.
 * @param ctx the parse tree
 */
fn enter_relationSourceReference(&mut self, _ctx: &RelationSourceReferenceContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code relationSourceReference}
 * labeled alternative in {@link SubstraitPlanParser#relation_detail}.
 * @param ctx the parse tree
 */
fn exit_relationSourceReference(&mut self, _ctx: &RelationSourceReferenceContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code relationGrouping}
 * labeled alternative in {@link SubstraitPlanParser#relation_detail}.
 * @param ctx the parse tree
 */
fn enter_relationGrouping(&mut self, _ctx: &RelationGroupingContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code relationGrouping}
 * labeled alternative in {@link SubstraitPlanParser#relation_detail}.
 * @param ctx the parse tree
 */
fn exit_relationGrouping(&mut self, _ctx: &RelationGroupingContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code relationMeasure}
 * labeled alternative in {@link SubstraitPlanParser#relation_detail}.
 * @param ctx the parse tree
 */
fn enter_relationMeasure(&mut self, _ctx: &RelationMeasureContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code relationMeasure}
 * labeled alternative in {@link SubstraitPlanParser#relation_detail}.
 * @param ctx the parse tree
 */
fn exit_relationMeasure(&mut self, _ctx: &RelationMeasureContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code relationSort}
 * labeled alternative in {@link SubstraitPlanParser#relation_detail}.
 * @param ctx the parse tree
 */
fn enter_relationSort(&mut self, _ctx: &RelationSortContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code relationSort}
 * labeled alternative in {@link SubstraitPlanParser#relation_detail}.
 * @param ctx the parse tree
 */
fn exit_relationSort(&mut self, _ctx: &RelationSortContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code relationCount}
 * labeled alternative in {@link SubstraitPlanParser#relation_detail}.
 * @param ctx the parse tree
 */
fn enter_relationCount(&mut self, _ctx: &RelationCountContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code relationCount}
 * labeled alternative in {@link SubstraitPlanParser#relation_detail}.
 * @param ctx the parse tree
 */
fn exit_relationCount(&mut self, _ctx: &RelationCountContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code relationJoinType}
 * labeled alternative in {@link SubstraitPlanParser#relation_detail}.
 * @param ctx the parse tree
 */
fn enter_relationJoinType(&mut self, _ctx: &RelationJoinTypeContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code relationJoinType}
 * labeled alternative in {@link SubstraitPlanParser#relation_detail}.
 * @param ctx the parse tree
 */
fn exit_relationJoinType(&mut self, _ctx: &RelationJoinTypeContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code relationEmit}
 * labeled alternative in {@link SubstraitPlanParser#relation_detail}.
 * @param ctx the parse tree
 */
fn enter_relationEmit(&mut self, _ctx: &RelationEmitContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code relationEmit}
 * labeled alternative in {@link SubstraitPlanParser#relation_detail}.
 * @param ctx the parse tree
 */
fn exit_relationEmit(&mut self, _ctx: &RelationEmitContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code expressionScalarSubquery}
 * labeled alternative in {@link SubstraitPlanParser#expression}.
 * @param ctx the parse tree
 */
fn enter_expressionScalarSubquery(&mut self, _ctx: &ExpressionScalarSubqueryContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code expressionScalarSubquery}
 * labeled alternative in {@link SubstraitPlanParser#expression}.
 * @param ctx the parse tree
 */
fn exit_expressionScalarSubquery(&mut self, _ctx: &ExpressionScalarSubqueryContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code expressionConstant}
 * labeled alternative in {@link SubstraitPlanParser#expression}.
 * @param ctx the parse tree
 */
fn enter_expressionConstant(&mut self, _ctx: &ExpressionConstantContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code expressionConstant}
 * labeled alternative in {@link SubstraitPlanParser#expression}.
 * @param ctx the parse tree
 */
fn exit_expressionConstant(&mut self, _ctx: &ExpressionConstantContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code expressionFunctionUse}
 * labeled alternative in {@link SubstraitPlanParser#expression}.
 * @param ctx the parse tree
 */
fn enter_expressionFunctionUse(&mut self, _ctx: &ExpressionFunctionUseContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code expressionFunctionUse}
 * labeled alternative in {@link SubstraitPlanParser#expression}.
 * @param ctx the parse tree
 */
fn exit_expressionFunctionUse(&mut self, _ctx: &ExpressionFunctionUseContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code expressionColumn}
 * labeled alternative in {@link SubstraitPlanParser#expression}.
 * @param ctx the parse tree
 */
fn enter_expressionColumn(&mut self, _ctx: &ExpressionColumnContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code expressionColumn}
 * labeled alternative in {@link SubstraitPlanParser#expression}.
 * @param ctx the parse tree
 */
fn exit_expressionColumn(&mut self, _ctx: &ExpressionColumnContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code expressionSetComparisonSubquery}
 * labeled alternative in {@link SubstraitPlanParser#expression}.
 * @param ctx the parse tree
 */
fn enter_expressionSetComparisonSubquery(&mut self, _ctx: &ExpressionSetComparisonSubqueryContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code expressionSetComparisonSubquery}
 * labeled alternative in {@link SubstraitPlanParser#expression}.
 * @param ctx the parse tree
 */
fn exit_expressionSetComparisonSubquery(&mut self, _ctx: &ExpressionSetComparisonSubqueryContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code expressionInPredicateSubquery}
 * labeled alternative in {@link SubstraitPlanParser#expression}.
 * @param ctx the parse tree
 */
fn enter_expressionInPredicateSubquery(&mut self, _ctx: &ExpressionInPredicateSubqueryContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code expressionInPredicateSubquery}
 * labeled alternative in {@link SubstraitPlanParser#expression}.
 * @param ctx the parse tree
 */
fn exit_expressionInPredicateSubquery(&mut self, _ctx: &ExpressionInPredicateSubqueryContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code expressionCast}
 * labeled alternative in {@link SubstraitPlanParser#expression}.
 * @param ctx the parse tree
 */
fn enter_expressionCast(&mut self, _ctx: &ExpressionCastContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code expressionCast}
 * labeled alternative in {@link SubstraitPlanParser#expression}.
 * @param ctx the parse tree
 */
fn exit_expressionCast(&mut self, _ctx: &ExpressionCastContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code expressionSetPredicateSubquery}
 * labeled alternative in {@link SubstraitPlanParser#expression}.
 * @param ctx the parse tree
 */
fn enter_expressionSetPredicateSubquery(&mut self, _ctx: &ExpressionSetPredicateSubqueryContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code expressionSetPredicateSubquery}
 * labeled alternative in {@link SubstraitPlanParser#expression}.
 * @param ctx the parse tree
 */
fn exit_expressionSetPredicateSubquery(&mut self, _ctx: &ExpressionSetPredicateSubqueryContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SubstraitPlanParser#expression_list}.
 * @param ctx the parse tree
 */
fn enter_expression_list(&mut self, _ctx: &Expression_listContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SubstraitPlanParser#expression_list}.
 * @param ctx the parse tree
 */
fn exit_expression_list(&mut self, _ctx: &Expression_listContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SubstraitPlanParser#constant}.
 * @param ctx the parse tree
 */
fn enter_constant(&mut self, _ctx: &ConstantContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SubstraitPlanParser#constant}.
 * @param ctx the parse tree
 */
fn exit_constant(&mut self, _ctx: &ConstantContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SubstraitPlanParser#literal_basic_type}.
 * @param ctx the parse tree
 */
fn enter_literal_basic_type(&mut self, _ctx: &Literal_basic_typeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SubstraitPlanParser#literal_basic_type}.
 * @param ctx the parse tree
 */
fn exit_literal_basic_type(&mut self, _ctx: &Literal_basic_typeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SubstraitPlanParser#literal_complex_type}.
 * @param ctx the parse tree
 */
fn enter_literal_complex_type(&mut self, _ctx: &Literal_complex_typeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SubstraitPlanParser#literal_complex_type}.
 * @param ctx the parse tree
 */
fn exit_literal_complex_type(&mut self, _ctx: &Literal_complex_typeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SubstraitPlanParser#literal_specifier}.
 * @param ctx the parse tree
 */
fn enter_literal_specifier(&mut self, _ctx: &Literal_specifierContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SubstraitPlanParser#literal_specifier}.
 * @param ctx the parse tree
 */
fn exit_literal_specifier(&mut self, _ctx: &Literal_specifierContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SubstraitPlanParser#map_literal}.
 * @param ctx the parse tree
 */
fn enter_map_literal(&mut self, _ctx: &Map_literalContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SubstraitPlanParser#map_literal}.
 * @param ctx the parse tree
 */
fn exit_map_literal(&mut self, _ctx: &Map_literalContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SubstraitPlanParser#map_literal_value}.
 * @param ctx the parse tree
 */
fn enter_map_literal_value(&mut self, _ctx: &Map_literal_valueContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SubstraitPlanParser#map_literal_value}.
 * @param ctx the parse tree
 */
fn exit_map_literal_value(&mut self, _ctx: &Map_literal_valueContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SubstraitPlanParser#struct_literal}.
 * @param ctx the parse tree
 */
fn enter_struct_literal(&mut self, _ctx: &Struct_literalContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SubstraitPlanParser#struct_literal}.
 * @param ctx the parse tree
 */
fn exit_struct_literal(&mut self, _ctx: &Struct_literalContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SubstraitPlanParser#column_name}.
 * @param ctx the parse tree
 */
fn enter_column_name(&mut self, _ctx: &Column_nameContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SubstraitPlanParser#column_name}.
 * @param ctx the parse tree
 */
fn exit_column_name(&mut self, _ctx: &Column_nameContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SubstraitPlanParser#source_reference}.
 * @param ctx the parse tree
 */
fn enter_source_reference(&mut self, _ctx: &Source_referenceContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SubstraitPlanParser#source_reference}.
 * @param ctx the parse tree
 */
fn exit_source_reference(&mut self, _ctx: &Source_referenceContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SubstraitPlanParser#file_location}.
 * @param ctx the parse tree
 */
fn enter_file_location(&mut self, _ctx: &File_locationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SubstraitPlanParser#file_location}.
 * @param ctx the parse tree
 */
fn exit_file_location(&mut self, _ctx: &File_locationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SubstraitPlanParser#file_detail}.
 * @param ctx the parse tree
 */
fn enter_file_detail(&mut self, _ctx: &File_detailContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SubstraitPlanParser#file_detail}.
 * @param ctx the parse tree
 */
fn exit_file_detail(&mut self, _ctx: &File_detailContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SubstraitPlanParser#file}.
 * @param ctx the parse tree
 */
fn enter_file(&mut self, _ctx: &FileContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SubstraitPlanParser#file}.
 * @param ctx the parse tree
 */
fn exit_file(&mut self, _ctx: &FileContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SubstraitPlanParser#local_files_detail}.
 * @param ctx the parse tree
 */
fn enter_local_files_detail(&mut self, _ctx: &Local_files_detailContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SubstraitPlanParser#local_files_detail}.
 * @param ctx the parse tree
 */
fn exit_local_files_detail(&mut self, _ctx: &Local_files_detailContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SubstraitPlanParser#named_table_detail}.
 * @param ctx the parse tree
 */
fn enter_named_table_detail(&mut self, _ctx: &Named_table_detailContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SubstraitPlanParser#named_table_detail}.
 * @param ctx the parse tree
 */
fn exit_named_table_detail(&mut self, _ctx: &Named_table_detailContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SubstraitPlanParser#schema_definition}.
 * @param ctx the parse tree
 */
fn enter_schema_definition(&mut self, _ctx: &Schema_definitionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SubstraitPlanParser#schema_definition}.
 * @param ctx the parse tree
 */
fn exit_schema_definition(&mut self, _ctx: &Schema_definitionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SubstraitPlanParser#schema_item}.
 * @param ctx the parse tree
 */
fn enter_schema_item(&mut self, _ctx: &Schema_itemContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SubstraitPlanParser#schema_item}.
 * @param ctx the parse tree
 */
fn exit_schema_item(&mut self, _ctx: &Schema_itemContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SubstraitPlanParser#source_definition}.
 * @param ctx the parse tree
 */
fn enter_source_definition(&mut self, _ctx: &Source_definitionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SubstraitPlanParser#source_definition}.
 * @param ctx the parse tree
 */
fn exit_source_definition(&mut self, _ctx: &Source_definitionContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code localFiles}
 * labeled alternative in {@link SubstraitPlanParser#read_type}.
 * @param ctx the parse tree
 */
fn enter_localFiles(&mut self, _ctx: &LocalFilesContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code localFiles}
 * labeled alternative in {@link SubstraitPlanParser#read_type}.
 * @param ctx the parse tree
 */
fn exit_localFiles(&mut self, _ctx: &LocalFilesContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code virtualTable}
 * labeled alternative in {@link SubstraitPlanParser#read_type}.
 * @param ctx the parse tree
 */
fn enter_virtualTable(&mut self, _ctx: &VirtualTableContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code virtualTable}
 * labeled alternative in {@link SubstraitPlanParser#read_type}.
 * @param ctx the parse tree
 */
fn exit_virtualTable(&mut self, _ctx: &VirtualTableContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code namedTable}
 * labeled alternative in {@link SubstraitPlanParser#read_type}.
 * @param ctx the parse tree
 */
fn enter_namedTable(&mut self, _ctx: &NamedTableContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code namedTable}
 * labeled alternative in {@link SubstraitPlanParser#read_type}.
 * @param ctx the parse tree
 */
fn exit_namedTable(&mut self, _ctx: &NamedTableContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code extensionTable}
 * labeled alternative in {@link SubstraitPlanParser#read_type}.
 * @param ctx the parse tree
 */
fn enter_extensionTable(&mut self, _ctx: &ExtensionTableContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code extensionTable}
 * labeled alternative in {@link SubstraitPlanParser#read_type}.
 * @param ctx the parse tree
 */
fn exit_extensionTable(&mut self, _ctx: &ExtensionTableContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SubstraitPlanParser#extensionspace}.
 * @param ctx the parse tree
 */
fn enter_extensionspace(&mut self, _ctx: &ExtensionspaceContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SubstraitPlanParser#extensionspace}.
 * @param ctx the parse tree
 */
fn exit_extensionspace(&mut self, _ctx: &ExtensionspaceContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SubstraitPlanParser#function}.
 * @param ctx the parse tree
 */
fn enter_function(&mut self, _ctx: &FunctionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SubstraitPlanParser#function}.
 * @param ctx the parse tree
 */
fn exit_function(&mut self, _ctx: &FunctionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SubstraitPlanParser#sort_field}.
 * @param ctx the parse tree
 */
fn enter_sort_field(&mut self, _ctx: &Sort_fieldContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SubstraitPlanParser#sort_field}.
 * @param ctx the parse tree
 */
fn exit_sort_field(&mut self, _ctx: &Sort_fieldContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SubstraitPlanParser#name}.
 * @param ctx the parse tree
 */
fn enter_name(&mut self, _ctx: &NameContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SubstraitPlanParser#name}.
 * @param ctx the parse tree
 */
fn exit_name(&mut self, _ctx: &NameContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SubstraitPlanParser#signature}.
 * @param ctx the parse tree
 */
fn enter_signature(&mut self, _ctx: &SignatureContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SubstraitPlanParser#signature}.
 * @param ctx the parse tree
 */
fn exit_signature(&mut self, _ctx: &SignatureContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SubstraitPlanParser#id}.
 * @param ctx the parse tree
 */
fn enter_id(&mut self, _ctx: &IdContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SubstraitPlanParser#id}.
 * @param ctx the parse tree
 */
fn exit_id(&mut self, _ctx: &IdContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SubstraitPlanParser#simple_id}.
 * @param ctx the parse tree
 */
fn enter_simple_id(&mut self, _ctx: &Simple_idContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SubstraitPlanParser#simple_id}.
 * @param ctx the parse tree
 */
fn exit_simple_id(&mut self, _ctx: &Simple_idContext<'input>) { }

}

antlr_rust::coerce_from!{ 'input : SubstraitPlanParserListener<'input> }


