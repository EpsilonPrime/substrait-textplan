#![allow(nonstandard_style)]
// Generated from /var/folders/0f/rd3736xj3hb75w3nt4wsjzyr0000gn/T/substrait_antlr/SubstraitPlanParser.g4 by ANTLR 4.8
use antlr_rust::tree::{ParseTreeVisitor,ParseTreeVisitorCompat};
use super::substraitplanparser::*;

/**
 * This interface defines a complete generic visitor for a parse tree produced
 * by {@link SubstraitPlanParser}.
 */
pub trait SubstraitPlanParserVisitor<'input>: ParseTreeVisitor<'input,SubstraitPlanParserContextType>{
	/**
	 * Visit a parse tree produced by {@link SubstraitPlanParser#plan}.
	 * @param ctx the parse tree
	 */
	fn visit_plan(&mut self, ctx: &PlanContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SubstraitPlanParser#plan_detail}.
	 * @param ctx the parse tree
	 */
	fn visit_plan_detail(&mut self, ctx: &Plan_detailContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SubstraitPlanParser#pipelines}.
	 * @param ctx the parse tree
	 */
	fn visit_pipelines(&mut self, ctx: &PipelinesContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SubstraitPlanParser#pipeline}.
	 * @param ctx the parse tree
	 */
	fn visit_pipeline(&mut self, ctx: &PipelineContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SubstraitPlanParser#relation}.
	 * @param ctx the parse tree
	 */
	fn visit_relation(&mut self, ctx: &RelationContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SubstraitPlanParser#root_relation}.
	 * @param ctx the parse tree
	 */
	fn visit_root_relation(&mut self, ctx: &Root_relationContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SubstraitPlanParser#relation_type}.
	 * @param ctx the parse tree
	 */
	fn visit_relation_type(&mut self, ctx: &Relation_typeContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SubstraitPlanParser#relation_ref}.
	 * @param ctx the parse tree
	 */
	fn visit_relation_ref(&mut self, ctx: &Relation_refContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SubstraitPlanParser#relation_filter_behavior}.
	 * @param ctx the parse tree
	 */
	fn visit_relation_filter_behavior(&mut self, ctx: &Relation_filter_behaviorContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SubstraitPlanParser#measure_detail}.
	 * @param ctx the parse tree
	 */
	fn visit_measure_detail(&mut self, ctx: &Measure_detailContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code relationCommon}
	 * labeled alternative in {@link SubstraitPlanParser#relation_detail}.
	 * @param ctx the parse tree
	 */
	fn visit_relationCommon(&mut self, ctx: &RelationCommonContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code relationUsesSchema}
	 * labeled alternative in {@link SubstraitPlanParser#relation_detail}.
	 * @param ctx the parse tree
	 */
	fn visit_relationUsesSchema(&mut self, ctx: &RelationUsesSchemaContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code relationFilter}
	 * labeled alternative in {@link SubstraitPlanParser#relation_detail}.
	 * @param ctx the parse tree
	 */
	fn visit_relationFilter(&mut self, ctx: &RelationFilterContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code relationExpression}
	 * labeled alternative in {@link SubstraitPlanParser#relation_detail}.
	 * @param ctx the parse tree
	 */
	fn visit_relationExpression(&mut self, ctx: &RelationExpressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code relationAdvancedExtension}
	 * labeled alternative in {@link SubstraitPlanParser#relation_detail}.
	 * @param ctx the parse tree
	 */
	fn visit_relationAdvancedExtension(&mut self, ctx: &RelationAdvancedExtensionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code relationSourceReference}
	 * labeled alternative in {@link SubstraitPlanParser#relation_detail}.
	 * @param ctx the parse tree
	 */
	fn visit_relationSourceReference(&mut self, ctx: &RelationSourceReferenceContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code relationGrouping}
	 * labeled alternative in {@link SubstraitPlanParser#relation_detail}.
	 * @param ctx the parse tree
	 */
	fn visit_relationGrouping(&mut self, ctx: &RelationGroupingContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code relationMeasure}
	 * labeled alternative in {@link SubstraitPlanParser#relation_detail}.
	 * @param ctx the parse tree
	 */
	fn visit_relationMeasure(&mut self, ctx: &RelationMeasureContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code relationSort}
	 * labeled alternative in {@link SubstraitPlanParser#relation_detail}.
	 * @param ctx the parse tree
	 */
	fn visit_relationSort(&mut self, ctx: &RelationSortContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code relationCount}
	 * labeled alternative in {@link SubstraitPlanParser#relation_detail}.
	 * @param ctx the parse tree
	 */
	fn visit_relationCount(&mut self, ctx: &RelationCountContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code relationOffset}
	 * labeled alternative in {@link SubstraitPlanParser#relation_detail}.
	 * @param ctx the parse tree
	 */
	fn visit_relationOffset(&mut self, ctx: &RelationOffsetContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code relationJoinType}
	 * labeled alternative in {@link SubstraitPlanParser#relation_detail}.
	 * @param ctx the parse tree
	 */
	fn visit_relationJoinType(&mut self, ctx: &RelationJoinTypeContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code relationEmit}
	 * labeled alternative in {@link SubstraitPlanParser#relation_detail}.
	 * @param ctx the parse tree
	 */
	fn visit_relationEmit(&mut self, ctx: &RelationEmitContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code expressionScalarSubquery}
	 * labeled alternative in {@link SubstraitPlanParser#expression}.
	 * @param ctx the parse tree
	 */
	fn visit_expressionScalarSubquery(&mut self, ctx: &ExpressionScalarSubqueryContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code expressionConstant}
	 * labeled alternative in {@link SubstraitPlanParser#expression}.
	 * @param ctx the parse tree
	 */
	fn visit_expressionConstant(&mut self, ctx: &ExpressionConstantContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code expressionFunctionUse}
	 * labeled alternative in {@link SubstraitPlanParser#expression}.
	 * @param ctx the parse tree
	 */
	fn visit_expressionFunctionUse(&mut self, ctx: &ExpressionFunctionUseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code expressionColumn}
	 * labeled alternative in {@link SubstraitPlanParser#expression}.
	 * @param ctx the parse tree
	 */
	fn visit_expressionColumn(&mut self, ctx: &ExpressionColumnContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code expressionSetComparisonSubquery}
	 * labeled alternative in {@link SubstraitPlanParser#expression}.
	 * @param ctx the parse tree
	 */
	fn visit_expressionSetComparisonSubquery(&mut self, ctx: &ExpressionSetComparisonSubqueryContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code expressionInPredicateSubquery}
	 * labeled alternative in {@link SubstraitPlanParser#expression}.
	 * @param ctx the parse tree
	 */
	fn visit_expressionInPredicateSubquery(&mut self, ctx: &ExpressionInPredicateSubqueryContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code expressionCast}
	 * labeled alternative in {@link SubstraitPlanParser#expression}.
	 * @param ctx the parse tree
	 */
	fn visit_expressionCast(&mut self, ctx: &ExpressionCastContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code expressionSetPredicateSubquery}
	 * labeled alternative in {@link SubstraitPlanParser#expression}.
	 * @param ctx the parse tree
	 */
	fn visit_expressionSetPredicateSubquery(&mut self, ctx: &ExpressionSetPredicateSubqueryContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SubstraitPlanParser#expression_list}.
	 * @param ctx the parse tree
	 */
	fn visit_expression_list(&mut self, ctx: &Expression_listContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SubstraitPlanParser#constant}.
	 * @param ctx the parse tree
	 */
	fn visit_constant(&mut self, ctx: &ConstantContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SubstraitPlanParser#literal_basic_type}.
	 * @param ctx the parse tree
	 */
	fn visit_literal_basic_type(&mut self, ctx: &Literal_basic_typeContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SubstraitPlanParser#literal_complex_type}.
	 * @param ctx the parse tree
	 */
	fn visit_literal_complex_type(&mut self, ctx: &Literal_complex_typeContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SubstraitPlanParser#literal_specifier}.
	 * @param ctx the parse tree
	 */
	fn visit_literal_specifier(&mut self, ctx: &Literal_specifierContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SubstraitPlanParser#map_literal}.
	 * @param ctx the parse tree
	 */
	fn visit_map_literal(&mut self, ctx: &Map_literalContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SubstraitPlanParser#map_literal_value}.
	 * @param ctx the parse tree
	 */
	fn visit_map_literal_value(&mut self, ctx: &Map_literal_valueContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SubstraitPlanParser#struct_literal}.
	 * @param ctx the parse tree
	 */
	fn visit_struct_literal(&mut self, ctx: &Struct_literalContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SubstraitPlanParser#column_name}.
	 * @param ctx the parse tree
	 */
	fn visit_column_name(&mut self, ctx: &Column_nameContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SubstraitPlanParser#source_reference}.
	 * @param ctx the parse tree
	 */
	fn visit_source_reference(&mut self, ctx: &Source_referenceContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SubstraitPlanParser#file_location}.
	 * @param ctx the parse tree
	 */
	fn visit_file_location(&mut self, ctx: &File_locationContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SubstraitPlanParser#file_detail}.
	 * @param ctx the parse tree
	 */
	fn visit_file_detail(&mut self, ctx: &File_detailContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SubstraitPlanParser#file}.
	 * @param ctx the parse tree
	 */
	fn visit_file(&mut self, ctx: &FileContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SubstraitPlanParser#local_files_detail}.
	 * @param ctx the parse tree
	 */
	fn visit_local_files_detail(&mut self, ctx: &Local_files_detailContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SubstraitPlanParser#named_table_detail}.
	 * @param ctx the parse tree
	 */
	fn visit_named_table_detail(&mut self, ctx: &Named_table_detailContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SubstraitPlanParser#schema_definition}.
	 * @param ctx the parse tree
	 */
	fn visit_schema_definition(&mut self, ctx: &Schema_definitionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SubstraitPlanParser#schema_item}.
	 * @param ctx the parse tree
	 */
	fn visit_schema_item(&mut self, ctx: &Schema_itemContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SubstraitPlanParser#source_definition}.
	 * @param ctx the parse tree
	 */
	fn visit_source_definition(&mut self, ctx: &Source_definitionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code localFiles}
	 * labeled alternative in {@link SubstraitPlanParser#read_type}.
	 * @param ctx the parse tree
	 */
	fn visit_localFiles(&mut self, ctx: &LocalFilesContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code virtualTable}
	 * labeled alternative in {@link SubstraitPlanParser#read_type}.
	 * @param ctx the parse tree
	 */
	fn visit_virtualTable(&mut self, ctx: &VirtualTableContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code namedTable}
	 * labeled alternative in {@link SubstraitPlanParser#read_type}.
	 * @param ctx the parse tree
	 */
	fn visit_namedTable(&mut self, ctx: &NamedTableContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code extensionTable}
	 * labeled alternative in {@link SubstraitPlanParser#read_type}.
	 * @param ctx the parse tree
	 */
	fn visit_extensionTable(&mut self, ctx: &ExtensionTableContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SubstraitPlanParser#extensionspace}.
	 * @param ctx the parse tree
	 */
	fn visit_extensionspace(&mut self, ctx: &ExtensionspaceContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SubstraitPlanParser#function}.
	 * @param ctx the parse tree
	 */
	fn visit_function(&mut self, ctx: &FunctionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SubstraitPlanParser#sort_field}.
	 * @param ctx the parse tree
	 */
	fn visit_sort_field(&mut self, ctx: &Sort_fieldContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SubstraitPlanParser#name}.
	 * @param ctx the parse tree
	 */
	fn visit_name(&mut self, ctx: &NameContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SubstraitPlanParser#signature}.
	 * @param ctx the parse tree
	 */
	fn visit_signature(&mut self, ctx: &SignatureContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SubstraitPlanParser#id}.
	 * @param ctx the parse tree
	 */
	fn visit_id(&mut self, ctx: &IdContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SubstraitPlanParser#simple_id}.
	 * @param ctx the parse tree
	 */
	fn visit_simple_id(&mut self, ctx: &Simple_idContext<'input>) { self.visit_children(ctx) }

}

pub trait SubstraitPlanParserVisitorCompat<'input>:ParseTreeVisitorCompat<'input, Node= SubstraitPlanParserContextType>{
	/**
	 * Visit a parse tree produced by {@link SubstraitPlanParser#plan}.
	 * @param ctx the parse tree
	 */
		fn visit_plan(&mut self, ctx: &PlanContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SubstraitPlanParser#plan_detail}.
	 * @param ctx the parse tree
	 */
		fn visit_plan_detail(&mut self, ctx: &Plan_detailContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SubstraitPlanParser#pipelines}.
	 * @param ctx the parse tree
	 */
		fn visit_pipelines(&mut self, ctx: &PipelinesContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SubstraitPlanParser#pipeline}.
	 * @param ctx the parse tree
	 */
		fn visit_pipeline(&mut self, ctx: &PipelineContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SubstraitPlanParser#relation}.
	 * @param ctx the parse tree
	 */
		fn visit_relation(&mut self, ctx: &RelationContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SubstraitPlanParser#root_relation}.
	 * @param ctx the parse tree
	 */
		fn visit_root_relation(&mut self, ctx: &Root_relationContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SubstraitPlanParser#relation_type}.
	 * @param ctx the parse tree
	 */
		fn visit_relation_type(&mut self, ctx: &Relation_typeContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SubstraitPlanParser#relation_ref}.
	 * @param ctx the parse tree
	 */
		fn visit_relation_ref(&mut self, ctx: &Relation_refContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SubstraitPlanParser#relation_filter_behavior}.
	 * @param ctx the parse tree
	 */
		fn visit_relation_filter_behavior(&mut self, ctx: &Relation_filter_behaviorContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SubstraitPlanParser#measure_detail}.
	 * @param ctx the parse tree
	 */
		fn visit_measure_detail(&mut self, ctx: &Measure_detailContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code relationCommon}
	 * labeled alternative in {@link SubstraitPlanParser#relation_detail}.
	 * @param ctx the parse tree
	 */
		fn visit_relationCommon(&mut self, ctx: &RelationCommonContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code relationUsesSchema}
	 * labeled alternative in {@link SubstraitPlanParser#relation_detail}.
	 * @param ctx the parse tree
	 */
		fn visit_relationUsesSchema(&mut self, ctx: &RelationUsesSchemaContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code relationFilter}
	 * labeled alternative in {@link SubstraitPlanParser#relation_detail}.
	 * @param ctx the parse tree
	 */
		fn visit_relationFilter(&mut self, ctx: &RelationFilterContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code relationExpression}
	 * labeled alternative in {@link SubstraitPlanParser#relation_detail}.
	 * @param ctx the parse tree
	 */
		fn visit_relationExpression(&mut self, ctx: &RelationExpressionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code relationAdvancedExtension}
	 * labeled alternative in {@link SubstraitPlanParser#relation_detail}.
	 * @param ctx the parse tree
	 */
		fn visit_relationAdvancedExtension(&mut self, ctx: &RelationAdvancedExtensionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code relationSourceReference}
	 * labeled alternative in {@link SubstraitPlanParser#relation_detail}.
	 * @param ctx the parse tree
	 */
		fn visit_relationSourceReference(&mut self, ctx: &RelationSourceReferenceContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code relationGrouping}
	 * labeled alternative in {@link SubstraitPlanParser#relation_detail}.
	 * @param ctx the parse tree
	 */
		fn visit_relationGrouping(&mut self, ctx: &RelationGroupingContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code relationMeasure}
	 * labeled alternative in {@link SubstraitPlanParser#relation_detail}.
	 * @param ctx the parse tree
	 */
		fn visit_relationMeasure(&mut self, ctx: &RelationMeasureContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code relationSort}
	 * labeled alternative in {@link SubstraitPlanParser#relation_detail}.
	 * @param ctx the parse tree
	 */
		fn visit_relationSort(&mut self, ctx: &RelationSortContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code relationCount}
	 * labeled alternative in {@link SubstraitPlanParser#relation_detail}.
	 * @param ctx the parse tree
	 */
		fn visit_relationCount(&mut self, ctx: &RelationCountContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code relationOffset}
	 * labeled alternative in {@link SubstraitPlanParser#relation_detail}.
	 * @param ctx the parse tree
	 */
		fn visit_relationOffset(&mut self, ctx: &RelationOffsetContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code relationJoinType}
	 * labeled alternative in {@link SubstraitPlanParser#relation_detail}.
	 * @param ctx the parse tree
	 */
		fn visit_relationJoinType(&mut self, ctx: &RelationJoinTypeContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code relationEmit}
	 * labeled alternative in {@link SubstraitPlanParser#relation_detail}.
	 * @param ctx the parse tree
	 */
		fn visit_relationEmit(&mut self, ctx: &RelationEmitContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code expressionScalarSubquery}
	 * labeled alternative in {@link SubstraitPlanParser#expression}.
	 * @param ctx the parse tree
	 */
		fn visit_expressionScalarSubquery(&mut self, ctx: &ExpressionScalarSubqueryContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code expressionConstant}
	 * labeled alternative in {@link SubstraitPlanParser#expression}.
	 * @param ctx the parse tree
	 */
		fn visit_expressionConstant(&mut self, ctx: &ExpressionConstantContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code expressionFunctionUse}
	 * labeled alternative in {@link SubstraitPlanParser#expression}.
	 * @param ctx the parse tree
	 */
		fn visit_expressionFunctionUse(&mut self, ctx: &ExpressionFunctionUseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code expressionColumn}
	 * labeled alternative in {@link SubstraitPlanParser#expression}.
	 * @param ctx the parse tree
	 */
		fn visit_expressionColumn(&mut self, ctx: &ExpressionColumnContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code expressionSetComparisonSubquery}
	 * labeled alternative in {@link SubstraitPlanParser#expression}.
	 * @param ctx the parse tree
	 */
		fn visit_expressionSetComparisonSubquery(&mut self, ctx: &ExpressionSetComparisonSubqueryContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code expressionInPredicateSubquery}
	 * labeled alternative in {@link SubstraitPlanParser#expression}.
	 * @param ctx the parse tree
	 */
		fn visit_expressionInPredicateSubquery(&mut self, ctx: &ExpressionInPredicateSubqueryContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code expressionCast}
	 * labeled alternative in {@link SubstraitPlanParser#expression}.
	 * @param ctx the parse tree
	 */
		fn visit_expressionCast(&mut self, ctx: &ExpressionCastContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code expressionSetPredicateSubquery}
	 * labeled alternative in {@link SubstraitPlanParser#expression}.
	 * @param ctx the parse tree
	 */
		fn visit_expressionSetPredicateSubquery(&mut self, ctx: &ExpressionSetPredicateSubqueryContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SubstraitPlanParser#expression_list}.
	 * @param ctx the parse tree
	 */
		fn visit_expression_list(&mut self, ctx: &Expression_listContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SubstraitPlanParser#constant}.
	 * @param ctx the parse tree
	 */
		fn visit_constant(&mut self, ctx: &ConstantContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SubstraitPlanParser#literal_basic_type}.
	 * @param ctx the parse tree
	 */
		fn visit_literal_basic_type(&mut self, ctx: &Literal_basic_typeContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SubstraitPlanParser#literal_complex_type}.
	 * @param ctx the parse tree
	 */
		fn visit_literal_complex_type(&mut self, ctx: &Literal_complex_typeContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SubstraitPlanParser#literal_specifier}.
	 * @param ctx the parse tree
	 */
		fn visit_literal_specifier(&mut self, ctx: &Literal_specifierContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SubstraitPlanParser#map_literal}.
	 * @param ctx the parse tree
	 */
		fn visit_map_literal(&mut self, ctx: &Map_literalContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SubstraitPlanParser#map_literal_value}.
	 * @param ctx the parse tree
	 */
		fn visit_map_literal_value(&mut self, ctx: &Map_literal_valueContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SubstraitPlanParser#struct_literal}.
	 * @param ctx the parse tree
	 */
		fn visit_struct_literal(&mut self, ctx: &Struct_literalContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SubstraitPlanParser#column_name}.
	 * @param ctx the parse tree
	 */
		fn visit_column_name(&mut self, ctx: &Column_nameContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SubstraitPlanParser#source_reference}.
	 * @param ctx the parse tree
	 */
		fn visit_source_reference(&mut self, ctx: &Source_referenceContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SubstraitPlanParser#file_location}.
	 * @param ctx the parse tree
	 */
		fn visit_file_location(&mut self, ctx: &File_locationContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SubstraitPlanParser#file_detail}.
	 * @param ctx the parse tree
	 */
		fn visit_file_detail(&mut self, ctx: &File_detailContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SubstraitPlanParser#file}.
	 * @param ctx the parse tree
	 */
		fn visit_file(&mut self, ctx: &FileContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SubstraitPlanParser#local_files_detail}.
	 * @param ctx the parse tree
	 */
		fn visit_local_files_detail(&mut self, ctx: &Local_files_detailContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SubstraitPlanParser#named_table_detail}.
	 * @param ctx the parse tree
	 */
		fn visit_named_table_detail(&mut self, ctx: &Named_table_detailContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SubstraitPlanParser#schema_definition}.
	 * @param ctx the parse tree
	 */
		fn visit_schema_definition(&mut self, ctx: &Schema_definitionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SubstraitPlanParser#schema_item}.
	 * @param ctx the parse tree
	 */
		fn visit_schema_item(&mut self, ctx: &Schema_itemContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SubstraitPlanParser#source_definition}.
	 * @param ctx the parse tree
	 */
		fn visit_source_definition(&mut self, ctx: &Source_definitionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code localFiles}
	 * labeled alternative in {@link SubstraitPlanParser#read_type}.
	 * @param ctx the parse tree
	 */
		fn visit_localFiles(&mut self, ctx: &LocalFilesContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code virtualTable}
	 * labeled alternative in {@link SubstraitPlanParser#read_type}.
	 * @param ctx the parse tree
	 */
		fn visit_virtualTable(&mut self, ctx: &VirtualTableContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code namedTable}
	 * labeled alternative in {@link SubstraitPlanParser#read_type}.
	 * @param ctx the parse tree
	 */
		fn visit_namedTable(&mut self, ctx: &NamedTableContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code extensionTable}
	 * labeled alternative in {@link SubstraitPlanParser#read_type}.
	 * @param ctx the parse tree
	 */
		fn visit_extensionTable(&mut self, ctx: &ExtensionTableContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SubstraitPlanParser#extensionspace}.
	 * @param ctx the parse tree
	 */
		fn visit_extensionspace(&mut self, ctx: &ExtensionspaceContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SubstraitPlanParser#function}.
	 * @param ctx the parse tree
	 */
		fn visit_function(&mut self, ctx: &FunctionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SubstraitPlanParser#sort_field}.
	 * @param ctx the parse tree
	 */
		fn visit_sort_field(&mut self, ctx: &Sort_fieldContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SubstraitPlanParser#name}.
	 * @param ctx the parse tree
	 */
		fn visit_name(&mut self, ctx: &NameContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SubstraitPlanParser#signature}.
	 * @param ctx the parse tree
	 */
		fn visit_signature(&mut self, ctx: &SignatureContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SubstraitPlanParser#id}.
	 * @param ctx the parse tree
	 */
		fn visit_id(&mut self, ctx: &IdContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SubstraitPlanParser#simple_id}.
	 * @param ctx the parse tree
	 */
		fn visit_simple_id(&mut self, ctx: &Simple_idContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

}

impl<'input,T> SubstraitPlanParserVisitor<'input> for T
where
	T: SubstraitPlanParserVisitorCompat<'input>
{
	fn visit_plan(&mut self, ctx: &PlanContext<'input>){
		let result = <Self as SubstraitPlanParserVisitorCompat>::visit_plan(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_plan_detail(&mut self, ctx: &Plan_detailContext<'input>){
		let result = <Self as SubstraitPlanParserVisitorCompat>::visit_plan_detail(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_pipelines(&mut self, ctx: &PipelinesContext<'input>){
		let result = <Self as SubstraitPlanParserVisitorCompat>::visit_pipelines(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_pipeline(&mut self, ctx: &PipelineContext<'input>){
		let result = <Self as SubstraitPlanParserVisitorCompat>::visit_pipeline(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_relation(&mut self, ctx: &RelationContext<'input>){
		let result = <Self as SubstraitPlanParserVisitorCompat>::visit_relation(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_root_relation(&mut self, ctx: &Root_relationContext<'input>){
		let result = <Self as SubstraitPlanParserVisitorCompat>::visit_root_relation(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_relation_type(&mut self, ctx: &Relation_typeContext<'input>){
		let result = <Self as SubstraitPlanParserVisitorCompat>::visit_relation_type(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_relation_ref(&mut self, ctx: &Relation_refContext<'input>){
		let result = <Self as SubstraitPlanParserVisitorCompat>::visit_relation_ref(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_relation_filter_behavior(&mut self, ctx: &Relation_filter_behaviorContext<'input>){
		let result = <Self as SubstraitPlanParserVisitorCompat>::visit_relation_filter_behavior(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_measure_detail(&mut self, ctx: &Measure_detailContext<'input>){
		let result = <Self as SubstraitPlanParserVisitorCompat>::visit_measure_detail(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_relationCommon(&mut self, ctx: &RelationCommonContext<'input>){
		let result = <Self as SubstraitPlanParserVisitorCompat>::visit_relationCommon(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_relationUsesSchema(&mut self, ctx: &RelationUsesSchemaContext<'input>){
		let result = <Self as SubstraitPlanParserVisitorCompat>::visit_relationUsesSchema(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_relationFilter(&mut self, ctx: &RelationFilterContext<'input>){
		let result = <Self as SubstraitPlanParserVisitorCompat>::visit_relationFilter(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_relationExpression(&mut self, ctx: &RelationExpressionContext<'input>){
		let result = <Self as SubstraitPlanParserVisitorCompat>::visit_relationExpression(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_relationAdvancedExtension(&mut self, ctx: &RelationAdvancedExtensionContext<'input>){
		let result = <Self as SubstraitPlanParserVisitorCompat>::visit_relationAdvancedExtension(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_relationSourceReference(&mut self, ctx: &RelationSourceReferenceContext<'input>){
		let result = <Self as SubstraitPlanParserVisitorCompat>::visit_relationSourceReference(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_relationGrouping(&mut self, ctx: &RelationGroupingContext<'input>){
		let result = <Self as SubstraitPlanParserVisitorCompat>::visit_relationGrouping(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_relationMeasure(&mut self, ctx: &RelationMeasureContext<'input>){
		let result = <Self as SubstraitPlanParserVisitorCompat>::visit_relationMeasure(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_relationSort(&mut self, ctx: &RelationSortContext<'input>){
		let result = <Self as SubstraitPlanParserVisitorCompat>::visit_relationSort(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_relationCount(&mut self, ctx: &RelationCountContext<'input>){
		let result = <Self as SubstraitPlanParserVisitorCompat>::visit_relationCount(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_relationOffset(&mut self, ctx: &RelationOffsetContext<'input>){
		let result = <Self as SubstraitPlanParserVisitorCompat>::visit_relationOffset(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_relationJoinType(&mut self, ctx: &RelationJoinTypeContext<'input>){
		let result = <Self as SubstraitPlanParserVisitorCompat>::visit_relationJoinType(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_relationEmit(&mut self, ctx: &RelationEmitContext<'input>){
		let result = <Self as SubstraitPlanParserVisitorCompat>::visit_relationEmit(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_expressionScalarSubquery(&mut self, ctx: &ExpressionScalarSubqueryContext<'input>){
		let result = <Self as SubstraitPlanParserVisitorCompat>::visit_expressionScalarSubquery(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_expressionConstant(&mut self, ctx: &ExpressionConstantContext<'input>){
		let result = <Self as SubstraitPlanParserVisitorCompat>::visit_expressionConstant(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_expressionFunctionUse(&mut self, ctx: &ExpressionFunctionUseContext<'input>){
		let result = <Self as SubstraitPlanParserVisitorCompat>::visit_expressionFunctionUse(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_expressionColumn(&mut self, ctx: &ExpressionColumnContext<'input>){
		let result = <Self as SubstraitPlanParserVisitorCompat>::visit_expressionColumn(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_expressionSetComparisonSubquery(&mut self, ctx: &ExpressionSetComparisonSubqueryContext<'input>){
		let result = <Self as SubstraitPlanParserVisitorCompat>::visit_expressionSetComparisonSubquery(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_expressionInPredicateSubquery(&mut self, ctx: &ExpressionInPredicateSubqueryContext<'input>){
		let result = <Self as SubstraitPlanParserVisitorCompat>::visit_expressionInPredicateSubquery(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_expressionCast(&mut self, ctx: &ExpressionCastContext<'input>){
		let result = <Self as SubstraitPlanParserVisitorCompat>::visit_expressionCast(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_expressionSetPredicateSubquery(&mut self, ctx: &ExpressionSetPredicateSubqueryContext<'input>){
		let result = <Self as SubstraitPlanParserVisitorCompat>::visit_expressionSetPredicateSubquery(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_expression_list(&mut self, ctx: &Expression_listContext<'input>){
		let result = <Self as SubstraitPlanParserVisitorCompat>::visit_expression_list(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_constant(&mut self, ctx: &ConstantContext<'input>){
		let result = <Self as SubstraitPlanParserVisitorCompat>::visit_constant(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_literal_basic_type(&mut self, ctx: &Literal_basic_typeContext<'input>){
		let result = <Self as SubstraitPlanParserVisitorCompat>::visit_literal_basic_type(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_literal_complex_type(&mut self, ctx: &Literal_complex_typeContext<'input>){
		let result = <Self as SubstraitPlanParserVisitorCompat>::visit_literal_complex_type(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_literal_specifier(&mut self, ctx: &Literal_specifierContext<'input>){
		let result = <Self as SubstraitPlanParserVisitorCompat>::visit_literal_specifier(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_map_literal(&mut self, ctx: &Map_literalContext<'input>){
		let result = <Self as SubstraitPlanParserVisitorCompat>::visit_map_literal(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_map_literal_value(&mut self, ctx: &Map_literal_valueContext<'input>){
		let result = <Self as SubstraitPlanParserVisitorCompat>::visit_map_literal_value(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_struct_literal(&mut self, ctx: &Struct_literalContext<'input>){
		let result = <Self as SubstraitPlanParserVisitorCompat>::visit_struct_literal(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_column_name(&mut self, ctx: &Column_nameContext<'input>){
		let result = <Self as SubstraitPlanParserVisitorCompat>::visit_column_name(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_source_reference(&mut self, ctx: &Source_referenceContext<'input>){
		let result = <Self as SubstraitPlanParserVisitorCompat>::visit_source_reference(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_file_location(&mut self, ctx: &File_locationContext<'input>){
		let result = <Self as SubstraitPlanParserVisitorCompat>::visit_file_location(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_file_detail(&mut self, ctx: &File_detailContext<'input>){
		let result = <Self as SubstraitPlanParserVisitorCompat>::visit_file_detail(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_file(&mut self, ctx: &FileContext<'input>){
		let result = <Self as SubstraitPlanParserVisitorCompat>::visit_file(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_local_files_detail(&mut self, ctx: &Local_files_detailContext<'input>){
		let result = <Self as SubstraitPlanParserVisitorCompat>::visit_local_files_detail(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_named_table_detail(&mut self, ctx: &Named_table_detailContext<'input>){
		let result = <Self as SubstraitPlanParserVisitorCompat>::visit_named_table_detail(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_schema_definition(&mut self, ctx: &Schema_definitionContext<'input>){
		let result = <Self as SubstraitPlanParserVisitorCompat>::visit_schema_definition(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_schema_item(&mut self, ctx: &Schema_itemContext<'input>){
		let result = <Self as SubstraitPlanParserVisitorCompat>::visit_schema_item(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_source_definition(&mut self, ctx: &Source_definitionContext<'input>){
		let result = <Self as SubstraitPlanParserVisitorCompat>::visit_source_definition(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_localFiles(&mut self, ctx: &LocalFilesContext<'input>){
		let result = <Self as SubstraitPlanParserVisitorCompat>::visit_localFiles(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_virtualTable(&mut self, ctx: &VirtualTableContext<'input>){
		let result = <Self as SubstraitPlanParserVisitorCompat>::visit_virtualTable(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_namedTable(&mut self, ctx: &NamedTableContext<'input>){
		let result = <Self as SubstraitPlanParserVisitorCompat>::visit_namedTable(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_extensionTable(&mut self, ctx: &ExtensionTableContext<'input>){
		let result = <Self as SubstraitPlanParserVisitorCompat>::visit_extensionTable(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_extensionspace(&mut self, ctx: &ExtensionspaceContext<'input>){
		let result = <Self as SubstraitPlanParserVisitorCompat>::visit_extensionspace(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_function(&mut self, ctx: &FunctionContext<'input>){
		let result = <Self as SubstraitPlanParserVisitorCompat>::visit_function(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_sort_field(&mut self, ctx: &Sort_fieldContext<'input>){
		let result = <Self as SubstraitPlanParserVisitorCompat>::visit_sort_field(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_name(&mut self, ctx: &NameContext<'input>){
		let result = <Self as SubstraitPlanParserVisitorCompat>::visit_name(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_signature(&mut self, ctx: &SignatureContext<'input>){
		let result = <Self as SubstraitPlanParserVisitorCompat>::visit_signature(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_id(&mut self, ctx: &IdContext<'input>){
		let result = <Self as SubstraitPlanParserVisitorCompat>::visit_id(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_simple_id(&mut self, ctx: &Simple_idContext<'input>){
		let result = <Self as SubstraitPlanParserVisitorCompat>::visit_simple_id(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

}