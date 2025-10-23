// Generated from src/substrait/textplan/parser/grammar/SubstraitPlanParser.g4 by ANTLR 4.8
#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
use antlr_rust::PredictionContextCache;
use antlr_rust::parser::{Parser, BaseParser, ParserRecog, ParserNodeType};
use antlr_rust::token_stream::TokenStream;
use antlr_rust::TokenSource;
use antlr_rust::parser_atn_simulator::ParserATNSimulator;
use antlr_rust::errors::*;
use antlr_rust::rule_context::{BaseRuleContext, CustomRuleContext, RuleContext};
use antlr_rust::recognizer::{Recognizer,Actions};
use antlr_rust::atn_deserializer::ATNDeserializer;
use antlr_rust::dfa::DFA;
use antlr_rust::atn::{ATN, INVALID_ALT};
use antlr_rust::error_strategy::{ErrorStrategy, DefaultErrorStrategy};
use antlr_rust::parser_rule_context::{BaseParserRuleContext, ParserRuleContext,cast,cast_mut};
use antlr_rust::tree::*;
use antlr_rust::token::{TOKEN_EOF,OwningToken,Token};
use antlr_rust::int_stream::EOF;
use antlr_rust::vocabulary::{Vocabulary,VocabularyImpl};
use antlr_rust::token_factory::{CommonTokenFactory,TokenFactory, TokenAware};
use super::substraitplanparserlistener::*;
use antlr_rust::lazy_static;
use antlr_rust::{TidAble,TidExt};

use std::marker::PhantomData;
use std::sync::Arc;
use std::rc::Rc;
use std::convert::TryFrom;
use std::cell::RefCell;
use std::ops::{DerefMut, Deref};
use std::borrow::{Borrow,BorrowMut};
use std::any::{Any,TypeId};

		pub const SPACES:isize=1; 
		pub const EXTENSION_SPACE:isize=2; 
		pub const FUNCTION:isize=3; 
		pub const AS:isize=4; 
		pub const NAMED:isize=5; 
		pub const SCHEMA:isize=6; 
		pub const RELATION:isize=7; 
		pub const PIPELINES:isize=8; 
		pub const COMMON:isize=9; 
		pub const BASE_SCHEMA:isize=10; 
		pub const FILTER:isize=11; 
		pub const PROJECTION:isize=12; 
		pub const EXPRESSION:isize=13; 
		pub const ADVANCED_EXTENSION:isize=14; 
		pub const GROUPING:isize=15; 
		pub const MEASURE:isize=16; 
		pub const INVOCATION:isize=17; 
		pub const SORT:isize=18; 
		pub const BY:isize=19; 
		pub const COUNT:isize=20; 
		pub const TYPE:isize=21; 
		pub const EMIT:isize=22; 
		pub const SUBQUERY:isize=23; 
		pub const EXISTS:isize=24; 
		pub const UNIQUE:isize=25; 
		pub const IN:isize=26; 
		pub const ALL:isize=27; 
		pub const ANY:isize=28; 
		pub const COMPARISON:isize=29; 
		pub const VIRTUAL_TABLE:isize=30; 
		pub const LOCAL_FILES:isize=31; 
		pub const NAMED_TABLE:isize=32; 
		pub const EXTENSION_TABLE:isize=33; 
		pub const SOURCE:isize=34; 
		pub const ROOT:isize=35; 
		pub const ITEMS:isize=36; 
		pub const NAMES:isize=37; 
		pub const URI_FILE:isize=38; 
		pub const URI_PATH:isize=39; 
		pub const URI_PATH_GLOB:isize=40; 
		pub const URI_FOLDER:isize=41; 
		pub const PARTITION_INDEX:isize=42; 
		pub const START:isize=43; 
		pub const LENGTH:isize=44; 
		pub const ORC:isize=45; 
		pub const PARQUET:isize=46; 
		pub const NULLVAL:isize=47; 
		pub const TRUEVAL:isize=48; 
		pub const FALSEVAL:isize=49; 
		pub const LIST:isize=50; 
		pub const MAP:isize=51; 
		pub const STRUCT:isize=52; 
		pub const ARROW:isize=53; 
		pub const COLON:isize=54; 
		pub const SEMICOLON:isize=55; 
		pub const LEFTBRACE:isize=56; 
		pub const RIGHTBRACE:isize=57; 
		pub const LEFTPAREN:isize=58; 
		pub const RIGHTPAREN:isize=59; 
		pub const COMMA:isize=60; 
		pub const PERIOD:isize=61; 
		pub const EQUAL:isize=62; 
		pub const LEFTBRACKET:isize=63; 
		pub const RIGHTBRACKET:isize=64; 
		pub const UNDERSCORE:isize=65; 
		pub const MINUS:isize=66; 
		pub const LEFTANGLEBRACKET:isize=67; 
		pub const RIGHTANGLEBRACKET:isize=68; 
		pub const QUESTIONMARK:isize=69; 
		pub const ATSIGN:isize=70; 
		pub const IDENTIFIER:isize=71; 
		pub const NUMBER:isize=72; 
		pub const STRING:isize=73; 
		pub const SINGLE_LINE_COMMENT:isize=74; 
		pub const URI:isize=75;
	pub const RULE_plan:usize = 0; 
	pub const RULE_plan_detail:usize = 1; 
	pub const RULE_pipelines:usize = 2; 
	pub const RULE_pipeline:usize = 3; 
	pub const RULE_relation:usize = 4; 
	pub const RULE_root_relation:usize = 5; 
	pub const RULE_relation_type:usize = 6; 
	pub const RULE_relation_ref:usize = 7; 
	pub const RULE_relation_filter_behavior:usize = 8; 
	pub const RULE_measure_detail:usize = 9; 
	pub const RULE_relation_detail:usize = 10; 
	pub const RULE_expression:usize = 11; 
	pub const RULE_expression_list:usize = 12; 
	pub const RULE_constant:usize = 13; 
	pub const RULE_literal_basic_type:usize = 14; 
	pub const RULE_literal_complex_type:usize = 15; 
	pub const RULE_literal_specifier:usize = 16; 
	pub const RULE_map_literal:usize = 17; 
	pub const RULE_map_literal_value:usize = 18; 
	pub const RULE_struct_literal:usize = 19; 
	pub const RULE_column_name:usize = 20; 
	pub const RULE_source_reference:usize = 21; 
	pub const RULE_file_location:usize = 22; 
	pub const RULE_file_detail:usize = 23; 
	pub const RULE_file:usize = 24; 
	pub const RULE_local_files_detail:usize = 25; 
	pub const RULE_named_table_detail:usize = 26; 
	pub const RULE_schema_definition:usize = 27; 
	pub const RULE_schema_item:usize = 28; 
	pub const RULE_source_definition:usize = 29; 
	pub const RULE_read_type:usize = 30; 
	pub const RULE_extensionspace:usize = 31; 
	pub const RULE_function:usize = 32; 
	pub const RULE_sort_field:usize = 33; 
	pub const RULE_name:usize = 34; 
	pub const RULE_signature:usize = 35; 
	pub const RULE_id:usize = 36; 
	pub const RULE_simple_id:usize = 37;
	pub const ruleNames: [&'static str; 38] =  [
		"plan", "plan_detail", "pipelines", "pipeline", "relation", "root_relation", 
		"relation_type", "relation_ref", "relation_filter_behavior", "measure_detail", 
		"relation_detail", "expression", "expression_list", "constant", "literal_basic_type", 
		"literal_complex_type", "literal_specifier", "map_literal", "map_literal_value", 
		"struct_literal", "column_name", "source_reference", "file_location", 
		"file_detail", "file", "local_files_detail", "named_table_detail", "schema_definition", 
		"schema_item", "source_definition", "read_type", "extensionspace", "function", 
		"sort_field", "name", "signature", "id", "simple_id"
	];


	pub const _LITERAL_NAMES: [Option<&'static str>;71] = [
		None, None, Some("'EXTENSION_SPACE'"), Some("'FUNCTION'"), Some("'AS'"), 
		Some("'NAMED'"), Some("'SCHEMA'"), Some("'RELATION'"), Some("'PIPELINES'"), 
		Some("'COMMON'"), Some("'BASE_SCHEMA'"), Some("'FILTER'"), Some("'PROJECTION'"), 
		Some("'EXPRESSION'"), Some("'ADVANCED_EXTENSION'"), Some("'GROUPING'"), 
		Some("'MEASURE'"), Some("'INVOCATION'"), Some("'SORT'"), Some("'BY'"), 
		Some("'COUNT'"), Some("'TYPE'"), Some("'EMIT'"), Some("'SUBQUERY'"), Some("'EXISTS'"), 
		Some("'UNIQUE'"), Some("'IN'"), Some("'ALL'"), Some("'ANY'"), None, Some("'VIRTUAL_TABLE'"), 
		Some("'LOCAL_FILES'"), Some("'NAMED_TABLE'"), Some("'EXTENSION_TABLE'"), 
		Some("'SOURCE'"), Some("'ROOT'"), Some("'ITEMS'"), Some("'NAMES'"), Some("'URI_FILE'"), 
		Some("'URI_PATH'"), Some("'URI_PATH_GLOB'"), Some("'URI_FOLDER'"), Some("'PARTITION_INDEX'"), 
		Some("'START'"), Some("'LENGTH'"), Some("'ORC'"), Some("'PARQUET'"), Some("'NULL'"), 
		Some("'TRUE'"), Some("'FALSE'"), Some("'LIST'"), Some("'MAP'"), Some("'STRUCT'"), 
		Some("'->'"), Some("':'"), Some("';'"), Some("'{'"), Some("'}'"), Some("'('"), 
		Some("')'"), Some("','"), Some("'.'"), Some("'='"), Some("'['"), Some("']'"), 
		Some("'_'"), Some("'-'"), Some("'<'"), Some("'>'"), Some("'?'"), Some("'@'")
	];
	pub const _SYMBOLIC_NAMES: [Option<&'static str>;76]  = [
		None, Some("SPACES"), Some("EXTENSION_SPACE"), Some("FUNCTION"), Some("AS"), 
		Some("NAMED"), Some("SCHEMA"), Some("RELATION"), Some("PIPELINES"), Some("COMMON"), 
		Some("BASE_SCHEMA"), Some("FILTER"), Some("PROJECTION"), Some("EXPRESSION"), 
		Some("ADVANCED_EXTENSION"), Some("GROUPING"), Some("MEASURE"), Some("INVOCATION"), 
		Some("SORT"), Some("BY"), Some("COUNT"), Some("TYPE"), Some("EMIT"), Some("SUBQUERY"), 
		Some("EXISTS"), Some("UNIQUE"), Some("IN"), Some("ALL"), Some("ANY"), 
		Some("COMPARISON"), Some("VIRTUAL_TABLE"), Some("LOCAL_FILES"), Some("NAMED_TABLE"), 
		Some("EXTENSION_TABLE"), Some("SOURCE"), Some("ROOT"), Some("ITEMS"), 
		Some("NAMES"), Some("URI_FILE"), Some("URI_PATH"), Some("URI_PATH_GLOB"), 
		Some("URI_FOLDER"), Some("PARTITION_INDEX"), Some("START"), Some("LENGTH"), 
		Some("ORC"), Some("PARQUET"), Some("NULLVAL"), Some("TRUEVAL"), Some("FALSEVAL"), 
		Some("LIST"), Some("MAP"), Some("STRUCT"), Some("ARROW"), Some("COLON"), 
		Some("SEMICOLON"), Some("LEFTBRACE"), Some("RIGHTBRACE"), Some("LEFTPAREN"), 
		Some("RIGHTPAREN"), Some("COMMA"), Some("PERIOD"), Some("EQUAL"), Some("LEFTBRACKET"), 
		Some("RIGHTBRACKET"), Some("UNDERSCORE"), Some("MINUS"), Some("LEFTANGLEBRACKET"), 
		Some("RIGHTANGLEBRACKET"), Some("QUESTIONMARK"), Some("ATSIGN"), Some("IDENTIFIER"), 
		Some("NUMBER"), Some("STRING"), Some("SINGLE_LINE_COMMENT"), Some("URI")
	];
	lazy_static!{
	    static ref _shared_context_cache: Arc<PredictionContextCache> = Arc::new(PredictionContextCache::new());
		static ref VOCABULARY: Box<dyn Vocabulary> = Box::new(VocabularyImpl::new(_LITERAL_NAMES.iter(), _SYMBOLIC_NAMES.iter(), None));
	}


type BaseParserType<'input, I> =
	BaseParser<'input,SubstraitPlanParserExt<'input>, I, SubstraitPlanParserContextType , dyn SubstraitPlanParserListener<'input> + 'input >;

type TokenType<'input> = <LocalTokenFactory<'input> as TokenFactory<'input>>::Tok;
pub type LocalTokenFactory<'input> = CommonTokenFactory;

pub type SubstraitPlanParserTreeWalker<'input,'a> =
	ParseTreeWalker<'input, 'a, SubstraitPlanParserContextType , dyn SubstraitPlanParserListener<'input> + 'a>;

/// Parser for SubstraitPlanParser grammar
pub struct SubstraitPlanParser<'input,I,H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	base:BaseParserType<'input,I>,
	interpreter:Arc<ParserATNSimulator>,
	_shared_context_cache: Box<PredictionContextCache>,
    pub err_handler: H,
}

impl<'input, I, H> SubstraitPlanParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn get_serialized_atn() -> &'static str { _serializedATN }

    pub fn set_error_strategy(&mut self, strategy: H) {
        self.err_handler = strategy
    }

    pub fn with_strategy(input: I, strategy: H) -> Self {
		antlr_rust::recognizer::check_version("0","3");
		let interpreter = Arc::new(ParserATNSimulator::new(
			_ATN.clone(),
			_decision_to_DFA.clone(),
			_shared_context_cache.clone(),
		));
		Self {
			base: BaseParser::new_base_parser(
				input,
				Arc::clone(&interpreter),
				SubstraitPlanParserExt{
					_pd: Default::default(),
				}
			),
			interpreter,
            _shared_context_cache: Box::new(PredictionContextCache::new()),
            err_handler: strategy,
        }
    }

}

type DynStrategy<'input,I> = Box<dyn ErrorStrategy<'input,BaseParserType<'input,I>> + 'input>;

impl<'input, I> SubstraitPlanParser<'input, I, DynStrategy<'input,I>>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
    pub fn with_dyn_strategy(input: I) -> Self{
    	Self::with_strategy(input,Box::new(DefaultErrorStrategy::new()))
    }
}

impl<'input, I> SubstraitPlanParser<'input, I, DefaultErrorStrategy<'input,SubstraitPlanParserContextType>>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
    pub fn new(input: I) -> Self{
    	Self::with_strategy(input,DefaultErrorStrategy::new())
    }
}

/// Trait for monomorphized trait object that corresponds to the nodes of parse tree generated for SubstraitPlanParser
pub trait SubstraitPlanParserContext<'input>:
	for<'x> Listenable<dyn SubstraitPlanParserListener<'input> + 'x > + 
	ParserRuleContext<'input, TF=LocalTokenFactory<'input>, Ctx=SubstraitPlanParserContextType>
{}

antlr_rust::coerce_from!{ 'input : SubstraitPlanParserContext<'input> }

impl<'input> SubstraitPlanParserContext<'input> for TerminalNode<'input,SubstraitPlanParserContextType> {}
impl<'input> SubstraitPlanParserContext<'input> for ErrorNode<'input,SubstraitPlanParserContextType> {}

antlr_rust::tid! { impl<'input> TidAble<'input> for dyn SubstraitPlanParserContext<'input> + 'input }

antlr_rust::tid! { impl<'input> TidAble<'input> for dyn SubstraitPlanParserListener<'input> + 'input }

pub struct SubstraitPlanParserContextType;
antlr_rust::tid!{SubstraitPlanParserContextType}

impl<'input> ParserNodeType<'input> for SubstraitPlanParserContextType{
	type TF = LocalTokenFactory<'input>;
	type Type = dyn SubstraitPlanParserContext<'input> + 'input;
}

impl<'input, I, H> Deref for SubstraitPlanParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
    type Target = BaseParserType<'input,I>;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl<'input, I, H> DerefMut for SubstraitPlanParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

pub struct SubstraitPlanParserExt<'input>{
	_pd: PhantomData<&'input str>,
}

impl<'input> SubstraitPlanParserExt<'input>{
}
antlr_rust::tid! { SubstraitPlanParserExt<'a> }

impl<'input> TokenAware<'input> for SubstraitPlanParserExt<'input>{
	type TF = LocalTokenFactory<'input>;
}

impl<'input,I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>> ParserRecog<'input, BaseParserType<'input,I>> for SubstraitPlanParserExt<'input>{}

impl<'input,I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>> Actions<'input, BaseParserType<'input,I>> for SubstraitPlanParserExt<'input>{
	fn get_grammar_file_name(&self) -> & str{ "SubstraitPlanParser.g4"}

   	fn get_rule_names(&self) -> &[& str] {&ruleNames}

   	fn get_vocabulary(&self) -> &dyn Vocabulary { &**VOCABULARY }
	fn sempred(_localctx: Option<&(dyn SubstraitPlanParserContext<'input> + 'input)>, rule_index: isize, pred_index: isize,
			   recog:&mut BaseParserType<'input,I>
	)->bool{
		match rule_index {
					3 => SubstraitPlanParser::<'input,I,_>::pipeline_sempred(_localctx.and_then(|x|x.downcast_ref()), pred_index, recog),
					11 => SubstraitPlanParser::<'input,I,_>::expression_sempred(_localctx.and_then(|x|x.downcast_ref()), pred_index, recog),
			_ => true
		}
	}
}

impl<'input, I> SubstraitPlanParser<'input, I, DefaultErrorStrategy<'input,SubstraitPlanParserContextType>>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	fn pipeline_sempred(_localctx: Option<&PipelineContext<'input>>, pred_index:isize,
						recog:&mut <Self as Deref>::Target
		) -> bool {
		match pred_index {
				0=>{
					recog.precpred(None, 2)
				}
			_ => true
		}
	}
	fn expression_sempred(_localctx: Option<&ExpressionContext<'input>>, pred_index:isize,
						recog:&mut <Self as Deref>::Target
		) -> bool {
		match pred_index {
				1=>{
					recog.precpred(None, 5)
				}
				2=>{
					recog.precpred(None, 1)
				}
			_ => true
		}
	}
}
//------------------- plan ----------------
pub type PlanContextAll<'input> = PlanContext<'input>;


pub type PlanContext<'input> = BaseParserRuleContext<'input,PlanContextExt<'input>>;

#[derive(Clone)]
pub struct PlanContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SubstraitPlanParserContext<'input> for PlanContext<'input>{}

impl<'input,'a> Listenable<dyn SubstraitPlanParserListener<'input> + 'a> for PlanContext<'input>{
		fn enter(&self,listener: &mut (dyn SubstraitPlanParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_plan(self);
		}fn exit(&self,listener: &mut (dyn SubstraitPlanParserListener<'input> + 'a)) {
			listener.exit_plan(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for PlanContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SubstraitPlanParserContextType;
	fn get_rule_index(&self) -> usize { RULE_plan }
	//fn type_rule_index() -> usize where Self: Sized { RULE_plan }
}
antlr_rust::tid!{PlanContextExt<'a>}

impl<'input> PlanContextExt<'input>{
	fn new(parent: Option<Rc<dyn SubstraitPlanParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<PlanContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,PlanContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait PlanContextAttrs<'input>: SubstraitPlanParserContext<'input> + BorrowMut<PlanContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token EOF
/// Returns `None` if there is no child corresponding to token EOF
fn EOF(&self) -> Option<Rc<TerminalNode<'input,SubstraitPlanParserContextType>>> where Self:Sized{
	self.get_token(EOF, 0)
}
fn plan_detail_all(&self) ->  Vec<Rc<Plan_detailContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn plan_detail(&self, i: usize) -> Option<Rc<Plan_detailContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> PlanContextAttrs<'input> for PlanContext<'input>{}

impl<'input, I, H> SubstraitPlanParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn plan(&mut self,)
	-> Result<Rc<PlanContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = PlanContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 0, RULE_plan);
        let mut _localctx: Rc<PlanContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(79);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while (((_la) & !0x3f) == 0 && ((1usize << _la) & ((1usize << EXTENSION_SPACE) | (1usize << NAMED) | (1usize << SCHEMA) | (1usize << PIPELINES) | (1usize << FILTER) | (1usize << GROUPING) | (1usize << MEASURE) | (1usize << SORT) | (1usize << COUNT) | (1usize << TYPE) | (1usize << EMIT) | (1usize << ALL) | (1usize << ANY) | (1usize << COMPARISON))) != 0) || ((((_la - 34)) & !0x3f) == 0 && ((1usize << (_la - 34)) & ((1usize << (SOURCE - 34)) | (1usize << (ROOT - 34)) | (1usize << (NULLVAL - 34)))) != 0) || _la==IDENTIFIER {
				{
				{
				/*InvokeRule plan_detail*/
				recog.base.set_state(76);
				recog.plan_detail()?;

				}
				}
				recog.base.set_state(81);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(82);
			recog.base.match_token(EOF,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- plan_detail ----------------
pub type Plan_detailContextAll<'input> = Plan_detailContext<'input>;


pub type Plan_detailContext<'input> = BaseParserRuleContext<'input,Plan_detailContextExt<'input>>;

#[derive(Clone)]
pub struct Plan_detailContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SubstraitPlanParserContext<'input> for Plan_detailContext<'input>{}

impl<'input,'a> Listenable<dyn SubstraitPlanParserListener<'input> + 'a> for Plan_detailContext<'input>{
		fn enter(&self,listener: &mut (dyn SubstraitPlanParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_plan_detail(self);
		}fn exit(&self,listener: &mut (dyn SubstraitPlanParserListener<'input> + 'a)) {
			listener.exit_plan_detail(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for Plan_detailContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SubstraitPlanParserContextType;
	fn get_rule_index(&self) -> usize { RULE_plan_detail }
	//fn type_rule_index() -> usize where Self: Sized { RULE_plan_detail }
}
antlr_rust::tid!{Plan_detailContextExt<'a>}

impl<'input> Plan_detailContextExt<'input>{
	fn new(parent: Option<Rc<dyn SubstraitPlanParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Plan_detailContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Plan_detailContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Plan_detailContextAttrs<'input>: SubstraitPlanParserContext<'input> + BorrowMut<Plan_detailContextExt<'input>>{

fn pipelines(&self) -> Option<Rc<PipelinesContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn relation(&self) -> Option<Rc<RelationContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn root_relation(&self) -> Option<Rc<Root_relationContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn schema_definition(&self) -> Option<Rc<Schema_definitionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn source_definition(&self) -> Option<Rc<Source_definitionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn extensionspace(&self) -> Option<Rc<ExtensionspaceContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Plan_detailContextAttrs<'input> for Plan_detailContext<'input>{}

impl<'input, I, H> SubstraitPlanParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn plan_detail(&mut self,)
	-> Result<Rc<Plan_detailContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Plan_detailContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 2, RULE_plan_detail);
        let mut _localctx: Rc<Plan_detailContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(90);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(1,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule pipelines*/
					recog.base.set_state(84);
					recog.pipelines()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule relation*/
					recog.base.set_state(85);
					recog.relation()?;

					}
				}
			,
				3 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					/*InvokeRule root_relation*/
					recog.base.set_state(86);
					recog.root_relation()?;

					}
				}
			,
				4 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 4);
					recog.base.enter_outer_alt(None, 4);
					{
					/*InvokeRule schema_definition*/
					recog.base.set_state(87);
					recog.schema_definition()?;

					}
				}
			,
				5 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 5);
					recog.base.enter_outer_alt(None, 5);
					{
					/*InvokeRule source_definition*/
					recog.base.set_state(88);
					recog.source_definition()?;

					}
				}
			,
				6 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 6);
					recog.base.enter_outer_alt(None, 6);
					{
					/*InvokeRule extensionspace*/
					recog.base.set_state(89);
					recog.extensionspace()?;

					}
				}

				_ => {}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- pipelines ----------------
pub type PipelinesContextAll<'input> = PipelinesContext<'input>;


pub type PipelinesContext<'input> = BaseParserRuleContext<'input,PipelinesContextExt<'input>>;

#[derive(Clone)]
pub struct PipelinesContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SubstraitPlanParserContext<'input> for PipelinesContext<'input>{}

impl<'input,'a> Listenable<dyn SubstraitPlanParserListener<'input> + 'a> for PipelinesContext<'input>{
		fn enter(&self,listener: &mut (dyn SubstraitPlanParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_pipelines(self);
		}fn exit(&self,listener: &mut (dyn SubstraitPlanParserListener<'input> + 'a)) {
			listener.exit_pipelines(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for PipelinesContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SubstraitPlanParserContextType;
	fn get_rule_index(&self) -> usize { RULE_pipelines }
	//fn type_rule_index() -> usize where Self: Sized { RULE_pipelines }
}
antlr_rust::tid!{PipelinesContextExt<'a>}

impl<'input> PipelinesContextExt<'input>{
	fn new(parent: Option<Rc<dyn SubstraitPlanParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<PipelinesContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,PipelinesContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait PipelinesContextAttrs<'input>: SubstraitPlanParserContext<'input> + BorrowMut<PipelinesContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token PIPELINES
/// Returns `None` if there is no child corresponding to token PIPELINES
fn PIPELINES(&self) -> Option<Rc<TerminalNode<'input,SubstraitPlanParserContextType>>> where Self:Sized{
	self.get_token(PIPELINES, 0)
}
/// Retrieves first TerminalNode corresponding to token LEFTBRACE
/// Returns `None` if there is no child corresponding to token LEFTBRACE
fn LEFTBRACE(&self) -> Option<Rc<TerminalNode<'input,SubstraitPlanParserContextType>>> where Self:Sized{
	self.get_token(LEFTBRACE, 0)
}
/// Retrieves first TerminalNode corresponding to token RIGHTBRACE
/// Returns `None` if there is no child corresponding to token RIGHTBRACE
fn RIGHTBRACE(&self) -> Option<Rc<TerminalNode<'input,SubstraitPlanParserContextType>>> where Self:Sized{
	self.get_token(RIGHTBRACE, 0)
}
fn pipeline_all(&self) ->  Vec<Rc<PipelineContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn pipeline(&self, i: usize) -> Option<Rc<PipelineContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves all `TerminalNode`s corresponding to token SEMICOLON in current rule
fn SEMICOLON_all(&self) -> Vec<Rc<TerminalNode<'input,SubstraitPlanParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token SEMICOLON, starting from 0.
/// Returns `None` if number of children corresponding to token SEMICOLON is less or equal than `i`.
fn SEMICOLON(&self, i: usize) -> Option<Rc<TerminalNode<'input,SubstraitPlanParserContextType>>> where Self:Sized{
	self.get_token(SEMICOLON, i)
}

}

impl<'input> PipelinesContextAttrs<'input> for PipelinesContext<'input>{}

impl<'input, I, H> SubstraitPlanParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn pipelines(&mut self,)
	-> Result<Rc<PipelinesContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = PipelinesContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 4, RULE_pipelines);
        let mut _localctx: Rc<PipelinesContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(92);
			recog.base.match_token(PIPELINES,&mut recog.err_handler)?;

			recog.base.set_state(93);
			recog.base.match_token(LEFTBRACE,&mut recog.err_handler)?;

			recog.base.set_state(99);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while ((((_la - 5)) & !0x3f) == 0 && ((1usize << (_la - 5)) & ((1usize << (NAMED - 5)) | (1usize << (SCHEMA - 5)) | (1usize << (FILTER - 5)) | (1usize << (GROUPING - 5)) | (1usize << (MEASURE - 5)) | (1usize << (SORT - 5)) | (1usize << (COUNT - 5)) | (1usize << (TYPE - 5)) | (1usize << (EMIT - 5)) | (1usize << (ALL - 5)) | (1usize << (ANY - 5)) | (1usize << (COMPARISON - 5)) | (1usize << (SOURCE - 5)) | (1usize << (ROOT - 5)))) != 0) || _la==NULLVAL || _la==IDENTIFIER {
				{
				{
				/*InvokeRule pipeline*/
				recog.base.set_state(94);
				recog.pipeline_rec(0)?;

				recog.base.set_state(95);
				recog.base.match_token(SEMICOLON,&mut recog.err_handler)?;

				}
				}
				recog.base.set_state(101);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(102);
			recog.base.match_token(RIGHTBRACE,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- pipeline ----------------
pub type PipelineContextAll<'input> = PipelineContext<'input>;


pub type PipelineContext<'input> = BaseParserRuleContext<'input,PipelineContextExt<'input>>;

#[derive(Clone)]
pub struct PipelineContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SubstraitPlanParserContext<'input> for PipelineContext<'input>{}

impl<'input,'a> Listenable<dyn SubstraitPlanParserListener<'input> + 'a> for PipelineContext<'input>{
		fn enter(&self,listener: &mut (dyn SubstraitPlanParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_pipeline(self);
		}fn exit(&self,listener: &mut (dyn SubstraitPlanParserListener<'input> + 'a)) {
			listener.exit_pipeline(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for PipelineContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SubstraitPlanParserContextType;
	fn get_rule_index(&self) -> usize { RULE_pipeline }
	//fn type_rule_index() -> usize where Self: Sized { RULE_pipeline }
}
antlr_rust::tid!{PipelineContextExt<'a>}

impl<'input> PipelineContextExt<'input>{
	fn new(parent: Option<Rc<dyn SubstraitPlanParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<PipelineContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,PipelineContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait PipelineContextAttrs<'input>: SubstraitPlanParserContext<'input> + BorrowMut<PipelineContextExt<'input>>{

fn relation_ref(&self) -> Option<Rc<Relation_refContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn pipeline(&self) -> Option<Rc<PipelineContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token ARROW
/// Returns `None` if there is no child corresponding to token ARROW
fn ARROW(&self) -> Option<Rc<TerminalNode<'input,SubstraitPlanParserContextType>>> where Self:Sized{
	self.get_token(ARROW, 0)
}

}

impl<'input> PipelineContextAttrs<'input> for PipelineContext<'input>{}

impl<'input, I, H> SubstraitPlanParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn  pipeline(&mut self,)
	-> Result<Rc<PipelineContextAll<'input>>,ANTLRError> {
		self.pipeline_rec(0)
	}

	fn pipeline_rec(&mut self, _p: isize)
	-> Result<Rc<PipelineContextAll<'input>>,ANTLRError> {
		let recog = self;
		let _parentctx = recog.ctx.take();
		let _parentState = recog.base.get_state();
		let mut _localctx = PipelineContextExt::new(_parentctx.clone(), recog.base.get_state());
		recog.base.enter_recursion_rule(_localctx.clone(), 6, RULE_pipeline, _p);
	    let mut _localctx: Rc<PipelineContextAll> = _localctx;
        let mut _prevctx = _localctx.clone();
		let _startState = 6;
		let result: Result<(), ANTLRError> = (|| {
			let mut _alt: isize;
			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			{
			/*InvokeRule relation_ref*/
			recog.base.set_state(105);
			recog.relation_ref()?;

			}

			let tmp = recog.input.lt(-1).cloned();
			recog.ctx.as_ref().unwrap().set_stop(tmp);
			recog.base.set_state(112);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(3,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					recog.trigger_exit_rule_event();
					_prevctx = _localctx.clone();
					{
					{
					/*recRuleAltStartAction*/
					let mut tmp = PipelineContextExt::new(_parentctx.clone(), _parentState);
					recog.push_new_recursion_context(tmp.clone(), _startState, RULE_pipeline);
					_localctx = tmp;
					recog.base.set_state(107);
					if !({recog.precpred(None, 2)}) {
						Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 2)".to_owned()), None))?;
					}
					recog.base.set_state(108);
					recog.base.match_token(ARROW,&mut recog.err_handler)?;

					/*InvokeRule relation_ref*/
					recog.base.set_state(109);
					recog.relation_ref()?;

					}
					} 
				}
				recog.base.set_state(114);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(3,&mut recog.base)?;
			}
			}
			Ok(())
		})();
		match result {
		Ok(_) => {},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re)=>{
			//_localctx.exception = re;
			recog.err_handler.report_error(&mut recog.base, re);
	        recog.err_handler.recover(&mut recog.base, re)?;}
		}
		recog.base.unroll_recursion_context(_parentctx);

		Ok(_localctx)
	}
}
//------------------- relation ----------------
pub type RelationContextAll<'input> = RelationContext<'input>;


pub type RelationContext<'input> = BaseParserRuleContext<'input,RelationContextExt<'input>>;

#[derive(Clone)]
pub struct RelationContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SubstraitPlanParserContext<'input> for RelationContext<'input>{}

impl<'input,'a> Listenable<dyn SubstraitPlanParserListener<'input> + 'a> for RelationContext<'input>{
		fn enter(&self,listener: &mut (dyn SubstraitPlanParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_relation(self);
		}fn exit(&self,listener: &mut (dyn SubstraitPlanParserListener<'input> + 'a)) {
			listener.exit_relation(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for RelationContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SubstraitPlanParserContextType;
	fn get_rule_index(&self) -> usize { RULE_relation }
	//fn type_rule_index() -> usize where Self: Sized { RULE_relation }
}
antlr_rust::tid!{RelationContextExt<'a>}

impl<'input> RelationContextExt<'input>{
	fn new(parent: Option<Rc<dyn SubstraitPlanParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<RelationContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,RelationContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait RelationContextAttrs<'input>: SubstraitPlanParserContext<'input> + BorrowMut<RelationContextExt<'input>>{

fn relation_type(&self) -> Option<Rc<Relation_typeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token RELATION
/// Returns `None` if there is no child corresponding to token RELATION
fn RELATION(&self) -> Option<Rc<TerminalNode<'input,SubstraitPlanParserContextType>>> where Self:Sized{
	self.get_token(RELATION, 0)
}
fn relation_ref(&self) -> Option<Rc<Relation_refContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token LEFTBRACE
/// Returns `None` if there is no child corresponding to token LEFTBRACE
fn LEFTBRACE(&self) -> Option<Rc<TerminalNode<'input,SubstraitPlanParserContextType>>> where Self:Sized{
	self.get_token(LEFTBRACE, 0)
}
/// Retrieves first TerminalNode corresponding to token RIGHTBRACE
/// Returns `None` if there is no child corresponding to token RIGHTBRACE
fn RIGHTBRACE(&self) -> Option<Rc<TerminalNode<'input,SubstraitPlanParserContextType>>> where Self:Sized{
	self.get_token(RIGHTBRACE, 0)
}
fn relation_detail_all(&self) ->  Vec<Rc<Relation_detailContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn relation_detail(&self, i: usize) -> Option<Rc<Relation_detailContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> RelationContextAttrs<'input> for RelationContext<'input>{}

impl<'input, I, H> SubstraitPlanParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn relation(&mut self,)
	-> Result<Rc<RelationContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = RelationContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 8, RULE_relation);
        let mut _localctx: Rc<RelationContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule relation_type*/
			recog.base.set_state(115);
			recog.relation_type()?;

			recog.base.set_state(116);
			recog.base.match_token(RELATION,&mut recog.err_handler)?;

			/*InvokeRule relation_ref*/
			recog.base.set_state(117);
			recog.relation_ref()?;

			recog.base.set_state(118);
			recog.base.match_token(LEFTBRACE,&mut recog.err_handler)?;

			recog.base.set_state(122);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while ((((_la - 5)) & !0x3f) == 0 && ((1usize << (_la - 5)) & ((1usize << (NAMED - 5)) | (1usize << (SCHEMA - 5)) | (1usize << (COMMON - 5)) | (1usize << (BASE_SCHEMA - 5)) | (1usize << (FILTER - 5)) | (1usize << (EXPRESSION - 5)) | (1usize << (ADVANCED_EXTENSION - 5)) | (1usize << (GROUPING - 5)) | (1usize << (MEASURE - 5)) | (1usize << (SORT - 5)) | (1usize << (COUNT - 5)) | (1usize << (TYPE - 5)) | (1usize << (EMIT - 5)) | (1usize << (ALL - 5)) | (1usize << (ANY - 5)) | (1usize << (COMPARISON - 5)) | (1usize << (SOURCE - 5)) | (1usize << (ROOT - 5)))) != 0) || _la==NULLVAL || _la==IDENTIFIER {
				{
				{
				/*InvokeRule relation_detail*/
				recog.base.set_state(119);
				recog.relation_detail()?;

				}
				}
				recog.base.set_state(124);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(125);
			recog.base.match_token(RIGHTBRACE,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- root_relation ----------------
pub type Root_relationContextAll<'input> = Root_relationContext<'input>;


pub type Root_relationContext<'input> = BaseParserRuleContext<'input,Root_relationContextExt<'input>>;

#[derive(Clone)]
pub struct Root_relationContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SubstraitPlanParserContext<'input> for Root_relationContext<'input>{}

impl<'input,'a> Listenable<dyn SubstraitPlanParserListener<'input> + 'a> for Root_relationContext<'input>{
		fn enter(&self,listener: &mut (dyn SubstraitPlanParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_root_relation(self);
		}fn exit(&self,listener: &mut (dyn SubstraitPlanParserListener<'input> + 'a)) {
			listener.exit_root_relation(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for Root_relationContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SubstraitPlanParserContextType;
	fn get_rule_index(&self) -> usize { RULE_root_relation }
	//fn type_rule_index() -> usize where Self: Sized { RULE_root_relation }
}
antlr_rust::tid!{Root_relationContextExt<'a>}

impl<'input> Root_relationContextExt<'input>{
	fn new(parent: Option<Rc<dyn SubstraitPlanParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Root_relationContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Root_relationContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Root_relationContextAttrs<'input>: SubstraitPlanParserContext<'input> + BorrowMut<Root_relationContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token ROOT
/// Returns `None` if there is no child corresponding to token ROOT
fn ROOT(&self) -> Option<Rc<TerminalNode<'input,SubstraitPlanParserContextType>>> where Self:Sized{
	self.get_token(ROOT, 0)
}
/// Retrieves first TerminalNode corresponding to token LEFTBRACE
/// Returns `None` if there is no child corresponding to token LEFTBRACE
fn LEFTBRACE(&self) -> Option<Rc<TerminalNode<'input,SubstraitPlanParserContextType>>> where Self:Sized{
	self.get_token(LEFTBRACE, 0)
}
/// Retrieves first TerminalNode corresponding to token NAMES
/// Returns `None` if there is no child corresponding to token NAMES
fn NAMES(&self) -> Option<Rc<TerminalNode<'input,SubstraitPlanParserContextType>>> where Self:Sized{
	self.get_token(NAMES, 0)
}
/// Retrieves first TerminalNode corresponding to token EQUAL
/// Returns `None` if there is no child corresponding to token EQUAL
fn EQUAL(&self) -> Option<Rc<TerminalNode<'input,SubstraitPlanParserContextType>>> where Self:Sized{
	self.get_token(EQUAL, 0)
}
/// Retrieves first TerminalNode corresponding to token LEFTBRACKET
/// Returns `None` if there is no child corresponding to token LEFTBRACKET
fn LEFTBRACKET(&self) -> Option<Rc<TerminalNode<'input,SubstraitPlanParserContextType>>> where Self:Sized{
	self.get_token(LEFTBRACKET, 0)
}
fn id_all(&self) ->  Vec<Rc<IdContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn id(&self, i: usize) -> Option<Rc<IdContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves first TerminalNode corresponding to token RIGHTBRACKET
/// Returns `None` if there is no child corresponding to token RIGHTBRACKET
fn RIGHTBRACKET(&self) -> Option<Rc<TerminalNode<'input,SubstraitPlanParserContextType>>> where Self:Sized{
	self.get_token(RIGHTBRACKET, 0)
}
/// Retrieves first TerminalNode corresponding to token RIGHTBRACE
/// Returns `None` if there is no child corresponding to token RIGHTBRACE
fn RIGHTBRACE(&self) -> Option<Rc<TerminalNode<'input,SubstraitPlanParserContextType>>> where Self:Sized{
	self.get_token(RIGHTBRACE, 0)
}
/// Retrieves all `TerminalNode`s corresponding to token COMMA in current rule
fn COMMA_all(&self) -> Vec<Rc<TerminalNode<'input,SubstraitPlanParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token COMMA, starting from 0.
/// Returns `None` if number of children corresponding to token COMMA is less or equal than `i`.
fn COMMA(&self, i: usize) -> Option<Rc<TerminalNode<'input,SubstraitPlanParserContextType>>> where Self:Sized{
	self.get_token(COMMA, i)
}

}

impl<'input> Root_relationContextAttrs<'input> for Root_relationContext<'input>{}

impl<'input, I, H> SubstraitPlanParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn root_relation(&mut self,)
	-> Result<Rc<Root_relationContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Root_relationContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 10, RULE_root_relation);
        let mut _localctx: Rc<Root_relationContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			let mut _alt: isize;
			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(127);
			recog.base.match_token(ROOT,&mut recog.err_handler)?;

			recog.base.set_state(128);
			recog.base.match_token(LEFTBRACE,&mut recog.err_handler)?;

			recog.base.set_state(129);
			recog.base.match_token(NAMES,&mut recog.err_handler)?;

			recog.base.set_state(130);
			recog.base.match_token(EQUAL,&mut recog.err_handler)?;

			recog.base.set_state(131);
			recog.base.match_token(LEFTBRACKET,&mut recog.err_handler)?;

			/*InvokeRule id*/
			recog.base.set_state(132);
			recog.id()?;

			recog.base.set_state(137);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(5,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					{
					{
					recog.base.set_state(133);
					recog.base.match_token(COMMA,&mut recog.err_handler)?;

					/*InvokeRule id*/
					recog.base.set_state(134);
					recog.id()?;

					}
					} 
				}
				recog.base.set_state(139);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(5,&mut recog.base)?;
			}
			recog.base.set_state(141);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==COMMA {
				{
				recog.base.set_state(140);
				recog.base.match_token(COMMA,&mut recog.err_handler)?;

				}
			}

			recog.base.set_state(143);
			recog.base.match_token(RIGHTBRACKET,&mut recog.err_handler)?;

			recog.base.set_state(144);
			recog.base.match_token(RIGHTBRACE,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- relation_type ----------------
pub type Relation_typeContextAll<'input> = Relation_typeContext<'input>;


pub type Relation_typeContext<'input> = BaseParserRuleContext<'input,Relation_typeContextExt<'input>>;

#[derive(Clone)]
pub struct Relation_typeContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SubstraitPlanParserContext<'input> for Relation_typeContext<'input>{}

impl<'input,'a> Listenable<dyn SubstraitPlanParserListener<'input> + 'a> for Relation_typeContext<'input>{
		fn enter(&self,listener: &mut (dyn SubstraitPlanParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_relation_type(self);
		}fn exit(&self,listener: &mut (dyn SubstraitPlanParserListener<'input> + 'a)) {
			listener.exit_relation_type(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for Relation_typeContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SubstraitPlanParserContextType;
	fn get_rule_index(&self) -> usize { RULE_relation_type }
	//fn type_rule_index() -> usize where Self: Sized { RULE_relation_type }
}
antlr_rust::tid!{Relation_typeContextExt<'a>}

impl<'input> Relation_typeContextExt<'input>{
	fn new(parent: Option<Rc<dyn SubstraitPlanParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Relation_typeContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Relation_typeContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Relation_typeContextAttrs<'input>: SubstraitPlanParserContext<'input> + BorrowMut<Relation_typeContextExt<'input>>{

fn id(&self) -> Option<Rc<IdContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Relation_typeContextAttrs<'input> for Relation_typeContext<'input>{}

impl<'input, I, H> SubstraitPlanParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn relation_type(&mut self,)
	-> Result<Rc<Relation_typeContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Relation_typeContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 12, RULE_relation_type);
        let mut _localctx: Rc<Relation_typeContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule id*/
			recog.base.set_state(146);
			recog.id()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- relation_ref ----------------
pub type Relation_refContextAll<'input> = Relation_refContext<'input>;


pub type Relation_refContext<'input> = BaseParserRuleContext<'input,Relation_refContextExt<'input>>;

#[derive(Clone)]
pub struct Relation_refContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SubstraitPlanParserContext<'input> for Relation_refContext<'input>{}

impl<'input,'a> Listenable<dyn SubstraitPlanParserListener<'input> + 'a> for Relation_refContext<'input>{
		fn enter(&self,listener: &mut (dyn SubstraitPlanParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_relation_ref(self);
		}fn exit(&self,listener: &mut (dyn SubstraitPlanParserListener<'input> + 'a)) {
			listener.exit_relation_ref(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for Relation_refContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SubstraitPlanParserContextType;
	fn get_rule_index(&self) -> usize { RULE_relation_ref }
	//fn type_rule_index() -> usize where Self: Sized { RULE_relation_ref }
}
antlr_rust::tid!{Relation_refContextExt<'a>}

impl<'input> Relation_refContextExt<'input>{
	fn new(parent: Option<Rc<dyn SubstraitPlanParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Relation_refContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Relation_refContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Relation_refContextAttrs<'input>: SubstraitPlanParserContext<'input> + BorrowMut<Relation_refContextExt<'input>>{

fn id_all(&self) ->  Vec<Rc<IdContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn id(&self, i: usize) -> Option<Rc<IdContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves first TerminalNode corresponding to token LEFTPAREN
/// Returns `None` if there is no child corresponding to token LEFTPAREN
fn LEFTPAREN(&self) -> Option<Rc<TerminalNode<'input,SubstraitPlanParserContextType>>> where Self:Sized{
	self.get_token(LEFTPAREN, 0)
}
/// Retrieves first TerminalNode corresponding to token SCHEMA
/// Returns `None` if there is no child corresponding to token SCHEMA
fn SCHEMA(&self) -> Option<Rc<TerminalNode<'input,SubstraitPlanParserContextType>>> where Self:Sized{
	self.get_token(SCHEMA, 0)
}
/// Retrieves first TerminalNode corresponding to token RIGHTPAREN
/// Returns `None` if there is no child corresponding to token RIGHTPAREN
fn RIGHTPAREN(&self) -> Option<Rc<TerminalNode<'input,SubstraitPlanParserContextType>>> where Self:Sized{
	self.get_token(RIGHTPAREN, 0)
}

}

impl<'input> Relation_refContextAttrs<'input> for Relation_refContext<'input>{}

impl<'input, I, H> SubstraitPlanParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn relation_ref(&mut self,)
	-> Result<Rc<Relation_refContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Relation_refContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 14, RULE_relation_ref);
        let mut _localctx: Rc<Relation_refContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule id*/
			recog.base.set_state(148);
			recog.id()?;

			recog.base.set_state(154);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(7,&mut recog.base)? {
				x if x == 1=>{
					{
					recog.base.set_state(149);
					recog.base.match_token(LEFTPAREN,&mut recog.err_handler)?;

					recog.base.set_state(150);
					recog.base.match_token(SCHEMA,&mut recog.err_handler)?;

					/*InvokeRule id*/
					recog.base.set_state(151);
					recog.id()?;

					recog.base.set_state(152);
					recog.base.match_token(RIGHTPAREN,&mut recog.err_handler)?;

					}
				}

				_ => {}
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- relation_filter_behavior ----------------
pub type Relation_filter_behaviorContextAll<'input> = Relation_filter_behaviorContext<'input>;


pub type Relation_filter_behaviorContext<'input> = BaseParserRuleContext<'input,Relation_filter_behaviorContextExt<'input>>;

#[derive(Clone)]
pub struct Relation_filter_behaviorContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SubstraitPlanParserContext<'input> for Relation_filter_behaviorContext<'input>{}

impl<'input,'a> Listenable<dyn SubstraitPlanParserListener<'input> + 'a> for Relation_filter_behaviorContext<'input>{
		fn enter(&self,listener: &mut (dyn SubstraitPlanParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_relation_filter_behavior(self);
		}fn exit(&self,listener: &mut (dyn SubstraitPlanParserListener<'input> + 'a)) {
			listener.exit_relation_filter_behavior(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for Relation_filter_behaviorContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SubstraitPlanParserContextType;
	fn get_rule_index(&self) -> usize { RULE_relation_filter_behavior }
	//fn type_rule_index() -> usize where Self: Sized { RULE_relation_filter_behavior }
}
antlr_rust::tid!{Relation_filter_behaviorContextExt<'a>}

impl<'input> Relation_filter_behaviorContextExt<'input>{
	fn new(parent: Option<Rc<dyn SubstraitPlanParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Relation_filter_behaviorContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Relation_filter_behaviorContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Relation_filter_behaviorContextAttrs<'input>: SubstraitPlanParserContext<'input> + BorrowMut<Relation_filter_behaviorContextExt<'input>>{

fn id_all(&self) ->  Vec<Rc<IdContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn id(&self, i: usize) -> Option<Rc<IdContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves first TerminalNode corresponding to token MINUS
/// Returns `None` if there is no child corresponding to token MINUS
fn MINUS(&self) -> Option<Rc<TerminalNode<'input,SubstraitPlanParserContextType>>> where Self:Sized{
	self.get_token(MINUS, 0)
}

}

impl<'input> Relation_filter_behaviorContextAttrs<'input> for Relation_filter_behaviorContext<'input>{}

impl<'input, I, H> SubstraitPlanParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn relation_filter_behavior(&mut self,)
	-> Result<Rc<Relation_filter_behaviorContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Relation_filter_behaviorContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 16, RULE_relation_filter_behavior);
        let mut _localctx: Rc<Relation_filter_behaviorContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(164);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(8,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule id*/
					recog.base.set_state(156);
					recog.id()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule id*/
					recog.base.set_state(157);
					recog.id()?;

					recog.base.set_state(158);
					recog.base.match_token(MINUS,&mut recog.err_handler)?;

					/*InvokeRule id*/
					recog.base.set_state(159);
					recog.id()?;

					}
				}
			,
				3 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					/*InvokeRule id*/
					recog.base.set_state(161);
					recog.id()?;

					/*InvokeRule id*/
					recog.base.set_state(162);
					recog.id()?;

					}
				}

				_ => {}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- measure_detail ----------------
pub type Measure_detailContextAll<'input> = Measure_detailContext<'input>;


pub type Measure_detailContext<'input> = BaseParserRuleContext<'input,Measure_detailContextExt<'input>>;

#[derive(Clone)]
pub struct Measure_detailContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SubstraitPlanParserContext<'input> for Measure_detailContext<'input>{}

impl<'input,'a> Listenable<dyn SubstraitPlanParserListener<'input> + 'a> for Measure_detailContext<'input>{
		fn enter(&self,listener: &mut (dyn SubstraitPlanParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_measure_detail(self);
		}fn exit(&self,listener: &mut (dyn SubstraitPlanParserListener<'input> + 'a)) {
			listener.exit_measure_detail(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for Measure_detailContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SubstraitPlanParserContextType;
	fn get_rule_index(&self) -> usize { RULE_measure_detail }
	//fn type_rule_index() -> usize where Self: Sized { RULE_measure_detail }
}
antlr_rust::tid!{Measure_detailContextExt<'a>}

impl<'input> Measure_detailContextExt<'input>{
	fn new(parent: Option<Rc<dyn SubstraitPlanParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Measure_detailContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Measure_detailContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Measure_detailContextAttrs<'input>: SubstraitPlanParserContext<'input> + BorrowMut<Measure_detailContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token MEASURE
/// Returns `None` if there is no child corresponding to token MEASURE
fn MEASURE(&self) -> Option<Rc<TerminalNode<'input,SubstraitPlanParserContextType>>> where Self:Sized{
	self.get_token(MEASURE, 0)
}
fn expression(&self) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token SEMICOLON
/// Returns `None` if there is no child corresponding to token SEMICOLON
fn SEMICOLON(&self) -> Option<Rc<TerminalNode<'input,SubstraitPlanParserContextType>>> where Self:Sized{
	self.get_token(SEMICOLON, 0)
}
/// Retrieves first TerminalNode corresponding to token ARROW
/// Returns `None` if there is no child corresponding to token ARROW
fn ARROW(&self) -> Option<Rc<TerminalNode<'input,SubstraitPlanParserContextType>>> where Self:Sized{
	self.get_token(ARROW, 0)
}
fn literal_complex_type(&self) -> Option<Rc<Literal_complex_typeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token ATSIGN
/// Returns `None` if there is no child corresponding to token ATSIGN
fn ATSIGN(&self) -> Option<Rc<TerminalNode<'input,SubstraitPlanParserContextType>>> where Self:Sized{
	self.get_token(ATSIGN, 0)
}
fn id_all(&self) ->  Vec<Rc<IdContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn id(&self, i: usize) -> Option<Rc<IdContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves first TerminalNode corresponding to token NAMED
/// Returns `None` if there is no child corresponding to token NAMED
fn NAMED(&self) -> Option<Rc<TerminalNode<'input,SubstraitPlanParserContextType>>> where Self:Sized{
	self.get_token(NAMED, 0)
}
/// Retrieves first TerminalNode corresponding to token FILTER
/// Returns `None` if there is no child corresponding to token FILTER
fn FILTER(&self) -> Option<Rc<TerminalNode<'input,SubstraitPlanParserContextType>>> where Self:Sized{
	self.get_token(FILTER, 0)
}
/// Retrieves first TerminalNode corresponding to token INVOCATION
/// Returns `None` if there is no child corresponding to token INVOCATION
fn INVOCATION(&self) -> Option<Rc<TerminalNode<'input,SubstraitPlanParserContextType>>> where Self:Sized{
	self.get_token(INVOCATION, 0)
}
fn sort_field(&self) -> Option<Rc<Sort_fieldContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Measure_detailContextAttrs<'input> for Measure_detailContext<'input>{}

impl<'input, I, H> SubstraitPlanParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn measure_detail(&mut self,)
	-> Result<Rc<Measure_detailContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Measure_detailContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 18, RULE_measure_detail);
        let mut _localctx: Rc<Measure_detailContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(191);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 MEASURE 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					recog.base.set_state(166);
					recog.base.match_token(MEASURE,&mut recog.err_handler)?;

					/*InvokeRule expression*/
					recog.base.set_state(167);
					recog.expression_rec(0)?;

					recog.base.set_state(170);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==ARROW {
						{
						recog.base.set_state(168);
						recog.base.match_token(ARROW,&mut recog.err_handler)?;

						/*InvokeRule literal_complex_type*/
						recog.base.set_state(169);
						recog.literal_complex_type()?;

						}
					}

					recog.base.set_state(174);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==ATSIGN {
						{
						recog.base.set_state(172);
						recog.base.match_token(ATSIGN,&mut recog.err_handler)?;

						/*InvokeRule id*/
						recog.base.set_state(173);
						recog.id()?;

						}
					}

					recog.base.set_state(178);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==NAMED {
						{
						recog.base.set_state(176);
						recog.base.match_token(NAMED,&mut recog.err_handler)?;

						/*InvokeRule id*/
						recog.base.set_state(177);
						recog.id()?;

						}
					}

					recog.base.set_state(180);
					recog.base.match_token(SEMICOLON,&mut recog.err_handler)?;

					}
				}

			 FILTER 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(182);
					recog.base.match_token(FILTER,&mut recog.err_handler)?;

					/*InvokeRule expression*/
					recog.base.set_state(183);
					recog.expression_rec(0)?;

					recog.base.set_state(184);
					recog.base.match_token(SEMICOLON,&mut recog.err_handler)?;

					}
				}

			 INVOCATION 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					recog.base.set_state(186);
					recog.base.match_token(INVOCATION,&mut recog.err_handler)?;

					/*InvokeRule id*/
					recog.base.set_state(187);
					recog.id()?;

					recog.base.set_state(188);
					recog.base.match_token(SEMICOLON,&mut recog.err_handler)?;

					}
				}

			 SORT 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 4);
					recog.base.enter_outer_alt(None, 4);
					{
					/*InvokeRule sort_field*/
					recog.base.set_state(190);
					recog.sort_field()?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- relation_detail ----------------
#[derive(Debug)]
pub enum Relation_detailContextAll<'input>{
	RelationSourceReferenceContext(RelationSourceReferenceContext<'input>),
	RelationEmitContext(RelationEmitContext<'input>),
	RelationFilterContext(RelationFilterContext<'input>),
	RelationMeasureContext(RelationMeasureContext<'input>),
	RelationUsesSchemaContext(RelationUsesSchemaContext<'input>),
	RelationJoinTypeContext(RelationJoinTypeContext<'input>),
	RelationAdvancedExtensionContext(RelationAdvancedExtensionContext<'input>),
	RelationExpressionContext(RelationExpressionContext<'input>),
	RelationCountContext(RelationCountContext<'input>),
	RelationCommonContext(RelationCommonContext<'input>),
	RelationSortContext(RelationSortContext<'input>),
	RelationGroupingContext(RelationGroupingContext<'input>),
Error(Relation_detailContext<'input>)
}
antlr_rust::tid!{Relation_detailContextAll<'a>}

impl<'input> antlr_rust::parser_rule_context::DerefSeal for Relation_detailContextAll<'input>{}

impl<'input> SubstraitPlanParserParserContext<'input> for Relation_detailContextAll<'input>{}

impl<'input> Deref for Relation_detailContextAll<'input>{
	type Target = dyn Relation_detailContextAttrs<'input> + 'input;
	fn deref(&self) -> &Self::Target{
		use Relation_detailContextAll::*;
		match self{
			RelationSourceReferenceContext(inner) => inner,
			RelationEmitContext(inner) => inner,
			RelationFilterContext(inner) => inner,
			RelationMeasureContext(inner) => inner,
			RelationUsesSchemaContext(inner) => inner,
			RelationJoinTypeContext(inner) => inner,
			RelationAdvancedExtensionContext(inner) => inner,
			RelationExpressionContext(inner) => inner,
			RelationCountContext(inner) => inner,
			RelationCommonContext(inner) => inner,
			RelationSortContext(inner) => inner,
			RelationGroupingContext(inner) => inner,
Error(inner) => inner
		}
	}
}
impl<'input,'a> Listenable<dyn SubstraitPlanParserListener<'input> + 'a> for Relation_detailContextAll<'input>{
    fn enter(&self, listener: &mut (dyn SubstraitPlanParserListener<'input> + 'a)) { self.deref().enter(listener) }
    fn exit(&self, listener: &mut (dyn SubstraitPlanParserListener<'input> + 'a)) { self.deref().exit(listener) }
}



pub type Relation_detailContext<'input> = BaseParserRuleContext<'input,Relation_detailContextExt<'input>>;

#[derive(Clone)]
pub struct Relation_detailContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SubstraitPlanParserContext<'input> for Relation_detailContext<'input>{}

impl<'input,'a> Listenable<dyn SubstraitPlanParserListener<'input> + 'a> for Relation_detailContext<'input>{
}

impl<'input> CustomRuleContext<'input> for Relation_detailContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SubstraitPlanParserContextType;
	fn get_rule_index(&self) -> usize { RULE_relation_detail }
	//fn type_rule_index() -> usize where Self: Sized { RULE_relation_detail }
}
antlr_rust::tid!{Relation_detailContextExt<'a>}

impl<'input> Relation_detailContextExt<'input>{
	fn new(parent: Option<Rc<dyn SubstraitPlanParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Relation_detailContextAll<'input>> {
		Rc::new(
		Relation_detailContextAll::Error(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Relation_detailContextExt{
				ph:PhantomData
			}),
		)
		)
	}
}

pub trait Relation_detailContextAttrs<'input>: SubstraitPlanParserContext<'input> + BorrowMut<Relation_detailContextExt<'input>>{


}

impl<'input> Relation_detailContextAttrs<'input> for Relation_detailContext<'input>{}

pub type RelationSourceReferenceContext<'input> = BaseParserRuleContext<'input,RelationSourceReferenceContextExt<'input>>;

pub trait RelationSourceReferenceContextAttrs<'input>: SubstraitPlanParserContext<'input>{
	fn source_reference(&self) -> Option<Rc<Source_referenceContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
	/// Retrieves first TerminalNode corresponding to token SEMICOLON
	/// Returns `None` if there is no child corresponding to token SEMICOLON
	fn SEMICOLON(&self) -> Option<Rc<TerminalNode<'input,SubstraitPlanParserContextType>>> where Self:Sized{
		self.get_token(SEMICOLON, 0)
	}
}

impl<'input> RelationSourceReferenceContextAttrs<'input> for RelationSourceReferenceContext<'input>{}

pub struct RelationSourceReferenceContextExt<'input>{
	base:Relation_detailContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{RelationSourceReferenceContextExt<'a>}

impl<'input> SubstraitPlanParserContext<'input> for RelationSourceReferenceContext<'input>{}

impl<'input,'a> Listenable<dyn SubstraitPlanParserListener<'input> + 'a> for RelationSourceReferenceContext<'input>{
	fn enter(&self,listener: &mut (dyn SubstraitPlanParserListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_relationSourceReference(self);
	}
}

impl<'input> CustomRuleContext<'input> for RelationSourceReferenceContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SubstraitPlanParserContextType;
	fn get_rule_index(&self) -> usize { RULE_relation_detail }
	//fn type_rule_index() -> usize where Self: Sized { RULE_relation_detail }
}

impl<'input> Borrow<Relation_detailContextExt<'input>> for RelationSourceReferenceContext<'input>{
	fn borrow(&self) -> &Relation_detailContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<Relation_detailContextExt<'input>> for RelationSourceReferenceContext<'input>{
	fn borrow_mut(&mut self) -> &mut Relation_detailContextExt<'input> { &mut self.base }
}

impl<'input> Relation_detailContextAttrs<'input> for RelationSourceReferenceContext<'input> {}

impl<'input> RelationSourceReferenceContextExt<'input>{
	fn new(ctx: &dyn Relation_detailContextAttrs<'input>) -> Rc<Relation_detailContextAll<'input>>  {
		Rc::new(
			Relation_detailContextAll::RelationSourceReferenceContext(
				BaseParserRuleContext::copy_from(ctx,RelationSourceReferenceContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type RelationEmitContext<'input> = BaseParserRuleContext<'input,RelationEmitContextExt<'input>>;

pub trait RelationEmitContextAttrs<'input>: SubstraitPlanParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token EMIT
	/// Returns `None` if there is no child corresponding to token EMIT
	fn EMIT(&self) -> Option<Rc<TerminalNode<'input,SubstraitPlanParserContextType>>> where Self:Sized{
		self.get_token(EMIT, 0)
	}
	fn column_name(&self) -> Option<Rc<Column_nameContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
	/// Retrieves first TerminalNode corresponding to token SEMICOLON
	/// Returns `None` if there is no child corresponding to token SEMICOLON
	fn SEMICOLON(&self) -> Option<Rc<TerminalNode<'input,SubstraitPlanParserContextType>>> where Self:Sized{
		self.get_token(SEMICOLON, 0)
	}
}

impl<'input> RelationEmitContextAttrs<'input> for RelationEmitContext<'input>{}

pub struct RelationEmitContextExt<'input>{
	base:Relation_detailContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{RelationEmitContextExt<'a>}

impl<'input> SubstraitPlanParserContext<'input> for RelationEmitContext<'input>{}

impl<'input,'a> Listenable<dyn SubstraitPlanParserListener<'input> + 'a> for RelationEmitContext<'input>{
	fn enter(&self,listener: &mut (dyn SubstraitPlanParserListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_relationEmit(self);
	}
}

impl<'input> CustomRuleContext<'input> for RelationEmitContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SubstraitPlanParserContextType;
	fn get_rule_index(&self) -> usize { RULE_relation_detail }
	//fn type_rule_index() -> usize where Self: Sized { RULE_relation_detail }
}

impl<'input> Borrow<Relation_detailContextExt<'input>> for RelationEmitContext<'input>{
	fn borrow(&self) -> &Relation_detailContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<Relation_detailContextExt<'input>> for RelationEmitContext<'input>{
	fn borrow_mut(&mut self) -> &mut Relation_detailContextExt<'input> { &mut self.base }
}

impl<'input> Relation_detailContextAttrs<'input> for RelationEmitContext<'input> {}

impl<'input> RelationEmitContextExt<'input>{
	fn new(ctx: &dyn Relation_detailContextAttrs<'input>) -> Rc<Relation_detailContextAll<'input>>  {
		Rc::new(
			Relation_detailContextAll::RelationEmitContext(
				BaseParserRuleContext::copy_from(ctx,RelationEmitContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type RelationFilterContext<'input> = BaseParserRuleContext<'input,RelationFilterContextExt<'input>>;

pub trait RelationFilterContextAttrs<'input>: SubstraitPlanParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token FILTER
	/// Returns `None` if there is no child corresponding to token FILTER
	fn FILTER(&self) -> Option<Rc<TerminalNode<'input,SubstraitPlanParserContextType>>> where Self:Sized{
		self.get_token(FILTER, 0)
	}
	fn expression(&self) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
	/// Retrieves first TerminalNode corresponding to token SEMICOLON
	/// Returns `None` if there is no child corresponding to token SEMICOLON
	fn SEMICOLON(&self) -> Option<Rc<TerminalNode<'input,SubstraitPlanParserContextType>>> where Self:Sized{
		self.get_token(SEMICOLON, 0)
	}
	fn relation_filter_behavior(&self) -> Option<Rc<Relation_filter_behaviorContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> RelationFilterContextAttrs<'input> for RelationFilterContext<'input>{}

pub struct RelationFilterContextExt<'input>{
	base:Relation_detailContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{RelationFilterContextExt<'a>}

impl<'input> SubstraitPlanParserContext<'input> for RelationFilterContext<'input>{}

impl<'input,'a> Listenable<dyn SubstraitPlanParserListener<'input> + 'a> for RelationFilterContext<'input>{
	fn enter(&self,listener: &mut (dyn SubstraitPlanParserListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_relationFilter(self);
	}
}

impl<'input> CustomRuleContext<'input> for RelationFilterContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SubstraitPlanParserContextType;
	fn get_rule_index(&self) -> usize { RULE_relation_detail }
	//fn type_rule_index() -> usize where Self: Sized { RULE_relation_detail }
}

impl<'input> Borrow<Relation_detailContextExt<'input>> for RelationFilterContext<'input>{
	fn borrow(&self) -> &Relation_detailContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<Relation_detailContextExt<'input>> for RelationFilterContext<'input>{
	fn borrow_mut(&mut self) -> &mut Relation_detailContextExt<'input> { &mut self.base }
}

impl<'input> Relation_detailContextAttrs<'input> for RelationFilterContext<'input> {}

impl<'input> RelationFilterContextExt<'input>{
	fn new(ctx: &dyn Relation_detailContextAttrs<'input>) -> Rc<Relation_detailContextAll<'input>>  {
		Rc::new(
			Relation_detailContextAll::RelationFilterContext(
				BaseParserRuleContext::copy_from(ctx,RelationFilterContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type RelationMeasureContext<'input> = BaseParserRuleContext<'input,RelationMeasureContextExt<'input>>;

pub trait RelationMeasureContextAttrs<'input>: SubstraitPlanParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token MEASURE
	/// Returns `None` if there is no child corresponding to token MEASURE
	fn MEASURE(&self) -> Option<Rc<TerminalNode<'input,SubstraitPlanParserContextType>>> where Self:Sized{
		self.get_token(MEASURE, 0)
	}
	/// Retrieves first TerminalNode corresponding to token LEFTBRACE
	/// Returns `None` if there is no child corresponding to token LEFTBRACE
	fn LEFTBRACE(&self) -> Option<Rc<TerminalNode<'input,SubstraitPlanParserContextType>>> where Self:Sized{
		self.get_token(LEFTBRACE, 0)
	}
	/// Retrieves first TerminalNode corresponding to token RIGHTBRACE
	/// Returns `None` if there is no child corresponding to token RIGHTBRACE
	fn RIGHTBRACE(&self) -> Option<Rc<TerminalNode<'input,SubstraitPlanParserContextType>>> where Self:Sized{
		self.get_token(RIGHTBRACE, 0)
	}
	fn measure_detail_all(&self) ->  Vec<Rc<Measure_detailContextAll<'input>>> where Self:Sized{
		self.children_of_type()
	}
	fn measure_detail(&self, i: usize) -> Option<Rc<Measure_detailContextAll<'input>>> where Self:Sized{
		self.child_of_type(i)
	}
}

impl<'input> RelationMeasureContextAttrs<'input> for RelationMeasureContext<'input>{}

pub struct RelationMeasureContextExt<'input>{
	base:Relation_detailContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{RelationMeasureContextExt<'a>}

impl<'input> SubstraitPlanParserContext<'input> for RelationMeasureContext<'input>{}

impl<'input,'a> Listenable<dyn SubstraitPlanParserListener<'input> + 'a> for RelationMeasureContext<'input>{
	fn enter(&self,listener: &mut (dyn SubstraitPlanParserListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_relationMeasure(self);
	}
}

impl<'input> CustomRuleContext<'input> for RelationMeasureContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SubstraitPlanParserContextType;
	fn get_rule_index(&self) -> usize { RULE_relation_detail }
	//fn type_rule_index() -> usize where Self: Sized { RULE_relation_detail }
}

impl<'input> Borrow<Relation_detailContextExt<'input>> for RelationMeasureContext<'input>{
	fn borrow(&self) -> &Relation_detailContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<Relation_detailContextExt<'input>> for RelationMeasureContext<'input>{
	fn borrow_mut(&mut self) -> &mut Relation_detailContextExt<'input> { &mut self.base }
}

impl<'input> Relation_detailContextAttrs<'input> for RelationMeasureContext<'input> {}

impl<'input> RelationMeasureContextExt<'input>{
	fn new(ctx: &dyn Relation_detailContextAttrs<'input>) -> Rc<Relation_detailContextAll<'input>>  {
		Rc::new(
			Relation_detailContextAll::RelationMeasureContext(
				BaseParserRuleContext::copy_from(ctx,RelationMeasureContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type RelationUsesSchemaContext<'input> = BaseParserRuleContext<'input,RelationUsesSchemaContextExt<'input>>;

pub trait RelationUsesSchemaContextAttrs<'input>: SubstraitPlanParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token BASE_SCHEMA
	/// Returns `None` if there is no child corresponding to token BASE_SCHEMA
	fn BASE_SCHEMA(&self) -> Option<Rc<TerminalNode<'input,SubstraitPlanParserContextType>>> where Self:Sized{
		self.get_token(BASE_SCHEMA, 0)
	}
	fn id(&self) -> Option<Rc<IdContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
	/// Retrieves first TerminalNode corresponding to token SEMICOLON
	/// Returns `None` if there is no child corresponding to token SEMICOLON
	fn SEMICOLON(&self) -> Option<Rc<TerminalNode<'input,SubstraitPlanParserContextType>>> where Self:Sized{
		self.get_token(SEMICOLON, 0)
	}
}

impl<'input> RelationUsesSchemaContextAttrs<'input> for RelationUsesSchemaContext<'input>{}

pub struct RelationUsesSchemaContextExt<'input>{
	base:Relation_detailContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{RelationUsesSchemaContextExt<'a>}

impl<'input> SubstraitPlanParserContext<'input> for RelationUsesSchemaContext<'input>{}

impl<'input,'a> Listenable<dyn SubstraitPlanParserListener<'input> + 'a> for RelationUsesSchemaContext<'input>{
	fn enter(&self,listener: &mut (dyn SubstraitPlanParserListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_relationUsesSchema(self);
	}
}

impl<'input> CustomRuleContext<'input> for RelationUsesSchemaContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SubstraitPlanParserContextType;
	fn get_rule_index(&self) -> usize { RULE_relation_detail }
	//fn type_rule_index() -> usize where Self: Sized { RULE_relation_detail }
}

impl<'input> Borrow<Relation_detailContextExt<'input>> for RelationUsesSchemaContext<'input>{
	fn borrow(&self) -> &Relation_detailContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<Relation_detailContextExt<'input>> for RelationUsesSchemaContext<'input>{
	fn borrow_mut(&mut self) -> &mut Relation_detailContextExt<'input> { &mut self.base }
}

impl<'input> Relation_detailContextAttrs<'input> for RelationUsesSchemaContext<'input> {}

impl<'input> RelationUsesSchemaContextExt<'input>{
	fn new(ctx: &dyn Relation_detailContextAttrs<'input>) -> Rc<Relation_detailContextAll<'input>>  {
		Rc::new(
			Relation_detailContextAll::RelationUsesSchemaContext(
				BaseParserRuleContext::copy_from(ctx,RelationUsesSchemaContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type RelationJoinTypeContext<'input> = BaseParserRuleContext<'input,RelationJoinTypeContextExt<'input>>;

pub trait RelationJoinTypeContextAttrs<'input>: SubstraitPlanParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token TYPE
	/// Returns `None` if there is no child corresponding to token TYPE
	fn TYPE(&self) -> Option<Rc<TerminalNode<'input,SubstraitPlanParserContextType>>> where Self:Sized{
		self.get_token(TYPE, 0)
	}
	fn id(&self) -> Option<Rc<IdContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
	/// Retrieves first TerminalNode corresponding to token SEMICOLON
	/// Returns `None` if there is no child corresponding to token SEMICOLON
	fn SEMICOLON(&self) -> Option<Rc<TerminalNode<'input,SubstraitPlanParserContextType>>> where Self:Sized{
		self.get_token(SEMICOLON, 0)
	}
}

impl<'input> RelationJoinTypeContextAttrs<'input> for RelationJoinTypeContext<'input>{}

pub struct RelationJoinTypeContextExt<'input>{
	base:Relation_detailContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{RelationJoinTypeContextExt<'a>}

impl<'input> SubstraitPlanParserContext<'input> for RelationJoinTypeContext<'input>{}

impl<'input,'a> Listenable<dyn SubstraitPlanParserListener<'input> + 'a> for RelationJoinTypeContext<'input>{
	fn enter(&self,listener: &mut (dyn SubstraitPlanParserListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_relationJoinType(self);
	}
}

impl<'input> CustomRuleContext<'input> for RelationJoinTypeContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SubstraitPlanParserContextType;
	fn get_rule_index(&self) -> usize { RULE_relation_detail }
	//fn type_rule_index() -> usize where Self: Sized { RULE_relation_detail }
}

impl<'input> Borrow<Relation_detailContextExt<'input>> for RelationJoinTypeContext<'input>{
	fn borrow(&self) -> &Relation_detailContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<Relation_detailContextExt<'input>> for RelationJoinTypeContext<'input>{
	fn borrow_mut(&mut self) -> &mut Relation_detailContextExt<'input> { &mut self.base }
}

impl<'input> Relation_detailContextAttrs<'input> for RelationJoinTypeContext<'input> {}

impl<'input> RelationJoinTypeContextExt<'input>{
	fn new(ctx: &dyn Relation_detailContextAttrs<'input>) -> Rc<Relation_detailContextAll<'input>>  {
		Rc::new(
			Relation_detailContextAll::RelationJoinTypeContext(
				BaseParserRuleContext::copy_from(ctx,RelationJoinTypeContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type RelationAdvancedExtensionContext<'input> = BaseParserRuleContext<'input,RelationAdvancedExtensionContextExt<'input>>;

pub trait RelationAdvancedExtensionContextAttrs<'input>: SubstraitPlanParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token ADVANCED_EXTENSION
	/// Returns `None` if there is no child corresponding to token ADVANCED_EXTENSION
	fn ADVANCED_EXTENSION(&self) -> Option<Rc<TerminalNode<'input,SubstraitPlanParserContextType>>> where Self:Sized{
		self.get_token(ADVANCED_EXTENSION, 0)
	}
	/// Retrieves first TerminalNode corresponding to token SEMICOLON
	/// Returns `None` if there is no child corresponding to token SEMICOLON
	fn SEMICOLON(&self) -> Option<Rc<TerminalNode<'input,SubstraitPlanParserContextType>>> where Self:Sized{
		self.get_token(SEMICOLON, 0)
	}
}

impl<'input> RelationAdvancedExtensionContextAttrs<'input> for RelationAdvancedExtensionContext<'input>{}

pub struct RelationAdvancedExtensionContextExt<'input>{
	base:Relation_detailContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{RelationAdvancedExtensionContextExt<'a>}

impl<'input> SubstraitPlanParserContext<'input> for RelationAdvancedExtensionContext<'input>{}

impl<'input,'a> Listenable<dyn SubstraitPlanParserListener<'input> + 'a> for RelationAdvancedExtensionContext<'input>{
	fn enter(&self,listener: &mut (dyn SubstraitPlanParserListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_relationAdvancedExtension(self);
	}
}

impl<'input> CustomRuleContext<'input> for RelationAdvancedExtensionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SubstraitPlanParserContextType;
	fn get_rule_index(&self) -> usize { RULE_relation_detail }
	//fn type_rule_index() -> usize where Self: Sized { RULE_relation_detail }
}

impl<'input> Borrow<Relation_detailContextExt<'input>> for RelationAdvancedExtensionContext<'input>{
	fn borrow(&self) -> &Relation_detailContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<Relation_detailContextExt<'input>> for RelationAdvancedExtensionContext<'input>{
	fn borrow_mut(&mut self) -> &mut Relation_detailContextExt<'input> { &mut self.base }
}

impl<'input> Relation_detailContextAttrs<'input> for RelationAdvancedExtensionContext<'input> {}

impl<'input> RelationAdvancedExtensionContextExt<'input>{
	fn new(ctx: &dyn Relation_detailContextAttrs<'input>) -> Rc<Relation_detailContextAll<'input>>  {
		Rc::new(
			Relation_detailContextAll::RelationAdvancedExtensionContext(
				BaseParserRuleContext::copy_from(ctx,RelationAdvancedExtensionContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type RelationExpressionContext<'input> = BaseParserRuleContext<'input,RelationExpressionContextExt<'input>>;

pub trait RelationExpressionContextAttrs<'input>: SubstraitPlanParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token EXPRESSION
	/// Returns `None` if there is no child corresponding to token EXPRESSION
	fn EXPRESSION(&self) -> Option<Rc<TerminalNode<'input,SubstraitPlanParserContextType>>> where Self:Sized{
		self.get_token(EXPRESSION, 0)
	}
	fn expression(&self) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
	/// Retrieves first TerminalNode corresponding to token SEMICOLON
	/// Returns `None` if there is no child corresponding to token SEMICOLON
	fn SEMICOLON(&self) -> Option<Rc<TerminalNode<'input,SubstraitPlanParserContextType>>> where Self:Sized{
		self.get_token(SEMICOLON, 0)
	}
	/// Retrieves first TerminalNode corresponding to token NAMED
	/// Returns `None` if there is no child corresponding to token NAMED
	fn NAMED(&self) -> Option<Rc<TerminalNode<'input,SubstraitPlanParserContextType>>> where Self:Sized{
		self.get_token(NAMED, 0)
	}
	fn id(&self) -> Option<Rc<IdContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> RelationExpressionContextAttrs<'input> for RelationExpressionContext<'input>{}

pub struct RelationExpressionContextExt<'input>{
	base:Relation_detailContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{RelationExpressionContextExt<'a>}

impl<'input> SubstraitPlanParserContext<'input> for RelationExpressionContext<'input>{}

impl<'input,'a> Listenable<dyn SubstraitPlanParserListener<'input> + 'a> for RelationExpressionContext<'input>{
	fn enter(&self,listener: &mut (dyn SubstraitPlanParserListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_relationExpression(self);
	}
}

impl<'input> CustomRuleContext<'input> for RelationExpressionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SubstraitPlanParserContextType;
	fn get_rule_index(&self) -> usize { RULE_relation_detail }
	//fn type_rule_index() -> usize where Self: Sized { RULE_relation_detail }
}

impl<'input> Borrow<Relation_detailContextExt<'input>> for RelationExpressionContext<'input>{
	fn borrow(&self) -> &Relation_detailContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<Relation_detailContextExt<'input>> for RelationExpressionContext<'input>{
	fn borrow_mut(&mut self) -> &mut Relation_detailContextExt<'input> { &mut self.base }
}

impl<'input> Relation_detailContextAttrs<'input> for RelationExpressionContext<'input> {}

impl<'input> RelationExpressionContextExt<'input>{
	fn new(ctx: &dyn Relation_detailContextAttrs<'input>) -> Rc<Relation_detailContextAll<'input>>  {
		Rc::new(
			Relation_detailContextAll::RelationExpressionContext(
				BaseParserRuleContext::copy_from(ctx,RelationExpressionContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type RelationCountContext<'input> = BaseParserRuleContext<'input,RelationCountContextExt<'input>>;

pub trait RelationCountContextAttrs<'input>: SubstraitPlanParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token COUNT
	/// Returns `None` if there is no child corresponding to token COUNT
	fn COUNT(&self) -> Option<Rc<TerminalNode<'input,SubstraitPlanParserContextType>>> where Self:Sized{
		self.get_token(COUNT, 0)
	}
	/// Retrieves first TerminalNode corresponding to token NUMBER
	/// Returns `None` if there is no child corresponding to token NUMBER
	fn NUMBER(&self) -> Option<Rc<TerminalNode<'input,SubstraitPlanParserContextType>>> where Self:Sized{
		self.get_token(NUMBER, 0)
	}
	/// Retrieves first TerminalNode corresponding to token SEMICOLON
	/// Returns `None` if there is no child corresponding to token SEMICOLON
	fn SEMICOLON(&self) -> Option<Rc<TerminalNode<'input,SubstraitPlanParserContextType>>> where Self:Sized{
		self.get_token(SEMICOLON, 0)
	}
}

impl<'input> RelationCountContextAttrs<'input> for RelationCountContext<'input>{}

pub struct RelationCountContextExt<'input>{
	base:Relation_detailContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{RelationCountContextExt<'a>}

impl<'input> SubstraitPlanParserContext<'input> for RelationCountContext<'input>{}

impl<'input,'a> Listenable<dyn SubstraitPlanParserListener<'input> + 'a> for RelationCountContext<'input>{
	fn enter(&self,listener: &mut (dyn SubstraitPlanParserListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_relationCount(self);
	}
}

impl<'input> CustomRuleContext<'input> for RelationCountContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SubstraitPlanParserContextType;
	fn get_rule_index(&self) -> usize { RULE_relation_detail }
	//fn type_rule_index() -> usize where Self: Sized { RULE_relation_detail }
}

impl<'input> Borrow<Relation_detailContextExt<'input>> for RelationCountContext<'input>{
	fn borrow(&self) -> &Relation_detailContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<Relation_detailContextExt<'input>> for RelationCountContext<'input>{
	fn borrow_mut(&mut self) -> &mut Relation_detailContextExt<'input> { &mut self.base }
}

impl<'input> Relation_detailContextAttrs<'input> for RelationCountContext<'input> {}

impl<'input> RelationCountContextExt<'input>{
	fn new(ctx: &dyn Relation_detailContextAttrs<'input>) -> Rc<Relation_detailContextAll<'input>>  {
		Rc::new(
			Relation_detailContextAll::RelationCountContext(
				BaseParserRuleContext::copy_from(ctx,RelationCountContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type RelationCommonContext<'input> = BaseParserRuleContext<'input,RelationCommonContextExt<'input>>;

pub trait RelationCommonContextAttrs<'input>: SubstraitPlanParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token COMMON
	/// Returns `None` if there is no child corresponding to token COMMON
	fn COMMON(&self) -> Option<Rc<TerminalNode<'input,SubstraitPlanParserContextType>>> where Self:Sized{
		self.get_token(COMMON, 0)
	}
	/// Retrieves first TerminalNode corresponding to token SEMICOLON
	/// Returns `None` if there is no child corresponding to token SEMICOLON
	fn SEMICOLON(&self) -> Option<Rc<TerminalNode<'input,SubstraitPlanParserContextType>>> where Self:Sized{
		self.get_token(SEMICOLON, 0)
	}
}

impl<'input> RelationCommonContextAttrs<'input> for RelationCommonContext<'input>{}

pub struct RelationCommonContextExt<'input>{
	base:Relation_detailContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{RelationCommonContextExt<'a>}

impl<'input> SubstraitPlanParserContext<'input> for RelationCommonContext<'input>{}

impl<'input,'a> Listenable<dyn SubstraitPlanParserListener<'input> + 'a> for RelationCommonContext<'input>{
	fn enter(&self,listener: &mut (dyn SubstraitPlanParserListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_relationCommon(self);
	}
}

impl<'input> CustomRuleContext<'input> for RelationCommonContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SubstraitPlanParserContextType;
	fn get_rule_index(&self) -> usize { RULE_relation_detail }
	//fn type_rule_index() -> usize where Self: Sized { RULE_relation_detail }
}

impl<'input> Borrow<Relation_detailContextExt<'input>> for RelationCommonContext<'input>{
	fn borrow(&self) -> &Relation_detailContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<Relation_detailContextExt<'input>> for RelationCommonContext<'input>{
	fn borrow_mut(&mut self) -> &mut Relation_detailContextExt<'input> { &mut self.base }
}

impl<'input> Relation_detailContextAttrs<'input> for RelationCommonContext<'input> {}

impl<'input> RelationCommonContextExt<'input>{
	fn new(ctx: &dyn Relation_detailContextAttrs<'input>) -> Rc<Relation_detailContextAll<'input>>  {
		Rc::new(
			Relation_detailContextAll::RelationCommonContext(
				BaseParserRuleContext::copy_from(ctx,RelationCommonContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type RelationSortContext<'input> = BaseParserRuleContext<'input,RelationSortContextExt<'input>>;

pub trait RelationSortContextAttrs<'input>: SubstraitPlanParserContext<'input>{
	fn sort_field(&self) -> Option<Rc<Sort_fieldContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> RelationSortContextAttrs<'input> for RelationSortContext<'input>{}

pub struct RelationSortContextExt<'input>{
	base:Relation_detailContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{RelationSortContextExt<'a>}

impl<'input> SubstraitPlanParserContext<'input> for RelationSortContext<'input>{}

impl<'input,'a> Listenable<dyn SubstraitPlanParserListener<'input> + 'a> for RelationSortContext<'input>{
	fn enter(&self,listener: &mut (dyn SubstraitPlanParserListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_relationSort(self);
	}
}

impl<'input> CustomRuleContext<'input> for RelationSortContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SubstraitPlanParserContextType;
	fn get_rule_index(&self) -> usize { RULE_relation_detail }
	//fn type_rule_index() -> usize where Self: Sized { RULE_relation_detail }
}

impl<'input> Borrow<Relation_detailContextExt<'input>> for RelationSortContext<'input>{
	fn borrow(&self) -> &Relation_detailContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<Relation_detailContextExt<'input>> for RelationSortContext<'input>{
	fn borrow_mut(&mut self) -> &mut Relation_detailContextExt<'input> { &mut self.base }
}

impl<'input> Relation_detailContextAttrs<'input> for RelationSortContext<'input> {}

impl<'input> RelationSortContextExt<'input>{
	fn new(ctx: &dyn Relation_detailContextAttrs<'input>) -> Rc<Relation_detailContextAll<'input>>  {
		Rc::new(
			Relation_detailContextAll::RelationSortContext(
				BaseParserRuleContext::copy_from(ctx,RelationSortContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type RelationGroupingContext<'input> = BaseParserRuleContext<'input,RelationGroupingContextExt<'input>>;

pub trait RelationGroupingContextAttrs<'input>: SubstraitPlanParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token GROUPING
	/// Returns `None` if there is no child corresponding to token GROUPING
	fn GROUPING(&self) -> Option<Rc<TerminalNode<'input,SubstraitPlanParserContextType>>> where Self:Sized{
		self.get_token(GROUPING, 0)
	}
	fn expression(&self) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
	/// Retrieves first TerminalNode corresponding to token SEMICOLON
	/// Returns `None` if there is no child corresponding to token SEMICOLON
	fn SEMICOLON(&self) -> Option<Rc<TerminalNode<'input,SubstraitPlanParserContextType>>> where Self:Sized{
		self.get_token(SEMICOLON, 0)
	}
}

impl<'input> RelationGroupingContextAttrs<'input> for RelationGroupingContext<'input>{}

pub struct RelationGroupingContextExt<'input>{
	base:Relation_detailContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{RelationGroupingContextExt<'a>}

impl<'input> SubstraitPlanParserContext<'input> for RelationGroupingContext<'input>{}

impl<'input,'a> Listenable<dyn SubstraitPlanParserListener<'input> + 'a> for RelationGroupingContext<'input>{
	fn enter(&self,listener: &mut (dyn SubstraitPlanParserListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_relationGrouping(self);
	}
}

impl<'input> CustomRuleContext<'input> for RelationGroupingContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SubstraitPlanParserContextType;
	fn get_rule_index(&self) -> usize { RULE_relation_detail }
	//fn type_rule_index() -> usize where Self: Sized { RULE_relation_detail }
}

impl<'input> Borrow<Relation_detailContextExt<'input>> for RelationGroupingContext<'input>{
	fn borrow(&self) -> &Relation_detailContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<Relation_detailContextExt<'input>> for RelationGroupingContext<'input>{
	fn borrow_mut(&mut self) -> &mut Relation_detailContextExt<'input> { &mut self.base }
}

impl<'input> Relation_detailContextAttrs<'input> for RelationGroupingContext<'input> {}

impl<'input> RelationGroupingContextExt<'input>{
	fn new(ctx: &dyn Relation_detailContextAttrs<'input>) -> Rc<Relation_detailContextAll<'input>>  {
		Rc::new(
			Relation_detailContextAll::RelationGroupingContext(
				BaseParserRuleContext::copy_from(ctx,RelationGroupingContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

impl<'input, I, H> SubstraitPlanParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn relation_detail(&mut self,)
	-> Result<Rc<Relation_detailContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Relation_detailContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 20, RULE_relation_detail);
        let mut _localctx: Rc<Relation_detailContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(244);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(16,&mut recog.base)? {
				1 =>{
					let tmp = RelationCommonContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 1);
					_localctx = tmp;
					{
					recog.base.set_state(193);
					recog.base.match_token(COMMON,&mut recog.err_handler)?;

					recog.base.set_state(194);
					recog.base.match_token(SEMICOLON,&mut recog.err_handler)?;

					}
				}
			,
				2 =>{
					let tmp = RelationUsesSchemaContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 2);
					_localctx = tmp;
					{
					recog.base.set_state(195);
					recog.base.match_token(BASE_SCHEMA,&mut recog.err_handler)?;

					/*InvokeRule id*/
					recog.base.set_state(196);
					recog.id()?;

					recog.base.set_state(197);
					recog.base.match_token(SEMICOLON,&mut recog.err_handler)?;

					}
				}
			,
				3 =>{
					let tmp = RelationFilterContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 3);
					_localctx = tmp;
					{
					recog.base.set_state(200);
					recog.err_handler.sync(&mut recog.base)?;
					match  recog.interpreter.adaptive_predict(13,&mut recog.base)? {
						x if x == 1=>{
							{
							/*InvokeRule relation_filter_behavior*/
							recog.base.set_state(199);
							recog.relation_filter_behavior()?;

							}
						}

						_ => {}
					}
					recog.base.set_state(202);
					recog.base.match_token(FILTER,&mut recog.err_handler)?;

					/*InvokeRule expression*/
					recog.base.set_state(203);
					recog.expression_rec(0)?;

					recog.base.set_state(204);
					recog.base.match_token(SEMICOLON,&mut recog.err_handler)?;

					}
				}
			,
				4 =>{
					let tmp = RelationExpressionContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 4);
					_localctx = tmp;
					{
					recog.base.set_state(206);
					recog.base.match_token(EXPRESSION,&mut recog.err_handler)?;

					/*InvokeRule expression*/
					recog.base.set_state(207);
					recog.expression_rec(0)?;

					recog.base.set_state(210);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==NAMED {
						{
						recog.base.set_state(208);
						recog.base.match_token(NAMED,&mut recog.err_handler)?;

						/*InvokeRule id*/
						recog.base.set_state(209);
						recog.id()?;

						}
					}

					recog.base.set_state(212);
					recog.base.match_token(SEMICOLON,&mut recog.err_handler)?;

					}
				}
			,
				5 =>{
					let tmp = RelationAdvancedExtensionContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 5);
					_localctx = tmp;
					{
					recog.base.set_state(214);
					recog.base.match_token(ADVANCED_EXTENSION,&mut recog.err_handler)?;

					recog.base.set_state(215);
					recog.base.match_token(SEMICOLON,&mut recog.err_handler)?;

					}
				}
			,
				6 =>{
					let tmp = RelationSourceReferenceContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 6);
					_localctx = tmp;
					{
					/*InvokeRule source_reference*/
					recog.base.set_state(216);
					recog.source_reference()?;

					recog.base.set_state(217);
					recog.base.match_token(SEMICOLON,&mut recog.err_handler)?;

					}
				}
			,
				7 =>{
					let tmp = RelationGroupingContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 7);
					_localctx = tmp;
					{
					recog.base.set_state(219);
					recog.base.match_token(GROUPING,&mut recog.err_handler)?;

					/*InvokeRule expression*/
					recog.base.set_state(220);
					recog.expression_rec(0)?;

					recog.base.set_state(221);
					recog.base.match_token(SEMICOLON,&mut recog.err_handler)?;

					}
				}
			,
				8 =>{
					let tmp = RelationMeasureContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 8);
					_localctx = tmp;
					{
					recog.base.set_state(223);
					recog.base.match_token(MEASURE,&mut recog.err_handler)?;

					recog.base.set_state(224);
					recog.base.match_token(LEFTBRACE,&mut recog.err_handler)?;

					recog.base.set_state(228);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					while (((_la) & !0x3f) == 0 && ((1usize << _la) & ((1usize << FILTER) | (1usize << MEASURE) | (1usize << INVOCATION) | (1usize << SORT))) != 0) {
						{
						{
						/*InvokeRule measure_detail*/
						recog.base.set_state(225);
						recog.measure_detail()?;

						}
						}
						recog.base.set_state(230);
						recog.err_handler.sync(&mut recog.base)?;
						_la = recog.base.input.la(1);
					}
					recog.base.set_state(231);
					recog.base.match_token(RIGHTBRACE,&mut recog.err_handler)?;

					}
				}
			,
				9 =>{
					let tmp = RelationSortContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 9);
					_localctx = tmp;
					{
					/*InvokeRule sort_field*/
					recog.base.set_state(232);
					recog.sort_field()?;

					}
				}
			,
				10 =>{
					let tmp = RelationCountContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 10);
					_localctx = tmp;
					{
					recog.base.set_state(233);
					recog.base.match_token(COUNT,&mut recog.err_handler)?;

					recog.base.set_state(234);
					recog.base.match_token(NUMBER,&mut recog.err_handler)?;

					recog.base.set_state(235);
					recog.base.match_token(SEMICOLON,&mut recog.err_handler)?;

					}
				}
			,
				11 =>{
					let tmp = RelationJoinTypeContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 11);
					_localctx = tmp;
					{
					recog.base.set_state(236);
					recog.base.match_token(TYPE,&mut recog.err_handler)?;

					/*InvokeRule id*/
					recog.base.set_state(237);
					recog.id()?;

					recog.base.set_state(238);
					recog.base.match_token(SEMICOLON,&mut recog.err_handler)?;

					}
				}
			,
				12 =>{
					let tmp = RelationEmitContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 12);
					_localctx = tmp;
					{
					recog.base.set_state(240);
					recog.base.match_token(EMIT,&mut recog.err_handler)?;

					/*InvokeRule column_name*/
					recog.base.set_state(241);
					recog.column_name()?;

					recog.base.set_state(242);
					recog.base.match_token(SEMICOLON,&mut recog.err_handler)?;

					}
				}

				_ => {}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- expression ----------------
#[derive(Debug)]
pub enum ExpressionContextAll<'input>{
	ExpressionScalarSubqueryContext(ExpressionScalarSubqueryContext<'input>),
	ExpressionConstantContext(ExpressionConstantContext<'input>),
	ExpressionFunctionUseContext(ExpressionFunctionUseContext<'input>),
	ExpressionColumnContext(ExpressionColumnContext<'input>),
	ExpressionSetComparisonSubqueryContext(ExpressionSetComparisonSubqueryContext<'input>),
	ExpressionInPredicateSubqueryContext(ExpressionInPredicateSubqueryContext<'input>),
	ExpressionCastContext(ExpressionCastContext<'input>),
	ExpressionSetPredicateSubqueryContext(ExpressionSetPredicateSubqueryContext<'input>),
Error(ExpressionContext<'input>)
}
antlr_rust::tid!{ExpressionContextAll<'a>}

impl<'input> antlr_rust::parser_rule_context::DerefSeal for ExpressionContextAll<'input>{}

impl<'input> SubstraitPlanParserParserContext<'input> for ExpressionContextAll<'input>{}

impl<'input> Deref for ExpressionContextAll<'input>{
	type Target = dyn ExpressionContextAttrs<'input> + 'input;
	fn deref(&self) -> &Self::Target{
		use ExpressionContextAll::*;
		match self{
			ExpressionScalarSubqueryContext(inner) => inner,
			ExpressionConstantContext(inner) => inner,
			ExpressionFunctionUseContext(inner) => inner,
			ExpressionColumnContext(inner) => inner,
			ExpressionSetComparisonSubqueryContext(inner) => inner,
			ExpressionInPredicateSubqueryContext(inner) => inner,
			ExpressionCastContext(inner) => inner,
			ExpressionSetPredicateSubqueryContext(inner) => inner,
Error(inner) => inner
		}
	}
}
impl<'input,'a> Listenable<dyn SubstraitPlanParserListener<'input> + 'a> for ExpressionContextAll<'input>{
    fn enter(&self, listener: &mut (dyn SubstraitPlanParserListener<'input> + 'a)) { self.deref().enter(listener) }
    fn exit(&self, listener: &mut (dyn SubstraitPlanParserListener<'input> + 'a)) { self.deref().exit(listener) }
}



pub type ExpressionContext<'input> = BaseParserRuleContext<'input,ExpressionContextExt<'input>>;

#[derive(Clone)]
pub struct ExpressionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SubstraitPlanParserContext<'input> for ExpressionContext<'input>{}

impl<'input,'a> Listenable<dyn SubstraitPlanParserListener<'input> + 'a> for ExpressionContext<'input>{
}

impl<'input> CustomRuleContext<'input> for ExpressionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SubstraitPlanParserContextType;
	fn get_rule_index(&self) -> usize { RULE_expression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_expression }
}
antlr_rust::tid!{ExpressionContextExt<'a>}

impl<'input> ExpressionContextExt<'input>{
	fn new(parent: Option<Rc<dyn SubstraitPlanParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ExpressionContextAll<'input>> {
		Rc::new(
		ExpressionContextAll::Error(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ExpressionContextExt{
				ph:PhantomData
			}),
		)
		)
	}
}

pub trait ExpressionContextAttrs<'input>: SubstraitPlanParserContext<'input> + BorrowMut<ExpressionContextExt<'input>>{


}

impl<'input> ExpressionContextAttrs<'input> for ExpressionContext<'input>{}

pub type ExpressionScalarSubqueryContext<'input> = BaseParserRuleContext<'input,ExpressionScalarSubqueryContextExt<'input>>;

pub trait ExpressionScalarSubqueryContextAttrs<'input>: SubstraitPlanParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token SUBQUERY
	/// Returns `None` if there is no child corresponding to token SUBQUERY
	fn SUBQUERY(&self) -> Option<Rc<TerminalNode<'input,SubstraitPlanParserContextType>>> where Self:Sized{
		self.get_token(SUBQUERY, 0)
	}
	fn relation_ref(&self) -> Option<Rc<Relation_refContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> ExpressionScalarSubqueryContextAttrs<'input> for ExpressionScalarSubqueryContext<'input>{}

pub struct ExpressionScalarSubqueryContextExt<'input>{
	base:ExpressionContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{ExpressionScalarSubqueryContextExt<'a>}

impl<'input> SubstraitPlanParserContext<'input> for ExpressionScalarSubqueryContext<'input>{}

impl<'input,'a> Listenable<dyn SubstraitPlanParserListener<'input> + 'a> for ExpressionScalarSubqueryContext<'input>{
	fn enter(&self,listener: &mut (dyn SubstraitPlanParserListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_expressionScalarSubquery(self);
	}
}

impl<'input> CustomRuleContext<'input> for ExpressionScalarSubqueryContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SubstraitPlanParserContextType;
	fn get_rule_index(&self) -> usize { RULE_expression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_expression }
}

impl<'input> Borrow<ExpressionContextExt<'input>> for ExpressionScalarSubqueryContext<'input>{
	fn borrow(&self) -> &ExpressionContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<ExpressionContextExt<'input>> for ExpressionScalarSubqueryContext<'input>{
	fn borrow_mut(&mut self) -> &mut ExpressionContextExt<'input> { &mut self.base }
}

impl<'input> ExpressionContextAttrs<'input> for ExpressionScalarSubqueryContext<'input> {}

impl<'input> ExpressionScalarSubqueryContextExt<'input>{
	fn new(ctx: &dyn ExpressionContextAttrs<'input>) -> Rc<ExpressionContextAll<'input>>  {
		Rc::new(
			ExpressionContextAll::ExpressionScalarSubqueryContext(
				BaseParserRuleContext::copy_from(ctx,ExpressionScalarSubqueryContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type ExpressionConstantContext<'input> = BaseParserRuleContext<'input,ExpressionConstantContextExt<'input>>;

pub trait ExpressionConstantContextAttrs<'input>: SubstraitPlanParserContext<'input>{
	fn constant(&self) -> Option<Rc<ConstantContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> ExpressionConstantContextAttrs<'input> for ExpressionConstantContext<'input>{}

pub struct ExpressionConstantContextExt<'input>{
	base:ExpressionContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{ExpressionConstantContextExt<'a>}

impl<'input> SubstraitPlanParserContext<'input> for ExpressionConstantContext<'input>{}

impl<'input,'a> Listenable<dyn SubstraitPlanParserListener<'input> + 'a> for ExpressionConstantContext<'input>{
	fn enter(&self,listener: &mut (dyn SubstraitPlanParserListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_expressionConstant(self);
	}
}

impl<'input> CustomRuleContext<'input> for ExpressionConstantContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SubstraitPlanParserContextType;
	fn get_rule_index(&self) -> usize { RULE_expression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_expression }
}

impl<'input> Borrow<ExpressionContextExt<'input>> for ExpressionConstantContext<'input>{
	fn borrow(&self) -> &ExpressionContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<ExpressionContextExt<'input>> for ExpressionConstantContext<'input>{
	fn borrow_mut(&mut self) -> &mut ExpressionContextExt<'input> { &mut self.base }
}

impl<'input> ExpressionContextAttrs<'input> for ExpressionConstantContext<'input> {}

impl<'input> ExpressionConstantContextExt<'input>{
	fn new(ctx: &dyn ExpressionContextAttrs<'input>) -> Rc<ExpressionContextAll<'input>>  {
		Rc::new(
			ExpressionContextAll::ExpressionConstantContext(
				BaseParserRuleContext::copy_from(ctx,ExpressionConstantContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type ExpressionFunctionUseContext<'input> = BaseParserRuleContext<'input,ExpressionFunctionUseContextExt<'input>>;

pub trait ExpressionFunctionUseContextAttrs<'input>: SubstraitPlanParserContext<'input>{
	fn id(&self) -> Option<Rc<IdContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
	/// Retrieves first TerminalNode corresponding to token LEFTPAREN
	/// Returns `None` if there is no child corresponding to token LEFTPAREN
	fn LEFTPAREN(&self) -> Option<Rc<TerminalNode<'input,SubstraitPlanParserContextType>>> where Self:Sized{
		self.get_token(LEFTPAREN, 0)
	}
	/// Retrieves first TerminalNode corresponding to token RIGHTPAREN
	/// Returns `None` if there is no child corresponding to token RIGHTPAREN
	fn RIGHTPAREN(&self) -> Option<Rc<TerminalNode<'input,SubstraitPlanParserContextType>>> where Self:Sized{
		self.get_token(RIGHTPAREN, 0)
	}
	fn expression_all(&self) ->  Vec<Rc<ExpressionContextAll<'input>>> where Self:Sized{
		self.children_of_type()
	}
	fn expression(&self, i: usize) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
		self.child_of_type(i)
	}
	/// Retrieves first TerminalNode corresponding to token ARROW
	/// Returns `None` if there is no child corresponding to token ARROW
	fn ARROW(&self) -> Option<Rc<TerminalNode<'input,SubstraitPlanParserContextType>>> where Self:Sized{
		self.get_token(ARROW, 0)
	}
	fn literal_complex_type(&self) -> Option<Rc<Literal_complex_typeContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
	/// Retrieves all `TerminalNode`s corresponding to token COMMA in current rule
	fn COMMA_all(&self) -> Vec<Rc<TerminalNode<'input,SubstraitPlanParserContextType>>>  where Self:Sized{
		self.children_of_type()
	}
	/// Retrieves 'i's TerminalNode corresponding to token COMMA, starting from 0.
	/// Returns `None` if number of children corresponding to token COMMA is less or equal than `i`.
	fn COMMA(&self, i: usize) -> Option<Rc<TerminalNode<'input,SubstraitPlanParserContextType>>> where Self:Sized{
		self.get_token(COMMA, i)
	}
}

impl<'input> ExpressionFunctionUseContextAttrs<'input> for ExpressionFunctionUseContext<'input>{}

pub struct ExpressionFunctionUseContextExt<'input>{
	base:ExpressionContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{ExpressionFunctionUseContextExt<'a>}

impl<'input> SubstraitPlanParserContext<'input> for ExpressionFunctionUseContext<'input>{}

impl<'input,'a> Listenable<dyn SubstraitPlanParserListener<'input> + 'a> for ExpressionFunctionUseContext<'input>{
	fn enter(&self,listener: &mut (dyn SubstraitPlanParserListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_expressionFunctionUse(self);
	}
}

impl<'input> CustomRuleContext<'input> for ExpressionFunctionUseContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SubstraitPlanParserContextType;
	fn get_rule_index(&self) -> usize { RULE_expression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_expression }
}

impl<'input> Borrow<ExpressionContextExt<'input>> for ExpressionFunctionUseContext<'input>{
	fn borrow(&self) -> &ExpressionContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<ExpressionContextExt<'input>> for ExpressionFunctionUseContext<'input>{
	fn borrow_mut(&mut self) -> &mut ExpressionContextExt<'input> { &mut self.base }
}

impl<'input> ExpressionContextAttrs<'input> for ExpressionFunctionUseContext<'input> {}

impl<'input> ExpressionFunctionUseContextExt<'input>{
	fn new(ctx: &dyn ExpressionContextAttrs<'input>) -> Rc<ExpressionContextAll<'input>>  {
		Rc::new(
			ExpressionContextAll::ExpressionFunctionUseContext(
				BaseParserRuleContext::copy_from(ctx,ExpressionFunctionUseContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type ExpressionColumnContext<'input> = BaseParserRuleContext<'input,ExpressionColumnContextExt<'input>>;

pub trait ExpressionColumnContextAttrs<'input>: SubstraitPlanParserContext<'input>{
	fn column_name(&self) -> Option<Rc<Column_nameContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> ExpressionColumnContextAttrs<'input> for ExpressionColumnContext<'input>{}

pub struct ExpressionColumnContextExt<'input>{
	base:ExpressionContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{ExpressionColumnContextExt<'a>}

impl<'input> SubstraitPlanParserContext<'input> for ExpressionColumnContext<'input>{}

impl<'input,'a> Listenable<dyn SubstraitPlanParserListener<'input> + 'a> for ExpressionColumnContext<'input>{
	fn enter(&self,listener: &mut (dyn SubstraitPlanParserListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_expressionColumn(self);
	}
}

impl<'input> CustomRuleContext<'input> for ExpressionColumnContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SubstraitPlanParserContextType;
	fn get_rule_index(&self) -> usize { RULE_expression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_expression }
}

impl<'input> Borrow<ExpressionContextExt<'input>> for ExpressionColumnContext<'input>{
	fn borrow(&self) -> &ExpressionContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<ExpressionContextExt<'input>> for ExpressionColumnContext<'input>{
	fn borrow_mut(&mut self) -> &mut ExpressionContextExt<'input> { &mut self.base }
}

impl<'input> ExpressionContextAttrs<'input> for ExpressionColumnContext<'input> {}

impl<'input> ExpressionColumnContextExt<'input>{
	fn new(ctx: &dyn ExpressionContextAttrs<'input>) -> Rc<ExpressionContextAll<'input>>  {
		Rc::new(
			ExpressionContextAll::ExpressionColumnContext(
				BaseParserRuleContext::copy_from(ctx,ExpressionColumnContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type ExpressionSetComparisonSubqueryContext<'input> = BaseParserRuleContext<'input,ExpressionSetComparisonSubqueryContextExt<'input>>;

pub trait ExpressionSetComparisonSubqueryContextAttrs<'input>: SubstraitPlanParserContext<'input>{
	fn expression(&self) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
	/// Retrieves first TerminalNode corresponding to token COMPARISON
	/// Returns `None` if there is no child corresponding to token COMPARISON
	fn COMPARISON(&self) -> Option<Rc<TerminalNode<'input,SubstraitPlanParserContextType>>> where Self:Sized{
		self.get_token(COMPARISON, 0)
	}
	/// Retrieves first TerminalNode corresponding to token SUBQUERY
	/// Returns `None` if there is no child corresponding to token SUBQUERY
	fn SUBQUERY(&self) -> Option<Rc<TerminalNode<'input,SubstraitPlanParserContextType>>> where Self:Sized{
		self.get_token(SUBQUERY, 0)
	}
	fn relation_ref(&self) -> Option<Rc<Relation_refContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
	/// Retrieves first TerminalNode corresponding to token ALL
	/// Returns `None` if there is no child corresponding to token ALL
	fn ALL(&self) -> Option<Rc<TerminalNode<'input,SubstraitPlanParserContextType>>> where Self:Sized{
		self.get_token(ALL, 0)
	}
	/// Retrieves first TerminalNode corresponding to token ANY
	/// Returns `None` if there is no child corresponding to token ANY
	fn ANY(&self) -> Option<Rc<TerminalNode<'input,SubstraitPlanParserContextType>>> where Self:Sized{
		self.get_token(ANY, 0)
	}
}

impl<'input> ExpressionSetComparisonSubqueryContextAttrs<'input> for ExpressionSetComparisonSubqueryContext<'input>{}

pub struct ExpressionSetComparisonSubqueryContextExt<'input>{
	base:ExpressionContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{ExpressionSetComparisonSubqueryContextExt<'a>}

impl<'input> SubstraitPlanParserContext<'input> for ExpressionSetComparisonSubqueryContext<'input>{}

impl<'input,'a> Listenable<dyn SubstraitPlanParserListener<'input> + 'a> for ExpressionSetComparisonSubqueryContext<'input>{
	fn enter(&self,listener: &mut (dyn SubstraitPlanParserListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_expressionSetComparisonSubquery(self);
	}
}

impl<'input> CustomRuleContext<'input> for ExpressionSetComparisonSubqueryContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SubstraitPlanParserContextType;
	fn get_rule_index(&self) -> usize { RULE_expression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_expression }
}

impl<'input> Borrow<ExpressionContextExt<'input>> for ExpressionSetComparisonSubqueryContext<'input>{
	fn borrow(&self) -> &ExpressionContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<ExpressionContextExt<'input>> for ExpressionSetComparisonSubqueryContext<'input>{
	fn borrow_mut(&mut self) -> &mut ExpressionContextExt<'input> { &mut self.base }
}

impl<'input> ExpressionContextAttrs<'input> for ExpressionSetComparisonSubqueryContext<'input> {}

impl<'input> ExpressionSetComparisonSubqueryContextExt<'input>{
	fn new(ctx: &dyn ExpressionContextAttrs<'input>) -> Rc<ExpressionContextAll<'input>>  {
		Rc::new(
			ExpressionContextAll::ExpressionSetComparisonSubqueryContext(
				BaseParserRuleContext::copy_from(ctx,ExpressionSetComparisonSubqueryContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type ExpressionInPredicateSubqueryContext<'input> = BaseParserRuleContext<'input,ExpressionInPredicateSubqueryContextExt<'input>>;

pub trait ExpressionInPredicateSubqueryContextAttrs<'input>: SubstraitPlanParserContext<'input>{
	fn expression_list(&self) -> Option<Rc<Expression_listContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
	/// Retrieves first TerminalNode corresponding to token IN
	/// Returns `None` if there is no child corresponding to token IN
	fn IN(&self) -> Option<Rc<TerminalNode<'input,SubstraitPlanParserContextType>>> where Self:Sized{
		self.get_token(IN, 0)
	}
	/// Retrieves first TerminalNode corresponding to token SUBQUERY
	/// Returns `None` if there is no child corresponding to token SUBQUERY
	fn SUBQUERY(&self) -> Option<Rc<TerminalNode<'input,SubstraitPlanParserContextType>>> where Self:Sized{
		self.get_token(SUBQUERY, 0)
	}
	fn relation_ref(&self) -> Option<Rc<Relation_refContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> ExpressionInPredicateSubqueryContextAttrs<'input> for ExpressionInPredicateSubqueryContext<'input>{}

pub struct ExpressionInPredicateSubqueryContextExt<'input>{
	base:ExpressionContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{ExpressionInPredicateSubqueryContextExt<'a>}

impl<'input> SubstraitPlanParserContext<'input> for ExpressionInPredicateSubqueryContext<'input>{}

impl<'input,'a> Listenable<dyn SubstraitPlanParserListener<'input> + 'a> for ExpressionInPredicateSubqueryContext<'input>{
	fn enter(&self,listener: &mut (dyn SubstraitPlanParserListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_expressionInPredicateSubquery(self);
	}
}

impl<'input> CustomRuleContext<'input> for ExpressionInPredicateSubqueryContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SubstraitPlanParserContextType;
	fn get_rule_index(&self) -> usize { RULE_expression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_expression }
}

impl<'input> Borrow<ExpressionContextExt<'input>> for ExpressionInPredicateSubqueryContext<'input>{
	fn borrow(&self) -> &ExpressionContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<ExpressionContextExt<'input>> for ExpressionInPredicateSubqueryContext<'input>{
	fn borrow_mut(&mut self) -> &mut ExpressionContextExt<'input> { &mut self.base }
}

impl<'input> ExpressionContextAttrs<'input> for ExpressionInPredicateSubqueryContext<'input> {}

impl<'input> ExpressionInPredicateSubqueryContextExt<'input>{
	fn new(ctx: &dyn ExpressionContextAttrs<'input>) -> Rc<ExpressionContextAll<'input>>  {
		Rc::new(
			ExpressionContextAll::ExpressionInPredicateSubqueryContext(
				BaseParserRuleContext::copy_from(ctx,ExpressionInPredicateSubqueryContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type ExpressionCastContext<'input> = BaseParserRuleContext<'input,ExpressionCastContextExt<'input>>;

pub trait ExpressionCastContextAttrs<'input>: SubstraitPlanParserContext<'input>{
	fn expression(&self) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
	/// Retrieves first TerminalNode corresponding to token AS
	/// Returns `None` if there is no child corresponding to token AS
	fn AS(&self) -> Option<Rc<TerminalNode<'input,SubstraitPlanParserContextType>>> where Self:Sized{
		self.get_token(AS, 0)
	}
	fn literal_complex_type(&self) -> Option<Rc<Literal_complex_typeContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> ExpressionCastContextAttrs<'input> for ExpressionCastContext<'input>{}

pub struct ExpressionCastContextExt<'input>{
	base:ExpressionContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{ExpressionCastContextExt<'a>}

impl<'input> SubstraitPlanParserContext<'input> for ExpressionCastContext<'input>{}

impl<'input,'a> Listenable<dyn SubstraitPlanParserListener<'input> + 'a> for ExpressionCastContext<'input>{
	fn enter(&self,listener: &mut (dyn SubstraitPlanParserListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_expressionCast(self);
	}
}

impl<'input> CustomRuleContext<'input> for ExpressionCastContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SubstraitPlanParserContextType;
	fn get_rule_index(&self) -> usize { RULE_expression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_expression }
}

impl<'input> Borrow<ExpressionContextExt<'input>> for ExpressionCastContext<'input>{
	fn borrow(&self) -> &ExpressionContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<ExpressionContextExt<'input>> for ExpressionCastContext<'input>{
	fn borrow_mut(&mut self) -> &mut ExpressionContextExt<'input> { &mut self.base }
}

impl<'input> ExpressionContextAttrs<'input> for ExpressionCastContext<'input> {}

impl<'input> ExpressionCastContextExt<'input>{
	fn new(ctx: &dyn ExpressionContextAttrs<'input>) -> Rc<ExpressionContextAll<'input>>  {
		Rc::new(
			ExpressionContextAll::ExpressionCastContext(
				BaseParserRuleContext::copy_from(ctx,ExpressionCastContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type ExpressionSetPredicateSubqueryContext<'input> = BaseParserRuleContext<'input,ExpressionSetPredicateSubqueryContextExt<'input>>;

pub trait ExpressionSetPredicateSubqueryContextAttrs<'input>: SubstraitPlanParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token IN
	/// Returns `None` if there is no child corresponding to token IN
	fn IN(&self) -> Option<Rc<TerminalNode<'input,SubstraitPlanParserContextType>>> where Self:Sized{
		self.get_token(IN, 0)
	}
	/// Retrieves first TerminalNode corresponding to token SUBQUERY
	/// Returns `None` if there is no child corresponding to token SUBQUERY
	fn SUBQUERY(&self) -> Option<Rc<TerminalNode<'input,SubstraitPlanParserContextType>>> where Self:Sized{
		self.get_token(SUBQUERY, 0)
	}
	fn relation_ref(&self) -> Option<Rc<Relation_refContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
	/// Retrieves first TerminalNode corresponding to token UNIQUE
	/// Returns `None` if there is no child corresponding to token UNIQUE
	fn UNIQUE(&self) -> Option<Rc<TerminalNode<'input,SubstraitPlanParserContextType>>> where Self:Sized{
		self.get_token(UNIQUE, 0)
	}
	/// Retrieves first TerminalNode corresponding to token EXISTS
	/// Returns `None` if there is no child corresponding to token EXISTS
	fn EXISTS(&self) -> Option<Rc<TerminalNode<'input,SubstraitPlanParserContextType>>> where Self:Sized{
		self.get_token(EXISTS, 0)
	}
}

impl<'input> ExpressionSetPredicateSubqueryContextAttrs<'input> for ExpressionSetPredicateSubqueryContext<'input>{}

pub struct ExpressionSetPredicateSubqueryContextExt<'input>{
	base:ExpressionContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{ExpressionSetPredicateSubqueryContextExt<'a>}

impl<'input> SubstraitPlanParserContext<'input> for ExpressionSetPredicateSubqueryContext<'input>{}

impl<'input,'a> Listenable<dyn SubstraitPlanParserListener<'input> + 'a> for ExpressionSetPredicateSubqueryContext<'input>{
	fn enter(&self,listener: &mut (dyn SubstraitPlanParserListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_expressionSetPredicateSubquery(self);
	}
}

impl<'input> CustomRuleContext<'input> for ExpressionSetPredicateSubqueryContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SubstraitPlanParserContextType;
	fn get_rule_index(&self) -> usize { RULE_expression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_expression }
}

impl<'input> Borrow<ExpressionContextExt<'input>> for ExpressionSetPredicateSubqueryContext<'input>{
	fn borrow(&self) -> &ExpressionContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<ExpressionContextExt<'input>> for ExpressionSetPredicateSubqueryContext<'input>{
	fn borrow_mut(&mut self) -> &mut ExpressionContextExt<'input> { &mut self.base }
}

impl<'input> ExpressionContextAttrs<'input> for ExpressionSetPredicateSubqueryContext<'input> {}

impl<'input> ExpressionSetPredicateSubqueryContextExt<'input>{
	fn new(ctx: &dyn ExpressionContextAttrs<'input>) -> Rc<ExpressionContextAll<'input>>  {
		Rc::new(
			ExpressionContextAll::ExpressionSetPredicateSubqueryContext(
				BaseParserRuleContext::copy_from(ctx,ExpressionSetPredicateSubqueryContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

impl<'input, I, H> SubstraitPlanParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn  expression(&mut self,)
	-> Result<Rc<ExpressionContextAll<'input>>,ANTLRError> {
		self.expression_rec(0)
	}

	fn expression_rec(&mut self, _p: isize)
	-> Result<Rc<ExpressionContextAll<'input>>,ANTLRError> {
		let recog = self;
		let _parentctx = recog.ctx.take();
		let _parentState = recog.base.get_state();
		let mut _localctx = ExpressionContextExt::new(_parentctx.clone(), recog.base.get_state());
		recog.base.enter_recursion_rule(_localctx.clone(), 22, RULE_expression, _p);
	    let mut _localctx: Rc<ExpressionContextAll> = _localctx;
        let mut _prevctx = _localctx.clone();
		let _startState = 22;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {
			let mut _alt: isize;
			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(276);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(20,&mut recog.base)? {
				1 =>{
					{
					let mut tmp = ExpressionFunctionUseContextExt::new(&**_localctx);
					recog.ctx = Some(tmp.clone());
					_localctx = tmp;
					_prevctx = _localctx.clone();


					/*InvokeRule id*/
					recog.base.set_state(247);
					recog.id()?;

					recog.base.set_state(248);
					recog.base.match_token(LEFTPAREN,&mut recog.err_handler)?;

					recog.base.set_state(255);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					while ((((_la - 5)) & !0x3f) == 0 && ((1usize << (_la - 5)) & ((1usize << (NAMED - 5)) | (1usize << (SCHEMA - 5)) | (1usize << (FILTER - 5)) | (1usize << (GROUPING - 5)) | (1usize << (MEASURE - 5)) | (1usize << (SORT - 5)) | (1usize << (COUNT - 5)) | (1usize << (TYPE - 5)) | (1usize << (EMIT - 5)) | (1usize << (SUBQUERY - 5)) | (1usize << (EXISTS - 5)) | (1usize << (UNIQUE - 5)) | (1usize << (ALL - 5)) | (1usize << (ANY - 5)) | (1usize << (COMPARISON - 5)) | (1usize << (SOURCE - 5)) | (1usize << (ROOT - 5)))) != 0) || ((((_la - 47)) & !0x3f) == 0 && ((1usize << (_la - 47)) & ((1usize << (NULLVAL - 47)) | (1usize << (TRUEVAL - 47)) | (1usize << (FALSEVAL - 47)) | (1usize << (LEFTBRACE - 47)) | (1usize << (LEFTPAREN - 47)) | (1usize << (IDENTIFIER - 47)) | (1usize << (NUMBER - 47)) | (1usize << (STRING - 47)))) != 0) {
						{
						{
						/*InvokeRule expression*/
						recog.base.set_state(249);
						recog.expression_rec(0)?;

						recog.base.set_state(251);
						recog.err_handler.sync(&mut recog.base)?;
						_la = recog.base.input.la(1);
						if _la==COMMA {
							{
							recog.base.set_state(250);
							recog.base.match_token(COMMA,&mut recog.err_handler)?;

							}
						}

						}
						}
						recog.base.set_state(257);
						recog.err_handler.sync(&mut recog.base)?;
						_la = recog.base.input.la(1);
					}
					recog.base.set_state(258);
					recog.base.match_token(RIGHTPAREN,&mut recog.err_handler)?;

					recog.base.set_state(261);
					recog.err_handler.sync(&mut recog.base)?;
					match  recog.interpreter.adaptive_predict(19,&mut recog.base)? {
						x if x == 1=>{
							{
							recog.base.set_state(259);
							recog.base.match_token(ARROW,&mut recog.err_handler)?;

							/*InvokeRule literal_complex_type*/
							recog.base.set_state(260);
							recog.literal_complex_type()?;

							}
						}

						_ => {}
					}
					}
				}
			,
				2 =>{
					{
					let mut tmp = ExpressionConstantContextExt::new(&**_localctx);
					recog.ctx = Some(tmp.clone());
					_localctx = tmp;
					_prevctx = _localctx.clone();
					/*InvokeRule constant*/
					recog.base.set_state(263);
					recog.constant()?;

					}
				}
			,
				3 =>{
					{
					let mut tmp = ExpressionColumnContextExt::new(&**_localctx);
					recog.ctx = Some(tmp.clone());
					_localctx = tmp;
					_prevctx = _localctx.clone();
					/*InvokeRule column_name*/
					recog.base.set_state(264);
					recog.column_name()?;

					}
				}
			,
				4 =>{
					{
					let mut tmp = ExpressionScalarSubqueryContextExt::new(&**_localctx);
					recog.ctx = Some(tmp.clone());
					_localctx = tmp;
					_prevctx = _localctx.clone();
					recog.base.set_state(265);
					recog.base.match_token(SUBQUERY,&mut recog.err_handler)?;

					/*InvokeRule relation_ref*/
					recog.base.set_state(266);
					recog.relation_ref()?;

					}
				}
			,
				5 =>{
					{
					let mut tmp = ExpressionInPredicateSubqueryContextExt::new(&**_localctx);
					recog.ctx = Some(tmp.clone());
					_localctx = tmp;
					_prevctx = _localctx.clone();
					/*InvokeRule expression_list*/
					recog.base.set_state(267);
					recog.expression_list()?;

					recog.base.set_state(268);
					recog.base.match_token(IN,&mut recog.err_handler)?;

					recog.base.set_state(269);
					recog.base.match_token(SUBQUERY,&mut recog.err_handler)?;

					/*InvokeRule relation_ref*/
					recog.base.set_state(270);
					recog.relation_ref()?;

					}
				}
			,
				6 =>{
					{
					let mut tmp = ExpressionSetPredicateSubqueryContextExt::new(&**_localctx);
					recog.ctx = Some(tmp.clone());
					_localctx = tmp;
					_prevctx = _localctx.clone();
					recog.base.set_state(272);
					_la = recog.base.input.la(1);
					if { !(_la==EXISTS || _la==UNIQUE) } {
						recog.err_handler.recover_inline(&mut recog.base)?;

					}
					else {
						if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
						recog.err_handler.report_match(&mut recog.base);
						recog.base.consume(&mut recog.err_handler);
					}
					recog.base.set_state(273);
					recog.base.match_token(IN,&mut recog.err_handler)?;

					recog.base.set_state(274);
					recog.base.match_token(SUBQUERY,&mut recog.err_handler)?;

					/*InvokeRule relation_ref*/
					recog.base.set_state(275);
					recog.relation_ref()?;

					}
				}

				_ => {}
			}

			let tmp = recog.input.lt(-1).cloned();
			recog.ctx.as_ref().unwrap().set_stop(tmp);
			recog.base.set_state(288);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(22,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					recog.trigger_exit_rule_event();
					_prevctx = _localctx.clone();
					{
					recog.base.set_state(286);
					recog.err_handler.sync(&mut recog.base)?;
					match  recog.interpreter.adaptive_predict(21,&mut recog.base)? {
						1 =>{
							{
							/*recRuleLabeledAltStartAction*/
							let mut tmp = ExpressionCastContextExt::new(&**ExpressionContextExt::new(_parentctx.clone(), _parentState));
							recog.push_new_recursion_context(tmp.clone(), _startState, RULE_expression);
							_localctx = tmp;
							recog.base.set_state(278);
							if !({recog.precpred(None, 5)}) {
								Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 5)".to_owned()), None))?;
							}
							recog.base.set_state(279);
							recog.base.match_token(AS,&mut recog.err_handler)?;

							/*InvokeRule literal_complex_type*/
							recog.base.set_state(280);
							recog.literal_complex_type()?;

							}
						}
					,
						2 =>{
							{
							/*recRuleLabeledAltStartAction*/
							let mut tmp = ExpressionSetComparisonSubqueryContextExt::new(&**ExpressionContextExt::new(_parentctx.clone(), _parentState));
							recog.push_new_recursion_context(tmp.clone(), _startState, RULE_expression);
							_localctx = tmp;
							recog.base.set_state(281);
							if !({recog.precpred(None, 1)}) {
								Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 1)".to_owned()), None))?;
							}
							recog.base.set_state(282);
							recog.base.match_token(COMPARISON,&mut recog.err_handler)?;

							recog.base.set_state(283);
							_la = recog.base.input.la(1);
							if { !(_la==ALL || _la==ANY) } {
								recog.err_handler.recover_inline(&mut recog.base)?;

							}
							else {
								if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
								recog.err_handler.report_match(&mut recog.base);
								recog.base.consume(&mut recog.err_handler);
							}
							recog.base.set_state(284);
							recog.base.match_token(SUBQUERY,&mut recog.err_handler)?;

							/*InvokeRule relation_ref*/
							recog.base.set_state(285);
							recog.relation_ref()?;

							}
						}

						_ => {}
					}
					} 
				}
				recog.base.set_state(290);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(22,&mut recog.base)?;
			}
			}
			Ok(())
		})();
		match result {
		Ok(_) => {},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re)=>{
			//_localctx.exception = re;
			recog.err_handler.report_error(&mut recog.base, re);
	        recog.err_handler.recover(&mut recog.base, re)?;}
		}
		recog.base.unroll_recursion_context(_parentctx);

		Ok(_localctx)
	}
}
//------------------- expression_list ----------------
pub type Expression_listContextAll<'input> = Expression_listContext<'input>;


pub type Expression_listContext<'input> = BaseParserRuleContext<'input,Expression_listContextExt<'input>>;

#[derive(Clone)]
pub struct Expression_listContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SubstraitPlanParserContext<'input> for Expression_listContext<'input>{}

impl<'input,'a> Listenable<dyn SubstraitPlanParserListener<'input> + 'a> for Expression_listContext<'input>{
		fn enter(&self,listener: &mut (dyn SubstraitPlanParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_expression_list(self);
		}fn exit(&self,listener: &mut (dyn SubstraitPlanParserListener<'input> + 'a)) {
			listener.exit_expression_list(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for Expression_listContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SubstraitPlanParserContextType;
	fn get_rule_index(&self) -> usize { RULE_expression_list }
	//fn type_rule_index() -> usize where Self: Sized { RULE_expression_list }
}
antlr_rust::tid!{Expression_listContextExt<'a>}

impl<'input> Expression_listContextExt<'input>{
	fn new(parent: Option<Rc<dyn SubstraitPlanParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Expression_listContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Expression_listContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Expression_listContextAttrs<'input>: SubstraitPlanParserContext<'input> + BorrowMut<Expression_listContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token LEFTPAREN
/// Returns `None` if there is no child corresponding to token LEFTPAREN
fn LEFTPAREN(&self) -> Option<Rc<TerminalNode<'input,SubstraitPlanParserContextType>>> where Self:Sized{
	self.get_token(LEFTPAREN, 0)
}
fn expression_all(&self) ->  Vec<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn expression(&self, i: usize) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves first TerminalNode corresponding to token RIGHTPAREN
/// Returns `None` if there is no child corresponding to token RIGHTPAREN
fn RIGHTPAREN(&self) -> Option<Rc<TerminalNode<'input,SubstraitPlanParserContextType>>> where Self:Sized{
	self.get_token(RIGHTPAREN, 0)
}
/// Retrieves all `TerminalNode`s corresponding to token COMMA in current rule
fn COMMA_all(&self) -> Vec<Rc<TerminalNode<'input,SubstraitPlanParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token COMMA, starting from 0.
/// Returns `None` if number of children corresponding to token COMMA is less or equal than `i`.
fn COMMA(&self, i: usize) -> Option<Rc<TerminalNode<'input,SubstraitPlanParserContextType>>> where Self:Sized{
	self.get_token(COMMA, i)
}

}

impl<'input> Expression_listContextAttrs<'input> for Expression_listContext<'input>{}

impl<'input, I, H> SubstraitPlanParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn expression_list(&mut self,)
	-> Result<Rc<Expression_listContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Expression_listContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 24, RULE_expression_list);
        let mut _localctx: Rc<Expression_listContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(291);
			recog.base.match_token(LEFTPAREN,&mut recog.err_handler)?;

			/*InvokeRule expression*/
			recog.base.set_state(292);
			recog.expression_rec(0)?;

			recog.base.set_state(297);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==COMMA {
				{
				{
				recog.base.set_state(293);
				recog.base.match_token(COMMA,&mut recog.err_handler)?;

				/*InvokeRule expression*/
				recog.base.set_state(294);
				recog.expression_rec(0)?;

				}
				}
				recog.base.set_state(299);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(300);
			recog.base.match_token(RIGHTPAREN,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- constant ----------------
pub type ConstantContextAll<'input> = ConstantContext<'input>;


pub type ConstantContext<'input> = BaseParserRuleContext<'input,ConstantContextExt<'input>>;

#[derive(Clone)]
pub struct ConstantContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SubstraitPlanParserContext<'input> for ConstantContext<'input>{}

impl<'input,'a> Listenable<dyn SubstraitPlanParserListener<'input> + 'a> for ConstantContext<'input>{
		fn enter(&self,listener: &mut (dyn SubstraitPlanParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_constant(self);
		}fn exit(&self,listener: &mut (dyn SubstraitPlanParserListener<'input> + 'a)) {
			listener.exit_constant(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for ConstantContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SubstraitPlanParserContextType;
	fn get_rule_index(&self) -> usize { RULE_constant }
	//fn type_rule_index() -> usize where Self: Sized { RULE_constant }
}
antlr_rust::tid!{ConstantContextExt<'a>}

impl<'input> ConstantContextExt<'input>{
	fn new(parent: Option<Rc<dyn SubstraitPlanParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ConstantContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ConstantContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ConstantContextAttrs<'input>: SubstraitPlanParserContext<'input> + BorrowMut<ConstantContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token NUMBER
/// Returns `None` if there is no child corresponding to token NUMBER
fn NUMBER(&self) -> Option<Rc<TerminalNode<'input,SubstraitPlanParserContextType>>> where Self:Sized{
	self.get_token(NUMBER, 0)
}
/// Retrieves first TerminalNode corresponding to token UNDERSCORE
/// Returns `None` if there is no child corresponding to token UNDERSCORE
fn UNDERSCORE(&self) -> Option<Rc<TerminalNode<'input,SubstraitPlanParserContextType>>> where Self:Sized{
	self.get_token(UNDERSCORE, 0)
}
fn literal_basic_type(&self) -> Option<Rc<Literal_basic_typeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token STRING
/// Returns `None` if there is no child corresponding to token STRING
fn STRING(&self) -> Option<Rc<TerminalNode<'input,SubstraitPlanParserContextType>>> where Self:Sized{
	self.get_token(STRING, 0)
}
fn map_literal(&self) -> Option<Rc<Map_literalContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn literal_complex_type(&self) -> Option<Rc<Literal_complex_typeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn struct_literal(&self) -> Option<Rc<Struct_literalContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token NULLVAL
/// Returns `None` if there is no child corresponding to token NULLVAL
fn NULLVAL(&self) -> Option<Rc<TerminalNode<'input,SubstraitPlanParserContextType>>> where Self:Sized{
	self.get_token(NULLVAL, 0)
}
/// Retrieves first TerminalNode corresponding to token TRUEVAL
/// Returns `None` if there is no child corresponding to token TRUEVAL
fn TRUEVAL(&self) -> Option<Rc<TerminalNode<'input,SubstraitPlanParserContextType>>> where Self:Sized{
	self.get_token(TRUEVAL, 0)
}
/// Retrieves first TerminalNode corresponding to token FALSEVAL
/// Returns `None` if there is no child corresponding to token FALSEVAL
fn FALSEVAL(&self) -> Option<Rc<TerminalNode<'input,SubstraitPlanParserContextType>>> where Self:Sized{
	self.get_token(FALSEVAL, 0)
}

}

impl<'input> ConstantContextAttrs<'input> for ConstantContext<'input>{}

impl<'input, I, H> SubstraitPlanParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn constant(&mut self,)
	-> Result<Rc<ConstantContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ConstantContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 26, RULE_constant);
        let mut _localctx: Rc<ConstantContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(337);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(31,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					recog.base.set_state(302);
					recog.base.match_token(NUMBER,&mut recog.err_handler)?;

					recog.base.set_state(305);
					recog.err_handler.sync(&mut recog.base)?;
					match  recog.interpreter.adaptive_predict(24,&mut recog.base)? {
						x if x == 1=>{
							{
							recog.base.set_state(303);
							recog.base.match_token(UNDERSCORE,&mut recog.err_handler)?;

							/*InvokeRule literal_basic_type*/
							recog.base.set_state(304);
							recog.literal_basic_type()?;

							}
						}

						_ => {}
					}
					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(307);
					recog.base.match_token(STRING,&mut recog.err_handler)?;

					recog.base.set_state(310);
					recog.err_handler.sync(&mut recog.base)?;
					match  recog.interpreter.adaptive_predict(25,&mut recog.base)? {
						x if x == 1=>{
							{
							recog.base.set_state(308);
							recog.base.match_token(UNDERSCORE,&mut recog.err_handler)?;

							/*InvokeRule literal_basic_type*/
							recog.base.set_state(309);
							recog.literal_basic_type()?;

							}
						}

						_ => {}
					}
					}
				}
			,
				3 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					/*InvokeRule map_literal*/
					recog.base.set_state(312);
					recog.map_literal()?;

					recog.base.set_state(315);
					recog.err_handler.sync(&mut recog.base)?;
					match  recog.interpreter.adaptive_predict(26,&mut recog.base)? {
						x if x == 1=>{
							{
							recog.base.set_state(313);
							recog.base.match_token(UNDERSCORE,&mut recog.err_handler)?;

							/*InvokeRule literal_complex_type*/
							recog.base.set_state(314);
							recog.literal_complex_type()?;

							}
						}

						_ => {}
					}
					}
				}
			,
				4 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 4);
					recog.base.enter_outer_alt(None, 4);
					{
					/*InvokeRule struct_literal*/
					recog.base.set_state(317);
					recog.struct_literal()?;

					recog.base.set_state(320);
					recog.err_handler.sync(&mut recog.base)?;
					match  recog.interpreter.adaptive_predict(27,&mut recog.base)? {
						x if x == 1=>{
							{
							recog.base.set_state(318);
							recog.base.match_token(UNDERSCORE,&mut recog.err_handler)?;

							/*InvokeRule literal_complex_type*/
							recog.base.set_state(319);
							recog.literal_complex_type()?;

							}
						}

						_ => {}
					}
					}
				}
			,
				5 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 5);
					recog.base.enter_outer_alt(None, 5);
					{
					recog.base.set_state(322);
					recog.base.match_token(NULLVAL,&mut recog.err_handler)?;

					recog.base.set_state(325);
					recog.err_handler.sync(&mut recog.base)?;
					match  recog.interpreter.adaptive_predict(28,&mut recog.base)? {
						x if x == 1=>{
							{
							recog.base.set_state(323);
							recog.base.match_token(UNDERSCORE,&mut recog.err_handler)?;

							/*InvokeRule literal_complex_type*/
							recog.base.set_state(324);
							recog.literal_complex_type()?;

							}
						}

						_ => {}
					}
					}
				}
			,
				6 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 6);
					recog.base.enter_outer_alt(None, 6);
					{
					recog.base.set_state(327);
					recog.base.match_token(TRUEVAL,&mut recog.err_handler)?;

					recog.base.set_state(330);
					recog.err_handler.sync(&mut recog.base)?;
					match  recog.interpreter.adaptive_predict(29,&mut recog.base)? {
						x if x == 1=>{
							{
							recog.base.set_state(328);
							recog.base.match_token(UNDERSCORE,&mut recog.err_handler)?;

							/*InvokeRule literal_basic_type*/
							recog.base.set_state(329);
							recog.literal_basic_type()?;

							}
						}

						_ => {}
					}
					}
				}
			,
				7 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 7);
					recog.base.enter_outer_alt(None, 7);
					{
					recog.base.set_state(332);
					recog.base.match_token(FALSEVAL,&mut recog.err_handler)?;

					recog.base.set_state(335);
					recog.err_handler.sync(&mut recog.base)?;
					match  recog.interpreter.adaptive_predict(30,&mut recog.base)? {
						x if x == 1=>{
							{
							recog.base.set_state(333);
							recog.base.match_token(UNDERSCORE,&mut recog.err_handler)?;

							/*InvokeRule literal_basic_type*/
							recog.base.set_state(334);
							recog.literal_basic_type()?;

							}
						}

						_ => {}
					}
					}
				}

				_ => {}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- literal_basic_type ----------------
pub type Literal_basic_typeContextAll<'input> = Literal_basic_typeContext<'input>;


pub type Literal_basic_typeContext<'input> = BaseParserRuleContext<'input,Literal_basic_typeContextExt<'input>>;

#[derive(Clone)]
pub struct Literal_basic_typeContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SubstraitPlanParserContext<'input> for Literal_basic_typeContext<'input>{}

impl<'input,'a> Listenable<dyn SubstraitPlanParserListener<'input> + 'a> for Literal_basic_typeContext<'input>{
		fn enter(&self,listener: &mut (dyn SubstraitPlanParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_literal_basic_type(self);
		}fn exit(&self,listener: &mut (dyn SubstraitPlanParserListener<'input> + 'a)) {
			listener.exit_literal_basic_type(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for Literal_basic_typeContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SubstraitPlanParserContextType;
	fn get_rule_index(&self) -> usize { RULE_literal_basic_type }
	//fn type_rule_index() -> usize where Self: Sized { RULE_literal_basic_type }
}
antlr_rust::tid!{Literal_basic_typeContextExt<'a>}

impl<'input> Literal_basic_typeContextExt<'input>{
	fn new(parent: Option<Rc<dyn SubstraitPlanParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Literal_basic_typeContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Literal_basic_typeContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Literal_basic_typeContextAttrs<'input>: SubstraitPlanParserContext<'input> + BorrowMut<Literal_basic_typeContextExt<'input>>{

fn id(&self) -> Option<Rc<IdContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token QUESTIONMARK
/// Returns `None` if there is no child corresponding to token QUESTIONMARK
fn QUESTIONMARK(&self) -> Option<Rc<TerminalNode<'input,SubstraitPlanParserContextType>>> where Self:Sized{
	self.get_token(QUESTIONMARK, 0)
}
fn literal_specifier(&self) -> Option<Rc<Literal_specifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Literal_basic_typeContextAttrs<'input> for Literal_basic_typeContext<'input>{}

impl<'input, I, H> SubstraitPlanParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn literal_basic_type(&mut self,)
	-> Result<Rc<Literal_basic_typeContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Literal_basic_typeContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 28, RULE_literal_basic_type);
        let mut _localctx: Rc<Literal_basic_typeContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule id*/
			recog.base.set_state(339);
			recog.id()?;

			recog.base.set_state(341);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(32,&mut recog.base)? {
				x if x == 1=>{
					{
					recog.base.set_state(340);
					recog.base.match_token(QUESTIONMARK,&mut recog.err_handler)?;

					}
				}

				_ => {}
			}
			recog.base.set_state(344);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(33,&mut recog.base)? {
				x if x == 1=>{
					{
					/*InvokeRule literal_specifier*/
					recog.base.set_state(343);
					recog.literal_specifier()?;

					}
				}

				_ => {}
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- literal_complex_type ----------------
pub type Literal_complex_typeContextAll<'input> = Literal_complex_typeContext<'input>;


pub type Literal_complex_typeContext<'input> = BaseParserRuleContext<'input,Literal_complex_typeContextExt<'input>>;

#[derive(Clone)]
pub struct Literal_complex_typeContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SubstraitPlanParserContext<'input> for Literal_complex_typeContext<'input>{}

impl<'input,'a> Listenable<dyn SubstraitPlanParserListener<'input> + 'a> for Literal_complex_typeContext<'input>{
		fn enter(&self,listener: &mut (dyn SubstraitPlanParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_literal_complex_type(self);
		}fn exit(&self,listener: &mut (dyn SubstraitPlanParserListener<'input> + 'a)) {
			listener.exit_literal_complex_type(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for Literal_complex_typeContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SubstraitPlanParserContextType;
	fn get_rule_index(&self) -> usize { RULE_literal_complex_type }
	//fn type_rule_index() -> usize where Self: Sized { RULE_literal_complex_type }
}
antlr_rust::tid!{Literal_complex_typeContextExt<'a>}

impl<'input> Literal_complex_typeContextExt<'input>{
	fn new(parent: Option<Rc<dyn SubstraitPlanParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Literal_complex_typeContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Literal_complex_typeContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Literal_complex_typeContextAttrs<'input>: SubstraitPlanParserContext<'input> + BorrowMut<Literal_complex_typeContextExt<'input>>{

fn literal_basic_type(&self) -> Option<Rc<Literal_basic_typeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token LIST
/// Returns `None` if there is no child corresponding to token LIST
fn LIST(&self) -> Option<Rc<TerminalNode<'input,SubstraitPlanParserContextType>>> where Self:Sized{
	self.get_token(LIST, 0)
}
/// Retrieves first TerminalNode corresponding to token LEFTANGLEBRACKET
/// Returns `None` if there is no child corresponding to token LEFTANGLEBRACKET
fn LEFTANGLEBRACKET(&self) -> Option<Rc<TerminalNode<'input,SubstraitPlanParserContextType>>> where Self:Sized{
	self.get_token(LEFTANGLEBRACKET, 0)
}
/// Retrieves first TerminalNode corresponding to token RIGHTANGLEBRACKET
/// Returns `None` if there is no child corresponding to token RIGHTANGLEBRACKET
fn RIGHTANGLEBRACKET(&self) -> Option<Rc<TerminalNode<'input,SubstraitPlanParserContextType>>> where Self:Sized{
	self.get_token(RIGHTANGLEBRACKET, 0)
}
/// Retrieves first TerminalNode corresponding to token QUESTIONMARK
/// Returns `None` if there is no child corresponding to token QUESTIONMARK
fn QUESTIONMARK(&self) -> Option<Rc<TerminalNode<'input,SubstraitPlanParserContextType>>> where Self:Sized{
	self.get_token(QUESTIONMARK, 0)
}
fn literal_complex_type_all(&self) ->  Vec<Rc<Literal_complex_typeContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn literal_complex_type(&self, i: usize) -> Option<Rc<Literal_complex_typeContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves first TerminalNode corresponding to token MAP
/// Returns `None` if there is no child corresponding to token MAP
fn MAP(&self) -> Option<Rc<TerminalNode<'input,SubstraitPlanParserContextType>>> where Self:Sized{
	self.get_token(MAP, 0)
}
/// Retrieves all `TerminalNode`s corresponding to token COMMA in current rule
fn COMMA_all(&self) -> Vec<Rc<TerminalNode<'input,SubstraitPlanParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token COMMA, starting from 0.
/// Returns `None` if number of children corresponding to token COMMA is less or equal than `i`.
fn COMMA(&self, i: usize) -> Option<Rc<TerminalNode<'input,SubstraitPlanParserContextType>>> where Self:Sized{
	self.get_token(COMMA, i)
}
/// Retrieves first TerminalNode corresponding to token STRUCT
/// Returns `None` if there is no child corresponding to token STRUCT
fn STRUCT(&self) -> Option<Rc<TerminalNode<'input,SubstraitPlanParserContextType>>> where Self:Sized{
	self.get_token(STRUCT, 0)
}

}

impl<'input> Literal_complex_typeContextAttrs<'input> for Literal_complex_typeContext<'input>{}

impl<'input, I, H> SubstraitPlanParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn literal_complex_type(&mut self,)
	-> Result<Rc<Literal_complex_typeContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Literal_complex_typeContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 30, RULE_literal_complex_type);
        let mut _localctx: Rc<Literal_complex_typeContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(387);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 NAMED | SCHEMA | FILTER | GROUPING | MEASURE | SORT | COUNT | TYPE |
			 EMIT | ALL | ANY | COMPARISON | SOURCE | ROOT | NULLVAL | IDENTIFIER 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule literal_basic_type*/
					recog.base.set_state(346);
					recog.literal_basic_type()?;

					}
				}

			 LIST 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(347);
					recog.base.match_token(LIST,&mut recog.err_handler)?;

					recog.base.set_state(349);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==QUESTIONMARK {
						{
						recog.base.set_state(348);
						recog.base.match_token(QUESTIONMARK,&mut recog.err_handler)?;

						}
					}

					recog.base.set_state(351);
					recog.base.match_token(LEFTANGLEBRACKET,&mut recog.err_handler)?;

					recog.base.set_state(353);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if ((((_la - 5)) & !0x3f) == 0 && ((1usize << (_la - 5)) & ((1usize << (NAMED - 5)) | (1usize << (SCHEMA - 5)) | (1usize << (FILTER - 5)) | (1usize << (GROUPING - 5)) | (1usize << (MEASURE - 5)) | (1usize << (SORT - 5)) | (1usize << (COUNT - 5)) | (1usize << (TYPE - 5)) | (1usize << (EMIT - 5)) | (1usize << (ALL - 5)) | (1usize << (ANY - 5)) | (1usize << (COMPARISON - 5)) | (1usize << (SOURCE - 5)) | (1usize << (ROOT - 5)))) != 0) || ((((_la - 47)) & !0x3f) == 0 && ((1usize << (_la - 47)) & ((1usize << (NULLVAL - 47)) | (1usize << (LIST - 47)) | (1usize << (MAP - 47)) | (1usize << (STRUCT - 47)) | (1usize << (IDENTIFIER - 47)))) != 0) {
						{
						/*InvokeRule literal_complex_type*/
						recog.base.set_state(352);
						recog.literal_complex_type()?;

						}
					}

					recog.base.set_state(355);
					recog.base.match_token(RIGHTANGLEBRACKET,&mut recog.err_handler)?;

					}
				}

			 MAP 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					recog.base.set_state(356);
					recog.base.match_token(MAP,&mut recog.err_handler)?;

					recog.base.set_state(358);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==QUESTIONMARK {
						{
						recog.base.set_state(357);
						recog.base.match_token(QUESTIONMARK,&mut recog.err_handler)?;

						}
					}

					recog.base.set_state(360);
					recog.base.match_token(LEFTANGLEBRACKET,&mut recog.err_handler)?;

					recog.base.set_state(362);
					recog.err_handler.sync(&mut recog.base)?;
					match  recog.interpreter.adaptive_predict(37,&mut recog.base)? {
						x if x == 1=>{
							{
							/*InvokeRule literal_basic_type*/
							recog.base.set_state(361);
							recog.literal_basic_type()?;

							}
						}

						_ => {}
					}
					recog.base.set_state(365);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==COMMA {
						{
						recog.base.set_state(364);
						recog.base.match_token(COMMA,&mut recog.err_handler)?;

						}
					}

					recog.base.set_state(368);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if ((((_la - 5)) & !0x3f) == 0 && ((1usize << (_la - 5)) & ((1usize << (NAMED - 5)) | (1usize << (SCHEMA - 5)) | (1usize << (FILTER - 5)) | (1usize << (GROUPING - 5)) | (1usize << (MEASURE - 5)) | (1usize << (SORT - 5)) | (1usize << (COUNT - 5)) | (1usize << (TYPE - 5)) | (1usize << (EMIT - 5)) | (1usize << (ALL - 5)) | (1usize << (ANY - 5)) | (1usize << (COMPARISON - 5)) | (1usize << (SOURCE - 5)) | (1usize << (ROOT - 5)))) != 0) || ((((_la - 47)) & !0x3f) == 0 && ((1usize << (_la - 47)) & ((1usize << (NULLVAL - 47)) | (1usize << (LIST - 47)) | (1usize << (MAP - 47)) | (1usize << (STRUCT - 47)) | (1usize << (IDENTIFIER - 47)))) != 0) {
						{
						/*InvokeRule literal_complex_type*/
						recog.base.set_state(367);
						recog.literal_complex_type()?;

						}
					}

					recog.base.set_state(370);
					recog.base.match_token(RIGHTANGLEBRACKET,&mut recog.err_handler)?;

					}
				}

			 STRUCT 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 4);
					recog.base.enter_outer_alt(None, 4);
					{
					recog.base.set_state(371);
					recog.base.match_token(STRUCT,&mut recog.err_handler)?;

					recog.base.set_state(373);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==QUESTIONMARK {
						{
						recog.base.set_state(372);
						recog.base.match_token(QUESTIONMARK,&mut recog.err_handler)?;

						}
					}

					recog.base.set_state(375);
					recog.base.match_token(LEFTANGLEBRACKET,&mut recog.err_handler)?;

					recog.base.set_state(377);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if ((((_la - 5)) & !0x3f) == 0 && ((1usize << (_la - 5)) & ((1usize << (NAMED - 5)) | (1usize << (SCHEMA - 5)) | (1usize << (FILTER - 5)) | (1usize << (GROUPING - 5)) | (1usize << (MEASURE - 5)) | (1usize << (SORT - 5)) | (1usize << (COUNT - 5)) | (1usize << (TYPE - 5)) | (1usize << (EMIT - 5)) | (1usize << (ALL - 5)) | (1usize << (ANY - 5)) | (1usize << (COMPARISON - 5)) | (1usize << (SOURCE - 5)) | (1usize << (ROOT - 5)))) != 0) || ((((_la - 47)) & !0x3f) == 0 && ((1usize << (_la - 47)) & ((1usize << (NULLVAL - 47)) | (1usize << (LIST - 47)) | (1usize << (MAP - 47)) | (1usize << (STRUCT - 47)) | (1usize << (IDENTIFIER - 47)))) != 0) {
						{
						/*InvokeRule literal_complex_type*/
						recog.base.set_state(376);
						recog.literal_complex_type()?;

						}
					}

					recog.base.set_state(383);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					while _la==COMMA {
						{
						{
						recog.base.set_state(379);
						recog.base.match_token(COMMA,&mut recog.err_handler)?;

						/*InvokeRule literal_complex_type*/
						recog.base.set_state(380);
						recog.literal_complex_type()?;

						}
						}
						recog.base.set_state(385);
						recog.err_handler.sync(&mut recog.base)?;
						_la = recog.base.input.la(1);
					}
					recog.base.set_state(386);
					recog.base.match_token(RIGHTANGLEBRACKET,&mut recog.err_handler)?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- literal_specifier ----------------
pub type Literal_specifierContextAll<'input> = Literal_specifierContext<'input>;


pub type Literal_specifierContext<'input> = BaseParserRuleContext<'input,Literal_specifierContextExt<'input>>;

#[derive(Clone)]
pub struct Literal_specifierContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SubstraitPlanParserContext<'input> for Literal_specifierContext<'input>{}

impl<'input,'a> Listenable<dyn SubstraitPlanParserListener<'input> + 'a> for Literal_specifierContext<'input>{
		fn enter(&self,listener: &mut (dyn SubstraitPlanParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_literal_specifier(self);
		}fn exit(&self,listener: &mut (dyn SubstraitPlanParserListener<'input> + 'a)) {
			listener.exit_literal_specifier(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for Literal_specifierContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SubstraitPlanParserContextType;
	fn get_rule_index(&self) -> usize { RULE_literal_specifier }
	//fn type_rule_index() -> usize where Self: Sized { RULE_literal_specifier }
}
antlr_rust::tid!{Literal_specifierContextExt<'a>}

impl<'input> Literal_specifierContextExt<'input>{
	fn new(parent: Option<Rc<dyn SubstraitPlanParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Literal_specifierContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Literal_specifierContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Literal_specifierContextAttrs<'input>: SubstraitPlanParserContext<'input> + BorrowMut<Literal_specifierContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token LEFTANGLEBRACKET
/// Returns `None` if there is no child corresponding to token LEFTANGLEBRACKET
fn LEFTANGLEBRACKET(&self) -> Option<Rc<TerminalNode<'input,SubstraitPlanParserContextType>>> where Self:Sized{
	self.get_token(LEFTANGLEBRACKET, 0)
}
/// Retrieves all `TerminalNode`s corresponding to token NUMBER in current rule
fn NUMBER_all(&self) -> Vec<Rc<TerminalNode<'input,SubstraitPlanParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token NUMBER, starting from 0.
/// Returns `None` if number of children corresponding to token NUMBER is less or equal than `i`.
fn NUMBER(&self, i: usize) -> Option<Rc<TerminalNode<'input,SubstraitPlanParserContextType>>> where Self:Sized{
	self.get_token(NUMBER, i)
}
/// Retrieves first TerminalNode corresponding to token RIGHTANGLEBRACKET
/// Returns `None` if there is no child corresponding to token RIGHTANGLEBRACKET
fn RIGHTANGLEBRACKET(&self) -> Option<Rc<TerminalNode<'input,SubstraitPlanParserContextType>>> where Self:Sized{
	self.get_token(RIGHTANGLEBRACKET, 0)
}
/// Retrieves all `TerminalNode`s corresponding to token COMMA in current rule
fn COMMA_all(&self) -> Vec<Rc<TerminalNode<'input,SubstraitPlanParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token COMMA, starting from 0.
/// Returns `None` if number of children corresponding to token COMMA is less or equal than `i`.
fn COMMA(&self, i: usize) -> Option<Rc<TerminalNode<'input,SubstraitPlanParserContextType>>> where Self:Sized{
	self.get_token(COMMA, i)
}

}

impl<'input> Literal_specifierContextAttrs<'input> for Literal_specifierContext<'input>{}

impl<'input, I, H> SubstraitPlanParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn literal_specifier(&mut self,)
	-> Result<Rc<Literal_specifierContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Literal_specifierContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 32, RULE_literal_specifier);
        let mut _localctx: Rc<Literal_specifierContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(389);
			recog.base.match_token(LEFTANGLEBRACKET,&mut recog.err_handler)?;

			recog.base.set_state(390);
			recog.base.match_token(NUMBER,&mut recog.err_handler)?;

			recog.base.set_state(395);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==COMMA {
				{
				{
				recog.base.set_state(391);
				recog.base.match_token(COMMA,&mut recog.err_handler)?;

				recog.base.set_state(392);
				recog.base.match_token(NUMBER,&mut recog.err_handler)?;

				}
				}
				recog.base.set_state(397);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(398);
			recog.base.match_token(RIGHTANGLEBRACKET,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- map_literal ----------------
pub type Map_literalContextAll<'input> = Map_literalContext<'input>;


pub type Map_literalContext<'input> = BaseParserRuleContext<'input,Map_literalContextExt<'input>>;

#[derive(Clone)]
pub struct Map_literalContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SubstraitPlanParserContext<'input> for Map_literalContext<'input>{}

impl<'input,'a> Listenable<dyn SubstraitPlanParserListener<'input> + 'a> for Map_literalContext<'input>{
		fn enter(&self,listener: &mut (dyn SubstraitPlanParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_map_literal(self);
		}fn exit(&self,listener: &mut (dyn SubstraitPlanParserListener<'input> + 'a)) {
			listener.exit_map_literal(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for Map_literalContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SubstraitPlanParserContextType;
	fn get_rule_index(&self) -> usize { RULE_map_literal }
	//fn type_rule_index() -> usize where Self: Sized { RULE_map_literal }
}
antlr_rust::tid!{Map_literalContextExt<'a>}

impl<'input> Map_literalContextExt<'input>{
	fn new(parent: Option<Rc<dyn SubstraitPlanParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Map_literalContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Map_literalContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Map_literalContextAttrs<'input>: SubstraitPlanParserContext<'input> + BorrowMut<Map_literalContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token LEFTBRACE
/// Returns `None` if there is no child corresponding to token LEFTBRACE
fn LEFTBRACE(&self) -> Option<Rc<TerminalNode<'input,SubstraitPlanParserContextType>>> where Self:Sized{
	self.get_token(LEFTBRACE, 0)
}
fn map_literal_value_all(&self) ->  Vec<Rc<Map_literal_valueContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn map_literal_value(&self, i: usize) -> Option<Rc<Map_literal_valueContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves first TerminalNode corresponding to token RIGHTBRACE
/// Returns `None` if there is no child corresponding to token RIGHTBRACE
fn RIGHTBRACE(&self) -> Option<Rc<TerminalNode<'input,SubstraitPlanParserContextType>>> where Self:Sized{
	self.get_token(RIGHTBRACE, 0)
}
/// Retrieves all `TerminalNode`s corresponding to token COMMA in current rule
fn COMMA_all(&self) -> Vec<Rc<TerminalNode<'input,SubstraitPlanParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token COMMA, starting from 0.
/// Returns `None` if number of children corresponding to token COMMA is less or equal than `i`.
fn COMMA(&self, i: usize) -> Option<Rc<TerminalNode<'input,SubstraitPlanParserContextType>>> where Self:Sized{
	self.get_token(COMMA, i)
}

}

impl<'input> Map_literalContextAttrs<'input> for Map_literalContext<'input>{}

impl<'input, I, H> SubstraitPlanParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn map_literal(&mut self,)
	-> Result<Rc<Map_literalContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Map_literalContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 34, RULE_map_literal);
        let mut _localctx: Rc<Map_literalContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(413);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(46,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					recog.base.set_state(400);
					recog.base.match_token(LEFTBRACE,&mut recog.err_handler)?;

					/*InvokeRule map_literal_value*/
					recog.base.set_state(401);
					recog.map_literal_value()?;

					recog.base.set_state(406);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					while _la==COMMA {
						{
						{
						recog.base.set_state(402);
						recog.base.match_token(COMMA,&mut recog.err_handler)?;

						/*InvokeRule map_literal_value*/
						recog.base.set_state(403);
						recog.map_literal_value()?;

						}
						}
						recog.base.set_state(408);
						recog.err_handler.sync(&mut recog.base)?;
						_la = recog.base.input.la(1);
					}
					recog.base.set_state(409);
					recog.base.match_token(RIGHTBRACE,&mut recog.err_handler)?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(411);
					recog.base.match_token(LEFTBRACE,&mut recog.err_handler)?;

					recog.base.set_state(412);
					recog.base.match_token(RIGHTBRACE,&mut recog.err_handler)?;

					}
				}

				_ => {}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- map_literal_value ----------------
pub type Map_literal_valueContextAll<'input> = Map_literal_valueContext<'input>;


pub type Map_literal_valueContext<'input> = BaseParserRuleContext<'input,Map_literal_valueContextExt<'input>>;

#[derive(Clone)]
pub struct Map_literal_valueContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SubstraitPlanParserContext<'input> for Map_literal_valueContext<'input>{}

impl<'input,'a> Listenable<dyn SubstraitPlanParserListener<'input> + 'a> for Map_literal_valueContext<'input>{
		fn enter(&self,listener: &mut (dyn SubstraitPlanParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_map_literal_value(self);
		}fn exit(&self,listener: &mut (dyn SubstraitPlanParserListener<'input> + 'a)) {
			listener.exit_map_literal_value(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for Map_literal_valueContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SubstraitPlanParserContextType;
	fn get_rule_index(&self) -> usize { RULE_map_literal_value }
	//fn type_rule_index() -> usize where Self: Sized { RULE_map_literal_value }
}
antlr_rust::tid!{Map_literal_valueContextExt<'a>}

impl<'input> Map_literal_valueContextExt<'input>{
	fn new(parent: Option<Rc<dyn SubstraitPlanParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Map_literal_valueContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Map_literal_valueContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Map_literal_valueContextAttrs<'input>: SubstraitPlanParserContext<'input> + BorrowMut<Map_literal_valueContextExt<'input>>{

fn constant_all(&self) ->  Vec<Rc<ConstantContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn constant(&self, i: usize) -> Option<Rc<ConstantContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves first TerminalNode corresponding to token COLON
/// Returns `None` if there is no child corresponding to token COLON
fn COLON(&self) -> Option<Rc<TerminalNode<'input,SubstraitPlanParserContextType>>> where Self:Sized{
	self.get_token(COLON, 0)
}

}

impl<'input> Map_literal_valueContextAttrs<'input> for Map_literal_valueContext<'input>{}

impl<'input, I, H> SubstraitPlanParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn map_literal_value(&mut self,)
	-> Result<Rc<Map_literal_valueContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Map_literal_valueContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 36, RULE_map_literal_value);
        let mut _localctx: Rc<Map_literal_valueContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule constant*/
			recog.base.set_state(415);
			recog.constant()?;

			recog.base.set_state(416);
			recog.base.match_token(COLON,&mut recog.err_handler)?;

			/*InvokeRule constant*/
			recog.base.set_state(417);
			recog.constant()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- struct_literal ----------------
pub type Struct_literalContextAll<'input> = Struct_literalContext<'input>;


pub type Struct_literalContext<'input> = BaseParserRuleContext<'input,Struct_literalContextExt<'input>>;

#[derive(Clone)]
pub struct Struct_literalContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SubstraitPlanParserContext<'input> for Struct_literalContext<'input>{}

impl<'input,'a> Listenable<dyn SubstraitPlanParserListener<'input> + 'a> for Struct_literalContext<'input>{
		fn enter(&self,listener: &mut (dyn SubstraitPlanParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_struct_literal(self);
		}fn exit(&self,listener: &mut (dyn SubstraitPlanParserListener<'input> + 'a)) {
			listener.exit_struct_literal(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for Struct_literalContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SubstraitPlanParserContextType;
	fn get_rule_index(&self) -> usize { RULE_struct_literal }
	//fn type_rule_index() -> usize where Self: Sized { RULE_struct_literal }
}
antlr_rust::tid!{Struct_literalContextExt<'a>}

impl<'input> Struct_literalContextExt<'input>{
	fn new(parent: Option<Rc<dyn SubstraitPlanParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Struct_literalContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Struct_literalContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Struct_literalContextAttrs<'input>: SubstraitPlanParserContext<'input> + BorrowMut<Struct_literalContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token LEFTBRACE
/// Returns `None` if there is no child corresponding to token LEFTBRACE
fn LEFTBRACE(&self) -> Option<Rc<TerminalNode<'input,SubstraitPlanParserContextType>>> where Self:Sized{
	self.get_token(LEFTBRACE, 0)
}
fn constant_all(&self) ->  Vec<Rc<ConstantContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn constant(&self, i: usize) -> Option<Rc<ConstantContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves first TerminalNode corresponding to token RIGHTBRACE
/// Returns `None` if there is no child corresponding to token RIGHTBRACE
fn RIGHTBRACE(&self) -> Option<Rc<TerminalNode<'input,SubstraitPlanParserContextType>>> where Self:Sized{
	self.get_token(RIGHTBRACE, 0)
}
/// Retrieves all `TerminalNode`s corresponding to token COMMA in current rule
fn COMMA_all(&self) -> Vec<Rc<TerminalNode<'input,SubstraitPlanParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token COMMA, starting from 0.
/// Returns `None` if number of children corresponding to token COMMA is less or equal than `i`.
fn COMMA(&self, i: usize) -> Option<Rc<TerminalNode<'input,SubstraitPlanParserContextType>>> where Self:Sized{
	self.get_token(COMMA, i)
}

}

impl<'input> Struct_literalContextAttrs<'input> for Struct_literalContext<'input>{}

impl<'input, I, H> SubstraitPlanParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn struct_literal(&mut self,)
	-> Result<Rc<Struct_literalContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Struct_literalContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 38, RULE_struct_literal);
        let mut _localctx: Rc<Struct_literalContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(432);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(48,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					recog.base.set_state(419);
					recog.base.match_token(LEFTBRACE,&mut recog.err_handler)?;

					/*InvokeRule constant*/
					recog.base.set_state(420);
					recog.constant()?;

					recog.base.set_state(425);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					while _la==COMMA {
						{
						{
						recog.base.set_state(421);
						recog.base.match_token(COMMA,&mut recog.err_handler)?;

						/*InvokeRule constant*/
						recog.base.set_state(422);
						recog.constant()?;

						}
						}
						recog.base.set_state(427);
						recog.err_handler.sync(&mut recog.base)?;
						_la = recog.base.input.la(1);
					}
					recog.base.set_state(428);
					recog.base.match_token(RIGHTBRACE,&mut recog.err_handler)?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(430);
					recog.base.match_token(LEFTBRACE,&mut recog.err_handler)?;

					recog.base.set_state(431);
					recog.base.match_token(RIGHTBRACE,&mut recog.err_handler)?;

					}
				}

				_ => {}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- column_name ----------------
pub type Column_nameContextAll<'input> = Column_nameContext<'input>;


pub type Column_nameContext<'input> = BaseParserRuleContext<'input,Column_nameContextExt<'input>>;

#[derive(Clone)]
pub struct Column_nameContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SubstraitPlanParserContext<'input> for Column_nameContext<'input>{}

impl<'input,'a> Listenable<dyn SubstraitPlanParserListener<'input> + 'a> for Column_nameContext<'input>{
		fn enter(&self,listener: &mut (dyn SubstraitPlanParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_column_name(self);
		}fn exit(&self,listener: &mut (dyn SubstraitPlanParserListener<'input> + 'a)) {
			listener.exit_column_name(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for Column_nameContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SubstraitPlanParserContextType;
	fn get_rule_index(&self) -> usize { RULE_column_name }
	//fn type_rule_index() -> usize where Self: Sized { RULE_column_name }
}
antlr_rust::tid!{Column_nameContextExt<'a>}

impl<'input> Column_nameContextExt<'input>{
	fn new(parent: Option<Rc<dyn SubstraitPlanParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Column_nameContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Column_nameContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Column_nameContextAttrs<'input>: SubstraitPlanParserContext<'input> + BorrowMut<Column_nameContextExt<'input>>{

fn id_all(&self) ->  Vec<Rc<IdContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn id(&self, i: usize) -> Option<Rc<IdContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves first TerminalNode corresponding to token PERIOD
/// Returns `None` if there is no child corresponding to token PERIOD
fn PERIOD(&self) -> Option<Rc<TerminalNode<'input,SubstraitPlanParserContextType>>> where Self:Sized{
	self.get_token(PERIOD, 0)
}

}

impl<'input> Column_nameContextAttrs<'input> for Column_nameContext<'input>{}

impl<'input, I, H> SubstraitPlanParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn column_name(&mut self,)
	-> Result<Rc<Column_nameContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Column_nameContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 40, RULE_column_name);
        let mut _localctx: Rc<Column_nameContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(437);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(49,&mut recog.base)? {
				x if x == 1=>{
					{
					/*InvokeRule id*/
					recog.base.set_state(434);
					recog.id()?;

					recog.base.set_state(435);
					recog.base.match_token(PERIOD,&mut recog.err_handler)?;

					}
				}

				_ => {}
			}
			/*InvokeRule id*/
			recog.base.set_state(439);
			recog.id()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- source_reference ----------------
pub type Source_referenceContextAll<'input> = Source_referenceContext<'input>;


pub type Source_referenceContext<'input> = BaseParserRuleContext<'input,Source_referenceContextExt<'input>>;

#[derive(Clone)]
pub struct Source_referenceContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SubstraitPlanParserContext<'input> for Source_referenceContext<'input>{}

impl<'input,'a> Listenable<dyn SubstraitPlanParserListener<'input> + 'a> for Source_referenceContext<'input>{
		fn enter(&self,listener: &mut (dyn SubstraitPlanParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_source_reference(self);
		}fn exit(&self,listener: &mut (dyn SubstraitPlanParserListener<'input> + 'a)) {
			listener.exit_source_reference(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for Source_referenceContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SubstraitPlanParserContextType;
	fn get_rule_index(&self) -> usize { RULE_source_reference }
	//fn type_rule_index() -> usize where Self: Sized { RULE_source_reference }
}
antlr_rust::tid!{Source_referenceContextExt<'a>}

impl<'input> Source_referenceContextExt<'input>{
	fn new(parent: Option<Rc<dyn SubstraitPlanParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Source_referenceContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Source_referenceContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Source_referenceContextAttrs<'input>: SubstraitPlanParserContext<'input> + BorrowMut<Source_referenceContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token SOURCE
/// Returns `None` if there is no child corresponding to token SOURCE
fn SOURCE(&self) -> Option<Rc<TerminalNode<'input,SubstraitPlanParserContextType>>> where Self:Sized{
	self.get_token(SOURCE, 0)
}
fn id(&self) -> Option<Rc<IdContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Source_referenceContextAttrs<'input> for Source_referenceContext<'input>{}

impl<'input, I, H> SubstraitPlanParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn source_reference(&mut self,)
	-> Result<Rc<Source_referenceContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Source_referenceContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 42, RULE_source_reference);
        let mut _localctx: Rc<Source_referenceContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(441);
			recog.base.match_token(SOURCE,&mut recog.err_handler)?;

			/*InvokeRule id*/
			recog.base.set_state(442);
			recog.id()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- file_location ----------------
pub type File_locationContextAll<'input> = File_locationContext<'input>;


pub type File_locationContext<'input> = BaseParserRuleContext<'input,File_locationContextExt<'input>>;

#[derive(Clone)]
pub struct File_locationContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SubstraitPlanParserContext<'input> for File_locationContext<'input>{}

impl<'input,'a> Listenable<dyn SubstraitPlanParserListener<'input> + 'a> for File_locationContext<'input>{
		fn enter(&self,listener: &mut (dyn SubstraitPlanParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_file_location(self);
		}fn exit(&self,listener: &mut (dyn SubstraitPlanParserListener<'input> + 'a)) {
			listener.exit_file_location(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for File_locationContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SubstraitPlanParserContextType;
	fn get_rule_index(&self) -> usize { RULE_file_location }
	//fn type_rule_index() -> usize where Self: Sized { RULE_file_location }
}
antlr_rust::tid!{File_locationContextExt<'a>}

impl<'input> File_locationContextExt<'input>{
	fn new(parent: Option<Rc<dyn SubstraitPlanParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<File_locationContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,File_locationContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait File_locationContextAttrs<'input>: SubstraitPlanParserContext<'input> + BorrowMut<File_locationContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token URI_FILE
/// Returns `None` if there is no child corresponding to token URI_FILE
fn URI_FILE(&self) -> Option<Rc<TerminalNode<'input,SubstraitPlanParserContextType>>> where Self:Sized{
	self.get_token(URI_FILE, 0)
}
/// Retrieves first TerminalNode corresponding to token COLON
/// Returns `None` if there is no child corresponding to token COLON
fn COLON(&self) -> Option<Rc<TerminalNode<'input,SubstraitPlanParserContextType>>> where Self:Sized{
	self.get_token(COLON, 0)
}
/// Retrieves first TerminalNode corresponding to token STRING
/// Returns `None` if there is no child corresponding to token STRING
fn STRING(&self) -> Option<Rc<TerminalNode<'input,SubstraitPlanParserContextType>>> where Self:Sized{
	self.get_token(STRING, 0)
}
/// Retrieves first TerminalNode corresponding to token URI_PATH
/// Returns `None` if there is no child corresponding to token URI_PATH
fn URI_PATH(&self) -> Option<Rc<TerminalNode<'input,SubstraitPlanParserContextType>>> where Self:Sized{
	self.get_token(URI_PATH, 0)
}
/// Retrieves first TerminalNode corresponding to token URI_PATH_GLOB
/// Returns `None` if there is no child corresponding to token URI_PATH_GLOB
fn URI_PATH_GLOB(&self) -> Option<Rc<TerminalNode<'input,SubstraitPlanParserContextType>>> where Self:Sized{
	self.get_token(URI_PATH_GLOB, 0)
}
/// Retrieves first TerminalNode corresponding to token URI_FOLDER
/// Returns `None` if there is no child corresponding to token URI_FOLDER
fn URI_FOLDER(&self) -> Option<Rc<TerminalNode<'input,SubstraitPlanParserContextType>>> where Self:Sized{
	self.get_token(URI_FOLDER, 0)
}

}

impl<'input> File_locationContextAttrs<'input> for File_locationContext<'input>{}

impl<'input, I, H> SubstraitPlanParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn file_location(&mut self,)
	-> Result<Rc<File_locationContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = File_locationContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 44, RULE_file_location);
        let mut _localctx: Rc<File_locationContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(456);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 URI_FILE 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					recog.base.set_state(444);
					recog.base.match_token(URI_FILE,&mut recog.err_handler)?;

					recog.base.set_state(445);
					recog.base.match_token(COLON,&mut recog.err_handler)?;

					recog.base.set_state(446);
					recog.base.match_token(STRING,&mut recog.err_handler)?;

					}
				}

			 URI_PATH 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(447);
					recog.base.match_token(URI_PATH,&mut recog.err_handler)?;

					recog.base.set_state(448);
					recog.base.match_token(COLON,&mut recog.err_handler)?;

					recog.base.set_state(449);
					recog.base.match_token(STRING,&mut recog.err_handler)?;

					}
				}

			 URI_PATH_GLOB 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					recog.base.set_state(450);
					recog.base.match_token(URI_PATH_GLOB,&mut recog.err_handler)?;

					recog.base.set_state(451);
					recog.base.match_token(COLON,&mut recog.err_handler)?;

					recog.base.set_state(452);
					recog.base.match_token(STRING,&mut recog.err_handler)?;

					}
				}

			 URI_FOLDER 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 4);
					recog.base.enter_outer_alt(None, 4);
					{
					recog.base.set_state(453);
					recog.base.match_token(URI_FOLDER,&mut recog.err_handler)?;

					recog.base.set_state(454);
					recog.base.match_token(COLON,&mut recog.err_handler)?;

					recog.base.set_state(455);
					recog.base.match_token(STRING,&mut recog.err_handler)?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- file_detail ----------------
pub type File_detailContextAll<'input> = File_detailContext<'input>;


pub type File_detailContext<'input> = BaseParserRuleContext<'input,File_detailContextExt<'input>>;

#[derive(Clone)]
pub struct File_detailContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SubstraitPlanParserContext<'input> for File_detailContext<'input>{}

impl<'input,'a> Listenable<dyn SubstraitPlanParserListener<'input> + 'a> for File_detailContext<'input>{
		fn enter(&self,listener: &mut (dyn SubstraitPlanParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_file_detail(self);
		}fn exit(&self,listener: &mut (dyn SubstraitPlanParserListener<'input> + 'a)) {
			listener.exit_file_detail(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for File_detailContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SubstraitPlanParserContextType;
	fn get_rule_index(&self) -> usize { RULE_file_detail }
	//fn type_rule_index() -> usize where Self: Sized { RULE_file_detail }
}
antlr_rust::tid!{File_detailContextExt<'a>}

impl<'input> File_detailContextExt<'input>{
	fn new(parent: Option<Rc<dyn SubstraitPlanParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<File_detailContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,File_detailContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait File_detailContextAttrs<'input>: SubstraitPlanParserContext<'input> + BorrowMut<File_detailContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token PARTITION_INDEX
/// Returns `None` if there is no child corresponding to token PARTITION_INDEX
fn PARTITION_INDEX(&self) -> Option<Rc<TerminalNode<'input,SubstraitPlanParserContextType>>> where Self:Sized{
	self.get_token(PARTITION_INDEX, 0)
}
/// Retrieves first TerminalNode corresponding to token COLON
/// Returns `None` if there is no child corresponding to token COLON
fn COLON(&self) -> Option<Rc<TerminalNode<'input,SubstraitPlanParserContextType>>> where Self:Sized{
	self.get_token(COLON, 0)
}
/// Retrieves first TerminalNode corresponding to token NUMBER
/// Returns `None` if there is no child corresponding to token NUMBER
fn NUMBER(&self) -> Option<Rc<TerminalNode<'input,SubstraitPlanParserContextType>>> where Self:Sized{
	self.get_token(NUMBER, 0)
}
/// Retrieves first TerminalNode corresponding to token START
/// Returns `None` if there is no child corresponding to token START
fn START(&self) -> Option<Rc<TerminalNode<'input,SubstraitPlanParserContextType>>> where Self:Sized{
	self.get_token(START, 0)
}
/// Retrieves first TerminalNode corresponding to token LENGTH
/// Returns `None` if there is no child corresponding to token LENGTH
fn LENGTH(&self) -> Option<Rc<TerminalNode<'input,SubstraitPlanParserContextType>>> where Self:Sized{
	self.get_token(LENGTH, 0)
}
/// Retrieves first TerminalNode corresponding to token ORC
/// Returns `None` if there is no child corresponding to token ORC
fn ORC(&self) -> Option<Rc<TerminalNode<'input,SubstraitPlanParserContextType>>> where Self:Sized{
	self.get_token(ORC, 0)
}
/// Retrieves first TerminalNode corresponding to token LEFTBRACE
/// Returns `None` if there is no child corresponding to token LEFTBRACE
fn LEFTBRACE(&self) -> Option<Rc<TerminalNode<'input,SubstraitPlanParserContextType>>> where Self:Sized{
	self.get_token(LEFTBRACE, 0)
}
/// Retrieves first TerminalNode corresponding to token RIGHTBRACE
/// Returns `None` if there is no child corresponding to token RIGHTBRACE
fn RIGHTBRACE(&self) -> Option<Rc<TerminalNode<'input,SubstraitPlanParserContextType>>> where Self:Sized{
	self.get_token(RIGHTBRACE, 0)
}
/// Retrieves first TerminalNode corresponding to token PARQUET
/// Returns `None` if there is no child corresponding to token PARQUET
fn PARQUET(&self) -> Option<Rc<TerminalNode<'input,SubstraitPlanParserContextType>>> where Self:Sized{
	self.get_token(PARQUET, 0)
}
fn file_location(&self) -> Option<Rc<File_locationContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> File_detailContextAttrs<'input> for File_detailContext<'input>{}

impl<'input, I, H> SubstraitPlanParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn file_detail(&mut self,)
	-> Result<Rc<File_detailContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = File_detailContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 46, RULE_file_detail);
        let mut _localctx: Rc<File_detailContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(476);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 PARTITION_INDEX 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					recog.base.set_state(458);
					recog.base.match_token(PARTITION_INDEX,&mut recog.err_handler)?;

					recog.base.set_state(459);
					recog.base.match_token(COLON,&mut recog.err_handler)?;

					recog.base.set_state(460);
					recog.base.match_token(NUMBER,&mut recog.err_handler)?;

					}
				}

			 START 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(461);
					recog.base.match_token(START,&mut recog.err_handler)?;

					recog.base.set_state(462);
					recog.base.match_token(COLON,&mut recog.err_handler)?;

					recog.base.set_state(463);
					recog.base.match_token(NUMBER,&mut recog.err_handler)?;

					}
				}

			 LENGTH 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					recog.base.set_state(464);
					recog.base.match_token(LENGTH,&mut recog.err_handler)?;

					recog.base.set_state(465);
					recog.base.match_token(COLON,&mut recog.err_handler)?;

					recog.base.set_state(466);
					recog.base.match_token(NUMBER,&mut recog.err_handler)?;

					}
				}

			 ORC 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 4);
					recog.base.enter_outer_alt(None, 4);
					{
					recog.base.set_state(467);
					recog.base.match_token(ORC,&mut recog.err_handler)?;

					recog.base.set_state(468);
					recog.base.match_token(COLON,&mut recog.err_handler)?;

					recog.base.set_state(469);
					recog.base.match_token(LEFTBRACE,&mut recog.err_handler)?;

					recog.base.set_state(470);
					recog.base.match_token(RIGHTBRACE,&mut recog.err_handler)?;

					}
				}

			 PARQUET 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 5);
					recog.base.enter_outer_alt(None, 5);
					{
					recog.base.set_state(471);
					recog.base.match_token(PARQUET,&mut recog.err_handler)?;

					recog.base.set_state(472);
					recog.base.match_token(COLON,&mut recog.err_handler)?;

					recog.base.set_state(473);
					recog.base.match_token(LEFTBRACE,&mut recog.err_handler)?;

					recog.base.set_state(474);
					recog.base.match_token(RIGHTBRACE,&mut recog.err_handler)?;

					}
				}

			 URI_FILE | URI_PATH | URI_PATH_GLOB | URI_FOLDER 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 6);
					recog.base.enter_outer_alt(None, 6);
					{
					/*InvokeRule file_location*/
					recog.base.set_state(475);
					recog.file_location()?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- file ----------------
pub type FileContextAll<'input> = FileContext<'input>;


pub type FileContext<'input> = BaseParserRuleContext<'input,FileContextExt<'input>>;

#[derive(Clone)]
pub struct FileContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SubstraitPlanParserContext<'input> for FileContext<'input>{}

impl<'input,'a> Listenable<dyn SubstraitPlanParserListener<'input> + 'a> for FileContext<'input>{
		fn enter(&self,listener: &mut (dyn SubstraitPlanParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_file(self);
		}fn exit(&self,listener: &mut (dyn SubstraitPlanParserListener<'input> + 'a)) {
			listener.exit_file(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for FileContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SubstraitPlanParserContextType;
	fn get_rule_index(&self) -> usize { RULE_file }
	//fn type_rule_index() -> usize where Self: Sized { RULE_file }
}
antlr_rust::tid!{FileContextExt<'a>}

impl<'input> FileContextExt<'input>{
	fn new(parent: Option<Rc<dyn SubstraitPlanParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<FileContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,FileContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait FileContextAttrs<'input>: SubstraitPlanParserContext<'input> + BorrowMut<FileContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token LEFTBRACE
/// Returns `None` if there is no child corresponding to token LEFTBRACE
fn LEFTBRACE(&self) -> Option<Rc<TerminalNode<'input,SubstraitPlanParserContextType>>> where Self:Sized{
	self.get_token(LEFTBRACE, 0)
}
/// Retrieves first TerminalNode corresponding to token RIGHTBRACE
/// Returns `None` if there is no child corresponding to token RIGHTBRACE
fn RIGHTBRACE(&self) -> Option<Rc<TerminalNode<'input,SubstraitPlanParserContextType>>> where Self:Sized{
	self.get_token(RIGHTBRACE, 0)
}
fn file_detail_all(&self) ->  Vec<Rc<File_detailContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn file_detail(&self, i: usize) -> Option<Rc<File_detailContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> FileContextAttrs<'input> for FileContext<'input>{}

impl<'input, I, H> SubstraitPlanParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn file(&mut self,)
	-> Result<Rc<FileContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = FileContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 48, RULE_file);
        let mut _localctx: Rc<FileContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(478);
			recog.base.match_token(LEFTBRACE,&mut recog.err_handler)?;

			recog.base.set_state(482);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while ((((_la - 38)) & !0x3f) == 0 && ((1usize << (_la - 38)) & ((1usize << (URI_FILE - 38)) | (1usize << (URI_PATH - 38)) | (1usize << (URI_PATH_GLOB - 38)) | (1usize << (URI_FOLDER - 38)) | (1usize << (PARTITION_INDEX - 38)) | (1usize << (START - 38)) | (1usize << (LENGTH - 38)) | (1usize << (ORC - 38)) | (1usize << (PARQUET - 38)))) != 0) {
				{
				{
				/*InvokeRule file_detail*/
				recog.base.set_state(479);
				recog.file_detail()?;

				}
				}
				recog.base.set_state(484);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(485);
			recog.base.match_token(RIGHTBRACE,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- local_files_detail ----------------
pub type Local_files_detailContextAll<'input> = Local_files_detailContext<'input>;


pub type Local_files_detailContext<'input> = BaseParserRuleContext<'input,Local_files_detailContextExt<'input>>;

#[derive(Clone)]
pub struct Local_files_detailContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SubstraitPlanParserContext<'input> for Local_files_detailContext<'input>{}

impl<'input,'a> Listenable<dyn SubstraitPlanParserListener<'input> + 'a> for Local_files_detailContext<'input>{
		fn enter(&self,listener: &mut (dyn SubstraitPlanParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_local_files_detail(self);
		}fn exit(&self,listener: &mut (dyn SubstraitPlanParserListener<'input> + 'a)) {
			listener.exit_local_files_detail(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for Local_files_detailContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SubstraitPlanParserContextType;
	fn get_rule_index(&self) -> usize { RULE_local_files_detail }
	//fn type_rule_index() -> usize where Self: Sized { RULE_local_files_detail }
}
antlr_rust::tid!{Local_files_detailContextExt<'a>}

impl<'input> Local_files_detailContextExt<'input>{
	fn new(parent: Option<Rc<dyn SubstraitPlanParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Local_files_detailContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Local_files_detailContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Local_files_detailContextAttrs<'input>: SubstraitPlanParserContext<'input> + BorrowMut<Local_files_detailContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token ADVANCED_EXTENSION
/// Returns `None` if there is no child corresponding to token ADVANCED_EXTENSION
fn ADVANCED_EXTENSION(&self) -> Option<Rc<TerminalNode<'input,SubstraitPlanParserContextType>>> where Self:Sized{
	self.get_token(ADVANCED_EXTENSION, 0)
}
fn id(&self) -> Option<Rc<IdContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token ITEMS
/// Returns `None` if there is no child corresponding to token ITEMS
fn ITEMS(&self) -> Option<Rc<TerminalNode<'input,SubstraitPlanParserContextType>>> where Self:Sized{
	self.get_token(ITEMS, 0)
}
/// Retrieves first TerminalNode corresponding to token EQUAL
/// Returns `None` if there is no child corresponding to token EQUAL
fn EQUAL(&self) -> Option<Rc<TerminalNode<'input,SubstraitPlanParserContextType>>> where Self:Sized{
	self.get_token(EQUAL, 0)
}
/// Retrieves first TerminalNode corresponding to token LEFTBRACKET
/// Returns `None` if there is no child corresponding to token LEFTBRACKET
fn LEFTBRACKET(&self) -> Option<Rc<TerminalNode<'input,SubstraitPlanParserContextType>>> where Self:Sized{
	self.get_token(LEFTBRACKET, 0)
}
fn file_all(&self) ->  Vec<Rc<FileContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn file(&self, i: usize) -> Option<Rc<FileContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves first TerminalNode corresponding to token RIGHTBRACKET
/// Returns `None` if there is no child corresponding to token RIGHTBRACKET
fn RIGHTBRACKET(&self) -> Option<Rc<TerminalNode<'input,SubstraitPlanParserContextType>>> where Self:Sized{
	self.get_token(RIGHTBRACKET, 0)
}
/// Retrieves all `TerminalNode`s corresponding to token COMMA in current rule
fn COMMA_all(&self) -> Vec<Rc<TerminalNode<'input,SubstraitPlanParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token COMMA, starting from 0.
/// Returns `None` if number of children corresponding to token COMMA is less or equal than `i`.
fn COMMA(&self, i: usize) -> Option<Rc<TerminalNode<'input,SubstraitPlanParserContextType>>> where Self:Sized{
	self.get_token(COMMA, i)
}

}

impl<'input> Local_files_detailContextAttrs<'input> for Local_files_detailContext<'input>{}

impl<'input, I, H> SubstraitPlanParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn local_files_detail(&mut self,)
	-> Result<Rc<Local_files_detailContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Local_files_detailContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 50, RULE_local_files_detail);
        let mut _localctx: Rc<Local_files_detailContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			let mut _alt: isize;
			recog.base.set_state(505);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 ADVANCED_EXTENSION 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					recog.base.set_state(487);
					recog.base.match_token(ADVANCED_EXTENSION,&mut recog.err_handler)?;

					/*InvokeRule id*/
					recog.base.set_state(488);
					recog.id()?;

					}
				}

			 ITEMS 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(489);
					recog.base.match_token(ITEMS,&mut recog.err_handler)?;

					recog.base.set_state(490);
					recog.base.match_token(EQUAL,&mut recog.err_handler)?;

					recog.base.set_state(491);
					recog.base.match_token(LEFTBRACKET,&mut recog.err_handler)?;

					/*InvokeRule file*/
					recog.base.set_state(492);
					recog.file()?;

					recog.base.set_state(497);
					recog.err_handler.sync(&mut recog.base)?;
					_alt = recog.interpreter.adaptive_predict(53,&mut recog.base)?;
					while { _alt!=2 && _alt!=INVALID_ALT } {
						if _alt==1 {
							{
							{
							recog.base.set_state(493);
							recog.base.match_token(COMMA,&mut recog.err_handler)?;

							/*InvokeRule file*/
							recog.base.set_state(494);
							recog.file()?;

							}
							} 
						}
						recog.base.set_state(499);
						recog.err_handler.sync(&mut recog.base)?;
						_alt = recog.interpreter.adaptive_predict(53,&mut recog.base)?;
					}
					recog.base.set_state(501);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==COMMA {
						{
						recog.base.set_state(500);
						recog.base.match_token(COMMA,&mut recog.err_handler)?;

						}
					}

					recog.base.set_state(503);
					recog.base.match_token(RIGHTBRACKET,&mut recog.err_handler)?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- named_table_detail ----------------
pub type Named_table_detailContextAll<'input> = Named_table_detailContext<'input>;


pub type Named_table_detailContext<'input> = BaseParserRuleContext<'input,Named_table_detailContextExt<'input>>;

#[derive(Clone)]
pub struct Named_table_detailContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SubstraitPlanParserContext<'input> for Named_table_detailContext<'input>{}

impl<'input,'a> Listenable<dyn SubstraitPlanParserListener<'input> + 'a> for Named_table_detailContext<'input>{
		fn enter(&self,listener: &mut (dyn SubstraitPlanParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_named_table_detail(self);
		}fn exit(&self,listener: &mut (dyn SubstraitPlanParserListener<'input> + 'a)) {
			listener.exit_named_table_detail(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for Named_table_detailContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SubstraitPlanParserContextType;
	fn get_rule_index(&self) -> usize { RULE_named_table_detail }
	//fn type_rule_index() -> usize where Self: Sized { RULE_named_table_detail }
}
antlr_rust::tid!{Named_table_detailContextExt<'a>}

impl<'input> Named_table_detailContextExt<'input>{
	fn new(parent: Option<Rc<dyn SubstraitPlanParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Named_table_detailContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Named_table_detailContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Named_table_detailContextAttrs<'input>: SubstraitPlanParserContext<'input> + BorrowMut<Named_table_detailContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token ADVANCED_EXTENSION
/// Returns `None` if there is no child corresponding to token ADVANCED_EXTENSION
fn ADVANCED_EXTENSION(&self) -> Option<Rc<TerminalNode<'input,SubstraitPlanParserContextType>>> where Self:Sized{
	self.get_token(ADVANCED_EXTENSION, 0)
}
fn id(&self) -> Option<Rc<IdContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token NAMES
/// Returns `None` if there is no child corresponding to token NAMES
fn NAMES(&self) -> Option<Rc<TerminalNode<'input,SubstraitPlanParserContextType>>> where Self:Sized{
	self.get_token(NAMES, 0)
}
/// Retrieves first TerminalNode corresponding to token EQUAL
/// Returns `None` if there is no child corresponding to token EQUAL
fn EQUAL(&self) -> Option<Rc<TerminalNode<'input,SubstraitPlanParserContextType>>> where Self:Sized{
	self.get_token(EQUAL, 0)
}
/// Retrieves first TerminalNode corresponding to token LEFTBRACKET
/// Returns `None` if there is no child corresponding to token LEFTBRACKET
fn LEFTBRACKET(&self) -> Option<Rc<TerminalNode<'input,SubstraitPlanParserContextType>>> where Self:Sized{
	self.get_token(LEFTBRACKET, 0)
}
/// Retrieves all `TerminalNode`s corresponding to token STRING in current rule
fn STRING_all(&self) -> Vec<Rc<TerminalNode<'input,SubstraitPlanParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token STRING, starting from 0.
/// Returns `None` if number of children corresponding to token STRING is less or equal than `i`.
fn STRING(&self, i: usize) -> Option<Rc<TerminalNode<'input,SubstraitPlanParserContextType>>> where Self:Sized{
	self.get_token(STRING, i)
}
/// Retrieves first TerminalNode corresponding to token RIGHTBRACKET
/// Returns `None` if there is no child corresponding to token RIGHTBRACKET
fn RIGHTBRACKET(&self) -> Option<Rc<TerminalNode<'input,SubstraitPlanParserContextType>>> where Self:Sized{
	self.get_token(RIGHTBRACKET, 0)
}
/// Retrieves all `TerminalNode`s corresponding to token COMMA in current rule
fn COMMA_all(&self) -> Vec<Rc<TerminalNode<'input,SubstraitPlanParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token COMMA, starting from 0.
/// Returns `None` if number of children corresponding to token COMMA is less or equal than `i`.
fn COMMA(&self, i: usize) -> Option<Rc<TerminalNode<'input,SubstraitPlanParserContextType>>> where Self:Sized{
	self.get_token(COMMA, i)
}

}

impl<'input> Named_table_detailContextAttrs<'input> for Named_table_detailContext<'input>{}

impl<'input, I, H> SubstraitPlanParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn named_table_detail(&mut self,)
	-> Result<Rc<Named_table_detailContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Named_table_detailContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 52, RULE_named_table_detail);
        let mut _localctx: Rc<Named_table_detailContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			let mut _alt: isize;
			recog.base.set_state(524);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 ADVANCED_EXTENSION 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					recog.base.set_state(507);
					recog.base.match_token(ADVANCED_EXTENSION,&mut recog.err_handler)?;

					/*InvokeRule id*/
					recog.base.set_state(508);
					recog.id()?;

					}
				}

			 NAMES 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(509);
					recog.base.match_token(NAMES,&mut recog.err_handler)?;

					recog.base.set_state(510);
					recog.base.match_token(EQUAL,&mut recog.err_handler)?;

					recog.base.set_state(511);
					recog.base.match_token(LEFTBRACKET,&mut recog.err_handler)?;

					recog.base.set_state(512);
					recog.base.match_token(STRING,&mut recog.err_handler)?;

					recog.base.set_state(517);
					recog.err_handler.sync(&mut recog.base)?;
					_alt = recog.interpreter.adaptive_predict(56,&mut recog.base)?;
					while { _alt!=2 && _alt!=INVALID_ALT } {
						if _alt==1 {
							{
							{
							recog.base.set_state(513);
							recog.base.match_token(COMMA,&mut recog.err_handler)?;

							recog.base.set_state(514);
							recog.base.match_token(STRING,&mut recog.err_handler)?;

							}
							} 
						}
						recog.base.set_state(519);
						recog.err_handler.sync(&mut recog.base)?;
						_alt = recog.interpreter.adaptive_predict(56,&mut recog.base)?;
					}
					recog.base.set_state(521);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==COMMA {
						{
						recog.base.set_state(520);
						recog.base.match_token(COMMA,&mut recog.err_handler)?;

						}
					}

					recog.base.set_state(523);
					recog.base.match_token(RIGHTBRACKET,&mut recog.err_handler)?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- schema_definition ----------------
pub type Schema_definitionContextAll<'input> = Schema_definitionContext<'input>;


pub type Schema_definitionContext<'input> = BaseParserRuleContext<'input,Schema_definitionContextExt<'input>>;

#[derive(Clone)]
pub struct Schema_definitionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SubstraitPlanParserContext<'input> for Schema_definitionContext<'input>{}

impl<'input,'a> Listenable<dyn SubstraitPlanParserListener<'input> + 'a> for Schema_definitionContext<'input>{
		fn enter(&self,listener: &mut (dyn SubstraitPlanParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_schema_definition(self);
		}fn exit(&self,listener: &mut (dyn SubstraitPlanParserListener<'input> + 'a)) {
			listener.exit_schema_definition(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for Schema_definitionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SubstraitPlanParserContextType;
	fn get_rule_index(&self) -> usize { RULE_schema_definition }
	//fn type_rule_index() -> usize where Self: Sized { RULE_schema_definition }
}
antlr_rust::tid!{Schema_definitionContextExt<'a>}

impl<'input> Schema_definitionContextExt<'input>{
	fn new(parent: Option<Rc<dyn SubstraitPlanParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Schema_definitionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Schema_definitionContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Schema_definitionContextAttrs<'input>: SubstraitPlanParserContext<'input> + BorrowMut<Schema_definitionContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token SCHEMA
/// Returns `None` if there is no child corresponding to token SCHEMA
fn SCHEMA(&self) -> Option<Rc<TerminalNode<'input,SubstraitPlanParserContextType>>> where Self:Sized{
	self.get_token(SCHEMA, 0)
}
fn id(&self) -> Option<Rc<IdContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token LEFTBRACE
/// Returns `None` if there is no child corresponding to token LEFTBRACE
fn LEFTBRACE(&self) -> Option<Rc<TerminalNode<'input,SubstraitPlanParserContextType>>> where Self:Sized{
	self.get_token(LEFTBRACE, 0)
}
/// Retrieves first TerminalNode corresponding to token RIGHTBRACE
/// Returns `None` if there is no child corresponding to token RIGHTBRACE
fn RIGHTBRACE(&self) -> Option<Rc<TerminalNode<'input,SubstraitPlanParserContextType>>> where Self:Sized{
	self.get_token(RIGHTBRACE, 0)
}
fn schema_item_all(&self) ->  Vec<Rc<Schema_itemContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn schema_item(&self, i: usize) -> Option<Rc<Schema_itemContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> Schema_definitionContextAttrs<'input> for Schema_definitionContext<'input>{}

impl<'input, I, H> SubstraitPlanParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn schema_definition(&mut self,)
	-> Result<Rc<Schema_definitionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Schema_definitionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 54, RULE_schema_definition);
        let mut _localctx: Rc<Schema_definitionContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(526);
			recog.base.match_token(SCHEMA,&mut recog.err_handler)?;

			/*InvokeRule id*/
			recog.base.set_state(527);
			recog.id()?;

			recog.base.set_state(528);
			recog.base.match_token(LEFTBRACE,&mut recog.err_handler)?;

			recog.base.set_state(532);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while ((((_la - 5)) & !0x3f) == 0 && ((1usize << (_la - 5)) & ((1usize << (NAMED - 5)) | (1usize << (SCHEMA - 5)) | (1usize << (FILTER - 5)) | (1usize << (GROUPING - 5)) | (1usize << (MEASURE - 5)) | (1usize << (SORT - 5)) | (1usize << (COUNT - 5)) | (1usize << (TYPE - 5)) | (1usize << (EMIT - 5)) | (1usize << (ALL - 5)) | (1usize << (ANY - 5)) | (1usize << (COMPARISON - 5)) | (1usize << (SOURCE - 5)) | (1usize << (ROOT - 5)))) != 0) || _la==NULLVAL || _la==IDENTIFIER {
				{
				{
				/*InvokeRule schema_item*/
				recog.base.set_state(529);
				recog.schema_item()?;

				}
				}
				recog.base.set_state(534);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(535);
			recog.base.match_token(RIGHTBRACE,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- schema_item ----------------
pub type Schema_itemContextAll<'input> = Schema_itemContext<'input>;


pub type Schema_itemContext<'input> = BaseParserRuleContext<'input,Schema_itemContextExt<'input>>;

#[derive(Clone)]
pub struct Schema_itemContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SubstraitPlanParserContext<'input> for Schema_itemContext<'input>{}

impl<'input,'a> Listenable<dyn SubstraitPlanParserListener<'input> + 'a> for Schema_itemContext<'input>{
		fn enter(&self,listener: &mut (dyn SubstraitPlanParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_schema_item(self);
		}fn exit(&self,listener: &mut (dyn SubstraitPlanParserListener<'input> + 'a)) {
			listener.exit_schema_item(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for Schema_itemContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SubstraitPlanParserContextType;
	fn get_rule_index(&self) -> usize { RULE_schema_item }
	//fn type_rule_index() -> usize where Self: Sized { RULE_schema_item }
}
antlr_rust::tid!{Schema_itemContextExt<'a>}

impl<'input> Schema_itemContextExt<'input>{
	fn new(parent: Option<Rc<dyn SubstraitPlanParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Schema_itemContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Schema_itemContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Schema_itemContextAttrs<'input>: SubstraitPlanParserContext<'input> + BorrowMut<Schema_itemContextExt<'input>>{

fn id_all(&self) ->  Vec<Rc<IdContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn id(&self, i: usize) -> Option<Rc<IdContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn literal_complex_type(&self) -> Option<Rc<Literal_complex_typeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token SEMICOLON
/// Returns `None` if there is no child corresponding to token SEMICOLON
fn SEMICOLON(&self) -> Option<Rc<TerminalNode<'input,SubstraitPlanParserContextType>>> where Self:Sized{
	self.get_token(SEMICOLON, 0)
}
/// Retrieves first TerminalNode corresponding to token NAMED
/// Returns `None` if there is no child corresponding to token NAMED
fn NAMED(&self) -> Option<Rc<TerminalNode<'input,SubstraitPlanParserContextType>>> where Self:Sized{
	self.get_token(NAMED, 0)
}

}

impl<'input> Schema_itemContextAttrs<'input> for Schema_itemContext<'input>{}

impl<'input, I, H> SubstraitPlanParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn schema_item(&mut self,)
	-> Result<Rc<Schema_itemContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Schema_itemContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 56, RULE_schema_item);
        let mut _localctx: Rc<Schema_itemContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule id*/
			recog.base.set_state(537);
			recog.id()?;

			/*InvokeRule literal_complex_type*/
			recog.base.set_state(538);
			recog.literal_complex_type()?;

			recog.base.set_state(541);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==NAMED {
				{
				recog.base.set_state(539);
				recog.base.match_token(NAMED,&mut recog.err_handler)?;

				/*InvokeRule id*/
				recog.base.set_state(540);
				recog.id()?;

				}
			}

			recog.base.set_state(543);
			recog.base.match_token(SEMICOLON,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- source_definition ----------------
pub type Source_definitionContextAll<'input> = Source_definitionContext<'input>;


pub type Source_definitionContext<'input> = BaseParserRuleContext<'input,Source_definitionContextExt<'input>>;

#[derive(Clone)]
pub struct Source_definitionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SubstraitPlanParserContext<'input> for Source_definitionContext<'input>{}

impl<'input,'a> Listenable<dyn SubstraitPlanParserListener<'input> + 'a> for Source_definitionContext<'input>{
		fn enter(&self,listener: &mut (dyn SubstraitPlanParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_source_definition(self);
		}fn exit(&self,listener: &mut (dyn SubstraitPlanParserListener<'input> + 'a)) {
			listener.exit_source_definition(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for Source_definitionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SubstraitPlanParserContextType;
	fn get_rule_index(&self) -> usize { RULE_source_definition }
	//fn type_rule_index() -> usize where Self: Sized { RULE_source_definition }
}
antlr_rust::tid!{Source_definitionContextExt<'a>}

impl<'input> Source_definitionContextExt<'input>{
	fn new(parent: Option<Rc<dyn SubstraitPlanParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Source_definitionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Source_definitionContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Source_definitionContextAttrs<'input>: SubstraitPlanParserContext<'input> + BorrowMut<Source_definitionContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token SOURCE
/// Returns `None` if there is no child corresponding to token SOURCE
fn SOURCE(&self) -> Option<Rc<TerminalNode<'input,SubstraitPlanParserContextType>>> where Self:Sized{
	self.get_token(SOURCE, 0)
}
fn read_type(&self) -> Option<Rc<Read_typeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Source_definitionContextAttrs<'input> for Source_definitionContext<'input>{}

impl<'input, I, H> SubstraitPlanParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn source_definition(&mut self,)
	-> Result<Rc<Source_definitionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Source_definitionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 58, RULE_source_definition);
        let mut _localctx: Rc<Source_definitionContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(545);
			recog.base.match_token(SOURCE,&mut recog.err_handler)?;

			/*InvokeRule read_type*/
			recog.base.set_state(546);
			recog.read_type()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- read_type ----------------
#[derive(Debug)]
pub enum Read_typeContextAll<'input>{
	NamedTableContext(NamedTableContext<'input>),
	LocalFilesContext(LocalFilesContext<'input>),
	VirtualTableContext(VirtualTableContext<'input>),
	ExtensionTableContext(ExtensionTableContext<'input>),
Error(Read_typeContext<'input>)
}
antlr_rust::tid!{Read_typeContextAll<'a>}

impl<'input> antlr_rust::parser_rule_context::DerefSeal for Read_typeContextAll<'input>{}

impl<'input> SubstraitPlanParserParserContext<'input> for Read_typeContextAll<'input>{}

impl<'input> Deref for Read_typeContextAll<'input>{
	type Target = dyn Read_typeContextAttrs<'input> + 'input;
	fn deref(&self) -> &Self::Target{
		use Read_typeContextAll::*;
		match self{
			NamedTableContext(inner) => inner,
			LocalFilesContext(inner) => inner,
			VirtualTableContext(inner) => inner,
			ExtensionTableContext(inner) => inner,
Error(inner) => inner
		}
	}
}
impl<'input,'a> Listenable<dyn SubstraitPlanParserListener<'input> + 'a> for Read_typeContextAll<'input>{
    fn enter(&self, listener: &mut (dyn SubstraitPlanParserListener<'input> + 'a)) { self.deref().enter(listener) }
    fn exit(&self, listener: &mut (dyn SubstraitPlanParserListener<'input> + 'a)) { self.deref().exit(listener) }
}



pub type Read_typeContext<'input> = BaseParserRuleContext<'input,Read_typeContextExt<'input>>;

#[derive(Clone)]
pub struct Read_typeContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SubstraitPlanParserContext<'input> for Read_typeContext<'input>{}

impl<'input,'a> Listenable<dyn SubstraitPlanParserListener<'input> + 'a> for Read_typeContext<'input>{
}

impl<'input> CustomRuleContext<'input> for Read_typeContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SubstraitPlanParserContextType;
	fn get_rule_index(&self) -> usize { RULE_read_type }
	//fn type_rule_index() -> usize where Self: Sized { RULE_read_type }
}
antlr_rust::tid!{Read_typeContextExt<'a>}

impl<'input> Read_typeContextExt<'input>{
	fn new(parent: Option<Rc<dyn SubstraitPlanParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Read_typeContextAll<'input>> {
		Rc::new(
		Read_typeContextAll::Error(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Read_typeContextExt{
				ph:PhantomData
			}),
		)
		)
	}
}

pub trait Read_typeContextAttrs<'input>: SubstraitPlanParserContext<'input> + BorrowMut<Read_typeContextExt<'input>>{


}

impl<'input> Read_typeContextAttrs<'input> for Read_typeContext<'input>{}

pub type NamedTableContext<'input> = BaseParserRuleContext<'input,NamedTableContextExt<'input>>;

pub trait NamedTableContextAttrs<'input>: SubstraitPlanParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token NAMED_TABLE
	/// Returns `None` if there is no child corresponding to token NAMED_TABLE
	fn NAMED_TABLE(&self) -> Option<Rc<TerminalNode<'input,SubstraitPlanParserContextType>>> where Self:Sized{
		self.get_token(NAMED_TABLE, 0)
	}
	fn id(&self) -> Option<Rc<IdContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
	/// Retrieves first TerminalNode corresponding to token LEFTBRACE
	/// Returns `None` if there is no child corresponding to token LEFTBRACE
	fn LEFTBRACE(&self) -> Option<Rc<TerminalNode<'input,SubstraitPlanParserContextType>>> where Self:Sized{
		self.get_token(LEFTBRACE, 0)
	}
	/// Retrieves first TerminalNode corresponding to token RIGHTBRACE
	/// Returns `None` if there is no child corresponding to token RIGHTBRACE
	fn RIGHTBRACE(&self) -> Option<Rc<TerminalNode<'input,SubstraitPlanParserContextType>>> where Self:Sized{
		self.get_token(RIGHTBRACE, 0)
	}
	fn named_table_detail_all(&self) ->  Vec<Rc<Named_table_detailContextAll<'input>>> where Self:Sized{
		self.children_of_type()
	}
	fn named_table_detail(&self, i: usize) -> Option<Rc<Named_table_detailContextAll<'input>>> where Self:Sized{
		self.child_of_type(i)
	}
}

impl<'input> NamedTableContextAttrs<'input> for NamedTableContext<'input>{}

pub struct NamedTableContextExt<'input>{
	base:Read_typeContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{NamedTableContextExt<'a>}

impl<'input> SubstraitPlanParserContext<'input> for NamedTableContext<'input>{}

impl<'input,'a> Listenable<dyn SubstraitPlanParserListener<'input> + 'a> for NamedTableContext<'input>{
	fn enter(&self,listener: &mut (dyn SubstraitPlanParserListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_namedTable(self);
	}
}

impl<'input> CustomRuleContext<'input> for NamedTableContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SubstraitPlanParserContextType;
	fn get_rule_index(&self) -> usize { RULE_read_type }
	//fn type_rule_index() -> usize where Self: Sized { RULE_read_type }
}

impl<'input> Borrow<Read_typeContextExt<'input>> for NamedTableContext<'input>{
	fn borrow(&self) -> &Read_typeContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<Read_typeContextExt<'input>> for NamedTableContext<'input>{
	fn borrow_mut(&mut self) -> &mut Read_typeContextExt<'input> { &mut self.base }
}

impl<'input> Read_typeContextAttrs<'input> for NamedTableContext<'input> {}

impl<'input> NamedTableContextExt<'input>{
	fn new(ctx: &dyn Read_typeContextAttrs<'input>) -> Rc<Read_typeContextAll<'input>>  {
		Rc::new(
			Read_typeContextAll::NamedTableContext(
				BaseParserRuleContext::copy_from(ctx,NamedTableContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type LocalFilesContext<'input> = BaseParserRuleContext<'input,LocalFilesContextExt<'input>>;

pub trait LocalFilesContextAttrs<'input>: SubstraitPlanParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token LOCAL_FILES
	/// Returns `None` if there is no child corresponding to token LOCAL_FILES
	fn LOCAL_FILES(&self) -> Option<Rc<TerminalNode<'input,SubstraitPlanParserContextType>>> where Self:Sized{
		self.get_token(LOCAL_FILES, 0)
	}
	fn id(&self) -> Option<Rc<IdContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
	/// Retrieves first TerminalNode corresponding to token LEFTBRACE
	/// Returns `None` if there is no child corresponding to token LEFTBRACE
	fn LEFTBRACE(&self) -> Option<Rc<TerminalNode<'input,SubstraitPlanParserContextType>>> where Self:Sized{
		self.get_token(LEFTBRACE, 0)
	}
	/// Retrieves first TerminalNode corresponding to token RIGHTBRACE
	/// Returns `None` if there is no child corresponding to token RIGHTBRACE
	fn RIGHTBRACE(&self) -> Option<Rc<TerminalNode<'input,SubstraitPlanParserContextType>>> where Self:Sized{
		self.get_token(RIGHTBRACE, 0)
	}
	fn local_files_detail_all(&self) ->  Vec<Rc<Local_files_detailContextAll<'input>>> where Self:Sized{
		self.children_of_type()
	}
	fn local_files_detail(&self, i: usize) -> Option<Rc<Local_files_detailContextAll<'input>>> where Self:Sized{
		self.child_of_type(i)
	}
}

impl<'input> LocalFilesContextAttrs<'input> for LocalFilesContext<'input>{}

pub struct LocalFilesContextExt<'input>{
	base:Read_typeContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{LocalFilesContextExt<'a>}

impl<'input> SubstraitPlanParserContext<'input> for LocalFilesContext<'input>{}

impl<'input,'a> Listenable<dyn SubstraitPlanParserListener<'input> + 'a> for LocalFilesContext<'input>{
	fn enter(&self,listener: &mut (dyn SubstraitPlanParserListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_localFiles(self);
	}
}

impl<'input> CustomRuleContext<'input> for LocalFilesContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SubstraitPlanParserContextType;
	fn get_rule_index(&self) -> usize { RULE_read_type }
	//fn type_rule_index() -> usize where Self: Sized { RULE_read_type }
}

impl<'input> Borrow<Read_typeContextExt<'input>> for LocalFilesContext<'input>{
	fn borrow(&self) -> &Read_typeContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<Read_typeContextExt<'input>> for LocalFilesContext<'input>{
	fn borrow_mut(&mut self) -> &mut Read_typeContextExt<'input> { &mut self.base }
}

impl<'input> Read_typeContextAttrs<'input> for LocalFilesContext<'input> {}

impl<'input> LocalFilesContextExt<'input>{
	fn new(ctx: &dyn Read_typeContextAttrs<'input>) -> Rc<Read_typeContextAll<'input>>  {
		Rc::new(
			Read_typeContextAll::LocalFilesContext(
				BaseParserRuleContext::copy_from(ctx,LocalFilesContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type VirtualTableContext<'input> = BaseParserRuleContext<'input,VirtualTableContextExt<'input>>;

pub trait VirtualTableContextAttrs<'input>: SubstraitPlanParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token VIRTUAL_TABLE
	/// Returns `None` if there is no child corresponding to token VIRTUAL_TABLE
	fn VIRTUAL_TABLE(&self) -> Option<Rc<TerminalNode<'input,SubstraitPlanParserContextType>>> where Self:Sized{
		self.get_token(VIRTUAL_TABLE, 0)
	}
	fn id(&self) -> Option<Rc<IdContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
	/// Retrieves first TerminalNode corresponding to token LEFTBRACE
	/// Returns `None` if there is no child corresponding to token LEFTBRACE
	fn LEFTBRACE(&self) -> Option<Rc<TerminalNode<'input,SubstraitPlanParserContextType>>> where Self:Sized{
		self.get_token(LEFTBRACE, 0)
	}
	/// Retrieves first TerminalNode corresponding to token RIGHTBRACE
	/// Returns `None` if there is no child corresponding to token RIGHTBRACE
	fn RIGHTBRACE(&self) -> Option<Rc<TerminalNode<'input,SubstraitPlanParserContextType>>> where Self:Sized{
		self.get_token(RIGHTBRACE, 0)
	}
}

impl<'input> VirtualTableContextAttrs<'input> for VirtualTableContext<'input>{}

pub struct VirtualTableContextExt<'input>{
	base:Read_typeContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{VirtualTableContextExt<'a>}

impl<'input> SubstraitPlanParserContext<'input> for VirtualTableContext<'input>{}

impl<'input,'a> Listenable<dyn SubstraitPlanParserListener<'input> + 'a> for VirtualTableContext<'input>{
	fn enter(&self,listener: &mut (dyn SubstraitPlanParserListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_virtualTable(self);
	}
}

impl<'input> CustomRuleContext<'input> for VirtualTableContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SubstraitPlanParserContextType;
	fn get_rule_index(&self) -> usize { RULE_read_type }
	//fn type_rule_index() -> usize where Self: Sized { RULE_read_type }
}

impl<'input> Borrow<Read_typeContextExt<'input>> for VirtualTableContext<'input>{
	fn borrow(&self) -> &Read_typeContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<Read_typeContextExt<'input>> for VirtualTableContext<'input>{
	fn borrow_mut(&mut self) -> &mut Read_typeContextExt<'input> { &mut self.base }
}

impl<'input> Read_typeContextAttrs<'input> for VirtualTableContext<'input> {}

impl<'input> VirtualTableContextExt<'input>{
	fn new(ctx: &dyn Read_typeContextAttrs<'input>) -> Rc<Read_typeContextAll<'input>>  {
		Rc::new(
			Read_typeContextAll::VirtualTableContext(
				BaseParserRuleContext::copy_from(ctx,VirtualTableContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type ExtensionTableContext<'input> = BaseParserRuleContext<'input,ExtensionTableContextExt<'input>>;

pub trait ExtensionTableContextAttrs<'input>: SubstraitPlanParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token EXTENSION_TABLE
	/// Returns `None` if there is no child corresponding to token EXTENSION_TABLE
	fn EXTENSION_TABLE(&self) -> Option<Rc<TerminalNode<'input,SubstraitPlanParserContextType>>> where Self:Sized{
		self.get_token(EXTENSION_TABLE, 0)
	}
	fn id(&self) -> Option<Rc<IdContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
	/// Retrieves first TerminalNode corresponding to token LEFTBRACE
	/// Returns `None` if there is no child corresponding to token LEFTBRACE
	fn LEFTBRACE(&self) -> Option<Rc<TerminalNode<'input,SubstraitPlanParserContextType>>> where Self:Sized{
		self.get_token(LEFTBRACE, 0)
	}
	/// Retrieves first TerminalNode corresponding to token RIGHTBRACE
	/// Returns `None` if there is no child corresponding to token RIGHTBRACE
	fn RIGHTBRACE(&self) -> Option<Rc<TerminalNode<'input,SubstraitPlanParserContextType>>> where Self:Sized{
		self.get_token(RIGHTBRACE, 0)
	}
}

impl<'input> ExtensionTableContextAttrs<'input> for ExtensionTableContext<'input>{}

pub struct ExtensionTableContextExt<'input>{
	base:Read_typeContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{ExtensionTableContextExt<'a>}

impl<'input> SubstraitPlanParserContext<'input> for ExtensionTableContext<'input>{}

impl<'input,'a> Listenable<dyn SubstraitPlanParserListener<'input> + 'a> for ExtensionTableContext<'input>{
	fn enter(&self,listener: &mut (dyn SubstraitPlanParserListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_extensionTable(self);
	}
}

impl<'input> CustomRuleContext<'input> for ExtensionTableContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SubstraitPlanParserContextType;
	fn get_rule_index(&self) -> usize { RULE_read_type }
	//fn type_rule_index() -> usize where Self: Sized { RULE_read_type }
}

impl<'input> Borrow<Read_typeContextExt<'input>> for ExtensionTableContext<'input>{
	fn borrow(&self) -> &Read_typeContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<Read_typeContextExt<'input>> for ExtensionTableContext<'input>{
	fn borrow_mut(&mut self) -> &mut Read_typeContextExt<'input> { &mut self.base }
}

impl<'input> Read_typeContextAttrs<'input> for ExtensionTableContext<'input> {}

impl<'input> ExtensionTableContextExt<'input>{
	fn new(ctx: &dyn Read_typeContextAttrs<'input>) -> Rc<Read_typeContextAll<'input>>  {
		Rc::new(
			Read_typeContextAll::ExtensionTableContext(
				BaseParserRuleContext::copy_from(ctx,ExtensionTableContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

impl<'input, I, H> SubstraitPlanParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn read_type(&mut self,)
	-> Result<Rc<Read_typeContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Read_typeContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 60, RULE_read_type);
        let mut _localctx: Rc<Read_typeContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(580);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 LOCAL_FILES 
				=> {
					let tmp = LocalFilesContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 1);
					_localctx = tmp;
					{
					recog.base.set_state(548);
					recog.base.match_token(LOCAL_FILES,&mut recog.err_handler)?;

					/*InvokeRule id*/
					recog.base.set_state(549);
					recog.id()?;

					recog.base.set_state(550);
					recog.base.match_token(LEFTBRACE,&mut recog.err_handler)?;

					recog.base.set_state(554);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					while _la==ADVANCED_EXTENSION || _la==ITEMS {
						{
						{
						/*InvokeRule local_files_detail*/
						recog.base.set_state(551);
						recog.local_files_detail()?;

						}
						}
						recog.base.set_state(556);
						recog.err_handler.sync(&mut recog.base)?;
						_la = recog.base.input.la(1);
					}
					recog.base.set_state(557);
					recog.base.match_token(RIGHTBRACE,&mut recog.err_handler)?;

					}
				}

			 VIRTUAL_TABLE 
				=> {
					let tmp = VirtualTableContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 2);
					_localctx = tmp;
					{
					recog.base.set_state(559);
					recog.base.match_token(VIRTUAL_TABLE,&mut recog.err_handler)?;

					/*InvokeRule id*/
					recog.base.set_state(560);
					recog.id()?;

					recog.base.set_state(561);
					recog.base.match_token(LEFTBRACE,&mut recog.err_handler)?;

					recog.base.set_state(562);
					recog.base.match_token(RIGHTBRACE,&mut recog.err_handler)?;

					}
				}

			 NAMED_TABLE 
				=> {
					let tmp = NamedTableContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 3);
					_localctx = tmp;
					{
					recog.base.set_state(564);
					recog.base.match_token(NAMED_TABLE,&mut recog.err_handler)?;

					/*InvokeRule id*/
					recog.base.set_state(565);
					recog.id()?;

					recog.base.set_state(566);
					recog.base.match_token(LEFTBRACE,&mut recog.err_handler)?;

					recog.base.set_state(570);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					while _la==ADVANCED_EXTENSION || _la==NAMES {
						{
						{
						/*InvokeRule named_table_detail*/
						recog.base.set_state(567);
						recog.named_table_detail()?;

						}
						}
						recog.base.set_state(572);
						recog.err_handler.sync(&mut recog.base)?;
						_la = recog.base.input.la(1);
					}
					recog.base.set_state(573);
					recog.base.match_token(RIGHTBRACE,&mut recog.err_handler)?;

					}
				}

			 EXTENSION_TABLE 
				=> {
					let tmp = ExtensionTableContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 4);
					_localctx = tmp;
					{
					recog.base.set_state(575);
					recog.base.match_token(EXTENSION_TABLE,&mut recog.err_handler)?;

					/*InvokeRule id*/
					recog.base.set_state(576);
					recog.id()?;

					recog.base.set_state(577);
					recog.base.match_token(LEFTBRACE,&mut recog.err_handler)?;

					recog.base.set_state(578);
					recog.base.match_token(RIGHTBRACE,&mut recog.err_handler)?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- extensionspace ----------------
pub type ExtensionspaceContextAll<'input> = ExtensionspaceContext<'input>;


pub type ExtensionspaceContext<'input> = BaseParserRuleContext<'input,ExtensionspaceContextExt<'input>>;

#[derive(Clone)]
pub struct ExtensionspaceContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SubstraitPlanParserContext<'input> for ExtensionspaceContext<'input>{}

impl<'input,'a> Listenable<dyn SubstraitPlanParserListener<'input> + 'a> for ExtensionspaceContext<'input>{
		fn enter(&self,listener: &mut (dyn SubstraitPlanParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_extensionspace(self);
		}fn exit(&self,listener: &mut (dyn SubstraitPlanParserListener<'input> + 'a)) {
			listener.exit_extensionspace(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for ExtensionspaceContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SubstraitPlanParserContextType;
	fn get_rule_index(&self) -> usize { RULE_extensionspace }
	//fn type_rule_index() -> usize where Self: Sized { RULE_extensionspace }
}
antlr_rust::tid!{ExtensionspaceContextExt<'a>}

impl<'input> ExtensionspaceContextExt<'input>{
	fn new(parent: Option<Rc<dyn SubstraitPlanParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ExtensionspaceContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ExtensionspaceContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ExtensionspaceContextAttrs<'input>: SubstraitPlanParserContext<'input> + BorrowMut<ExtensionspaceContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token EXTENSION_SPACE
/// Returns `None` if there is no child corresponding to token EXTENSION_SPACE
fn EXTENSION_SPACE(&self) -> Option<Rc<TerminalNode<'input,SubstraitPlanParserContextType>>> where Self:Sized{
	self.get_token(EXTENSION_SPACE, 0)
}
/// Retrieves first TerminalNode corresponding to token LEFTBRACE
/// Returns `None` if there is no child corresponding to token LEFTBRACE
fn LEFTBRACE(&self) -> Option<Rc<TerminalNode<'input,SubstraitPlanParserContextType>>> where Self:Sized{
	self.get_token(LEFTBRACE, 0)
}
/// Retrieves first TerminalNode corresponding to token RIGHTBRACE
/// Returns `None` if there is no child corresponding to token RIGHTBRACE
fn RIGHTBRACE(&self) -> Option<Rc<TerminalNode<'input,SubstraitPlanParserContextType>>> where Self:Sized{
	self.get_token(RIGHTBRACE, 0)
}
/// Retrieves first TerminalNode corresponding to token URI
/// Returns `None` if there is no child corresponding to token URI
fn URI(&self) -> Option<Rc<TerminalNode<'input,SubstraitPlanParserContextType>>> where Self:Sized{
	self.get_token(URI, 0)
}
fn function_all(&self) ->  Vec<Rc<FunctionContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn function(&self, i: usize) -> Option<Rc<FunctionContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> ExtensionspaceContextAttrs<'input> for ExtensionspaceContext<'input>{}

impl<'input, I, H> SubstraitPlanParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn extensionspace(&mut self,)
	-> Result<Rc<ExtensionspaceContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ExtensionspaceContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 62, RULE_extensionspace);
        let mut _localctx: Rc<ExtensionspaceContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(582);
			recog.base.match_token(EXTENSION_SPACE,&mut recog.err_handler)?;

			recog.base.set_state(584);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==URI {
				{
				recog.base.set_state(583);
				recog.base.match_token(URI,&mut recog.err_handler)?;

				}
			}

			recog.base.set_state(586);
			recog.base.match_token(LEFTBRACE,&mut recog.err_handler)?;

			recog.base.set_state(590);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==FUNCTION {
				{
				{
				/*InvokeRule function*/
				recog.base.set_state(587);
				recog.function()?;

				}
				}
				recog.base.set_state(592);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(593);
			recog.base.match_token(RIGHTBRACE,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- function ----------------
pub type FunctionContextAll<'input> = FunctionContext<'input>;


pub type FunctionContext<'input> = BaseParserRuleContext<'input,FunctionContextExt<'input>>;

#[derive(Clone)]
pub struct FunctionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SubstraitPlanParserContext<'input> for FunctionContext<'input>{}

impl<'input,'a> Listenable<dyn SubstraitPlanParserListener<'input> + 'a> for FunctionContext<'input>{
		fn enter(&self,listener: &mut (dyn SubstraitPlanParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_function(self);
		}fn exit(&self,listener: &mut (dyn SubstraitPlanParserListener<'input> + 'a)) {
			listener.exit_function(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for FunctionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SubstraitPlanParserContextType;
	fn get_rule_index(&self) -> usize { RULE_function }
	//fn type_rule_index() -> usize where Self: Sized { RULE_function }
}
antlr_rust::tid!{FunctionContextExt<'a>}

impl<'input> FunctionContextExt<'input>{
	fn new(parent: Option<Rc<dyn SubstraitPlanParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<FunctionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,FunctionContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait FunctionContextAttrs<'input>: SubstraitPlanParserContext<'input> + BorrowMut<FunctionContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token FUNCTION
/// Returns `None` if there is no child corresponding to token FUNCTION
fn FUNCTION(&self) -> Option<Rc<TerminalNode<'input,SubstraitPlanParserContextType>>> where Self:Sized{
	self.get_token(FUNCTION, 0)
}
fn name(&self) -> Option<Rc<NameContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token SEMICOLON
/// Returns `None` if there is no child corresponding to token SEMICOLON
fn SEMICOLON(&self) -> Option<Rc<TerminalNode<'input,SubstraitPlanParserContextType>>> where Self:Sized{
	self.get_token(SEMICOLON, 0)
}
/// Retrieves first TerminalNode corresponding to token AS
/// Returns `None` if there is no child corresponding to token AS
fn AS(&self) -> Option<Rc<TerminalNode<'input,SubstraitPlanParserContextType>>> where Self:Sized{
	self.get_token(AS, 0)
}
fn id(&self) -> Option<Rc<IdContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> FunctionContextAttrs<'input> for FunctionContext<'input>{}

impl<'input, I, H> SubstraitPlanParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn function(&mut self,)
	-> Result<Rc<FunctionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = FunctionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 64, RULE_function);
        let mut _localctx: Rc<FunctionContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(595);
			recog.base.match_token(FUNCTION,&mut recog.err_handler)?;

			/*InvokeRule name*/
			recog.base.set_state(596);
			recog.name()?;

			recog.base.set_state(599);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==AS {
				{
				recog.base.set_state(597);
				recog.base.match_token(AS,&mut recog.err_handler)?;

				/*InvokeRule id*/
				recog.base.set_state(598);
				recog.id()?;

				}
			}

			recog.base.set_state(601);
			recog.base.match_token(SEMICOLON,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- sort_field ----------------
pub type Sort_fieldContextAll<'input> = Sort_fieldContext<'input>;


pub type Sort_fieldContext<'input> = BaseParserRuleContext<'input,Sort_fieldContextExt<'input>>;

#[derive(Clone)]
pub struct Sort_fieldContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SubstraitPlanParserContext<'input> for Sort_fieldContext<'input>{}

impl<'input,'a> Listenable<dyn SubstraitPlanParserListener<'input> + 'a> for Sort_fieldContext<'input>{
		fn enter(&self,listener: &mut (dyn SubstraitPlanParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_sort_field(self);
		}fn exit(&self,listener: &mut (dyn SubstraitPlanParserListener<'input> + 'a)) {
			listener.exit_sort_field(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for Sort_fieldContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SubstraitPlanParserContextType;
	fn get_rule_index(&self) -> usize { RULE_sort_field }
	//fn type_rule_index() -> usize where Self: Sized { RULE_sort_field }
}
antlr_rust::tid!{Sort_fieldContextExt<'a>}

impl<'input> Sort_fieldContextExt<'input>{
	fn new(parent: Option<Rc<dyn SubstraitPlanParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Sort_fieldContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Sort_fieldContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Sort_fieldContextAttrs<'input>: SubstraitPlanParserContext<'input> + BorrowMut<Sort_fieldContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token SORT
/// Returns `None` if there is no child corresponding to token SORT
fn SORT(&self) -> Option<Rc<TerminalNode<'input,SubstraitPlanParserContextType>>> where Self:Sized{
	self.get_token(SORT, 0)
}
fn expression(&self) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token SEMICOLON
/// Returns `None` if there is no child corresponding to token SEMICOLON
fn SEMICOLON(&self) -> Option<Rc<TerminalNode<'input,SubstraitPlanParserContextType>>> where Self:Sized{
	self.get_token(SEMICOLON, 0)
}
/// Retrieves first TerminalNode corresponding to token BY
/// Returns `None` if there is no child corresponding to token BY
fn BY(&self) -> Option<Rc<TerminalNode<'input,SubstraitPlanParserContextType>>> where Self:Sized{
	self.get_token(BY, 0)
}
fn id(&self) -> Option<Rc<IdContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Sort_fieldContextAttrs<'input> for Sort_fieldContext<'input>{}

impl<'input, I, H> SubstraitPlanParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn sort_field(&mut self,)
	-> Result<Rc<Sort_fieldContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Sort_fieldContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 66, RULE_sort_field);
        let mut _localctx: Rc<Sort_fieldContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(603);
			recog.base.match_token(SORT,&mut recog.err_handler)?;

			/*InvokeRule expression*/
			recog.base.set_state(604);
			recog.expression_rec(0)?;

			recog.base.set_state(607);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==BY {
				{
				recog.base.set_state(605);
				recog.base.match_token(BY,&mut recog.err_handler)?;

				/*InvokeRule id*/
				recog.base.set_state(606);
				recog.id()?;

				}
			}

			recog.base.set_state(609);
			recog.base.match_token(SEMICOLON,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- name ----------------
pub type NameContextAll<'input> = NameContext<'input>;


pub type NameContext<'input> = BaseParserRuleContext<'input,NameContextExt<'input>>;

#[derive(Clone)]
pub struct NameContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SubstraitPlanParserContext<'input> for NameContext<'input>{}

impl<'input,'a> Listenable<dyn SubstraitPlanParserListener<'input> + 'a> for NameContext<'input>{
		fn enter(&self,listener: &mut (dyn SubstraitPlanParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_name(self);
		}fn exit(&self,listener: &mut (dyn SubstraitPlanParserListener<'input> + 'a)) {
			listener.exit_name(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for NameContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SubstraitPlanParserContextType;
	fn get_rule_index(&self) -> usize { RULE_name }
	//fn type_rule_index() -> usize where Self: Sized { RULE_name }
}
antlr_rust::tid!{NameContextExt<'a>}

impl<'input> NameContextExt<'input>{
	fn new(parent: Option<Rc<dyn SubstraitPlanParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<NameContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,NameContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait NameContextAttrs<'input>: SubstraitPlanParserContext<'input> + BorrowMut<NameContextExt<'input>>{

fn id(&self) -> Option<Rc<IdContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token COLON
/// Returns `None` if there is no child corresponding to token COLON
fn COLON(&self) -> Option<Rc<TerminalNode<'input,SubstraitPlanParserContextType>>> where Self:Sized{
	self.get_token(COLON, 0)
}
fn signature(&self) -> Option<Rc<SignatureContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> NameContextAttrs<'input> for NameContext<'input>{}

impl<'input, I, H> SubstraitPlanParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn name(&mut self,)
	-> Result<Rc<NameContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = NameContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 68, RULE_name);
        let mut _localctx: Rc<NameContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule id*/
			recog.base.set_state(611);
			recog.id()?;

			recog.base.set_state(612);
			recog.base.match_token(COLON,&mut recog.err_handler)?;

			recog.base.set_state(614);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if ((((_la - 5)) & !0x3f) == 0 && ((1usize << (_la - 5)) & ((1usize << (NAMED - 5)) | (1usize << (SCHEMA - 5)) | (1usize << (FILTER - 5)) | (1usize << (GROUPING - 5)) | (1usize << (MEASURE - 5)) | (1usize << (SORT - 5)) | (1usize << (COUNT - 5)) | (1usize << (TYPE - 5)) | (1usize << (EMIT - 5)) | (1usize << (ALL - 5)) | (1usize << (ANY - 5)) | (1usize << (COMPARISON - 5)) | (1usize << (SOURCE - 5)) | (1usize << (ROOT - 5)))) != 0) || _la==NULLVAL || _la==IDENTIFIER {
				{
				/*InvokeRule signature*/
				recog.base.set_state(613);
				recog.signature()?;

				}
			}

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- signature ----------------
pub type SignatureContextAll<'input> = SignatureContext<'input>;


pub type SignatureContext<'input> = BaseParserRuleContext<'input,SignatureContextExt<'input>>;

#[derive(Clone)]
pub struct SignatureContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SubstraitPlanParserContext<'input> for SignatureContext<'input>{}

impl<'input,'a> Listenable<dyn SubstraitPlanParserListener<'input> + 'a> for SignatureContext<'input>{
		fn enter(&self,listener: &mut (dyn SubstraitPlanParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_signature(self);
		}fn exit(&self,listener: &mut (dyn SubstraitPlanParserListener<'input> + 'a)) {
			listener.exit_signature(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for SignatureContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SubstraitPlanParserContextType;
	fn get_rule_index(&self) -> usize { RULE_signature }
	//fn type_rule_index() -> usize where Self: Sized { RULE_signature }
}
antlr_rust::tid!{SignatureContextExt<'a>}

impl<'input> SignatureContextExt<'input>{
	fn new(parent: Option<Rc<dyn SubstraitPlanParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<SignatureContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,SignatureContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait SignatureContextAttrs<'input>: SubstraitPlanParserContext<'input> + BorrowMut<SignatureContextExt<'input>>{

fn id(&self) -> Option<Rc<IdContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> SignatureContextAttrs<'input> for SignatureContext<'input>{}

impl<'input, I, H> SubstraitPlanParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn signature(&mut self,)
	-> Result<Rc<SignatureContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = SignatureContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 70, RULE_signature);
        let mut _localctx: Rc<SignatureContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule id*/
			recog.base.set_state(616);
			recog.id()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- id ----------------
pub type IdContextAll<'input> = IdContext<'input>;


pub type IdContext<'input> = BaseParserRuleContext<'input,IdContextExt<'input>>;

#[derive(Clone)]
pub struct IdContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SubstraitPlanParserContext<'input> for IdContext<'input>{}

impl<'input,'a> Listenable<dyn SubstraitPlanParserListener<'input> + 'a> for IdContext<'input>{
		fn enter(&self,listener: &mut (dyn SubstraitPlanParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_id(self);
		}fn exit(&self,listener: &mut (dyn SubstraitPlanParserListener<'input> + 'a)) {
			listener.exit_id(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for IdContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SubstraitPlanParserContextType;
	fn get_rule_index(&self) -> usize { RULE_id }
	//fn type_rule_index() -> usize where Self: Sized { RULE_id }
}
antlr_rust::tid!{IdContextExt<'a>}

impl<'input> IdContextExt<'input>{
	fn new(parent: Option<Rc<dyn SubstraitPlanParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<IdContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,IdContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait IdContextAttrs<'input>: SubstraitPlanParserContext<'input> + BorrowMut<IdContextExt<'input>>{

fn simple_id_all(&self) ->  Vec<Rc<Simple_idContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn simple_id(&self, i: usize) -> Option<Rc<Simple_idContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves all `TerminalNode`s corresponding to token UNDERSCORE in current rule
fn UNDERSCORE_all(&self) -> Vec<Rc<TerminalNode<'input,SubstraitPlanParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token UNDERSCORE, starting from 0.
/// Returns `None` if number of children corresponding to token UNDERSCORE is less or equal than `i`.
fn UNDERSCORE(&self, i: usize) -> Option<Rc<TerminalNode<'input,SubstraitPlanParserContextType>>> where Self:Sized{
	self.get_token(UNDERSCORE, i)
}

}

impl<'input> IdContextAttrs<'input> for IdContext<'input>{}

impl<'input, I, H> SubstraitPlanParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn id(&mut self,)
	-> Result<Rc<IdContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = IdContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 72, RULE_id);
        let mut _localctx: Rc<IdContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			let mut _alt: isize;
			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule simple_id*/
			recog.base.set_state(618);
			recog.simple_id()?;

			recog.base.set_state(627);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(70,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					{
					{
					recog.base.set_state(620); 
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					loop {
						{
						{
						recog.base.set_state(619);
						recog.base.match_token(UNDERSCORE,&mut recog.err_handler)?;

						}
						}
						recog.base.set_state(622); 
						recog.err_handler.sync(&mut recog.base)?;
						_la = recog.base.input.la(1);
						if !(_la==UNDERSCORE) {break}
					}
					/*InvokeRule simple_id*/
					recog.base.set_state(624);
					recog.simple_id()?;

					}
					} 
				}
				recog.base.set_state(629);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(70,&mut recog.base)?;
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- simple_id ----------------
pub type Simple_idContextAll<'input> = Simple_idContext<'input>;


pub type Simple_idContext<'input> = BaseParserRuleContext<'input,Simple_idContextExt<'input>>;

#[derive(Clone)]
pub struct Simple_idContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SubstraitPlanParserContext<'input> for Simple_idContext<'input>{}

impl<'input,'a> Listenable<dyn SubstraitPlanParserListener<'input> + 'a> for Simple_idContext<'input>{
		fn enter(&self,listener: &mut (dyn SubstraitPlanParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_simple_id(self);
		}fn exit(&self,listener: &mut (dyn SubstraitPlanParserListener<'input> + 'a)) {
			listener.exit_simple_id(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for Simple_idContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SubstraitPlanParserContextType;
	fn get_rule_index(&self) -> usize { RULE_simple_id }
	//fn type_rule_index() -> usize where Self: Sized { RULE_simple_id }
}
antlr_rust::tid!{Simple_idContextExt<'a>}

impl<'input> Simple_idContextExt<'input>{
	fn new(parent: Option<Rc<dyn SubstraitPlanParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Simple_idContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Simple_idContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Simple_idContextAttrs<'input>: SubstraitPlanParserContext<'input> + BorrowMut<Simple_idContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token IDENTIFIER
/// Returns `None` if there is no child corresponding to token IDENTIFIER
fn IDENTIFIER(&self) -> Option<Rc<TerminalNode<'input,SubstraitPlanParserContextType>>> where Self:Sized{
	self.get_token(IDENTIFIER, 0)
}
/// Retrieves first TerminalNode corresponding to token FILTER
/// Returns `None` if there is no child corresponding to token FILTER
fn FILTER(&self) -> Option<Rc<TerminalNode<'input,SubstraitPlanParserContextType>>> where Self:Sized{
	self.get_token(FILTER, 0)
}
/// Retrieves first TerminalNode corresponding to token ROOT
/// Returns `None` if there is no child corresponding to token ROOT
fn ROOT(&self) -> Option<Rc<TerminalNode<'input,SubstraitPlanParserContextType>>> where Self:Sized{
	self.get_token(ROOT, 0)
}
/// Retrieves first TerminalNode corresponding to token SOURCE
/// Returns `None` if there is no child corresponding to token SOURCE
fn SOURCE(&self) -> Option<Rc<TerminalNode<'input,SubstraitPlanParserContextType>>> where Self:Sized{
	self.get_token(SOURCE, 0)
}
/// Retrieves first TerminalNode corresponding to token SCHEMA
/// Returns `None` if there is no child corresponding to token SCHEMA
fn SCHEMA(&self) -> Option<Rc<TerminalNode<'input,SubstraitPlanParserContextType>>> where Self:Sized{
	self.get_token(SCHEMA, 0)
}
/// Retrieves first TerminalNode corresponding to token NULLVAL
/// Returns `None` if there is no child corresponding to token NULLVAL
fn NULLVAL(&self) -> Option<Rc<TerminalNode<'input,SubstraitPlanParserContextType>>> where Self:Sized{
	self.get_token(NULLVAL, 0)
}
/// Retrieves first TerminalNode corresponding to token SORT
/// Returns `None` if there is no child corresponding to token SORT
fn SORT(&self) -> Option<Rc<TerminalNode<'input,SubstraitPlanParserContextType>>> where Self:Sized{
	self.get_token(SORT, 0)
}
/// Retrieves first TerminalNode corresponding to token MEASURE
/// Returns `None` if there is no child corresponding to token MEASURE
fn MEASURE(&self) -> Option<Rc<TerminalNode<'input,SubstraitPlanParserContextType>>> where Self:Sized{
	self.get_token(MEASURE, 0)
}
/// Retrieves first TerminalNode corresponding to token GROUPING
/// Returns `None` if there is no child corresponding to token GROUPING
fn GROUPING(&self) -> Option<Rc<TerminalNode<'input,SubstraitPlanParserContextType>>> where Self:Sized{
	self.get_token(GROUPING, 0)
}
/// Retrieves first TerminalNode corresponding to token COUNT
/// Returns `None` if there is no child corresponding to token COUNT
fn COUNT(&self) -> Option<Rc<TerminalNode<'input,SubstraitPlanParserContextType>>> where Self:Sized{
	self.get_token(COUNT, 0)
}
/// Retrieves first TerminalNode corresponding to token TYPE
/// Returns `None` if there is no child corresponding to token TYPE
fn TYPE(&self) -> Option<Rc<TerminalNode<'input,SubstraitPlanParserContextType>>> where Self:Sized{
	self.get_token(TYPE, 0)
}
/// Retrieves first TerminalNode corresponding to token EMIT
/// Returns `None` if there is no child corresponding to token EMIT
fn EMIT(&self) -> Option<Rc<TerminalNode<'input,SubstraitPlanParserContextType>>> where Self:Sized{
	self.get_token(EMIT, 0)
}
/// Retrieves first TerminalNode corresponding to token NAMED
/// Returns `None` if there is no child corresponding to token NAMED
fn NAMED(&self) -> Option<Rc<TerminalNode<'input,SubstraitPlanParserContextType>>> where Self:Sized{
	self.get_token(NAMED, 0)
}
/// Retrieves first TerminalNode corresponding to token ALL
/// Returns `None` if there is no child corresponding to token ALL
fn ALL(&self) -> Option<Rc<TerminalNode<'input,SubstraitPlanParserContextType>>> where Self:Sized{
	self.get_token(ALL, 0)
}
/// Retrieves first TerminalNode corresponding to token ANY
/// Returns `None` if there is no child corresponding to token ANY
fn ANY(&self) -> Option<Rc<TerminalNode<'input,SubstraitPlanParserContextType>>> where Self:Sized{
	self.get_token(ANY, 0)
}
/// Retrieves first TerminalNode corresponding to token COMPARISON
/// Returns `None` if there is no child corresponding to token COMPARISON
fn COMPARISON(&self) -> Option<Rc<TerminalNode<'input,SubstraitPlanParserContextType>>> where Self:Sized{
	self.get_token(COMPARISON, 0)
}

}

impl<'input> Simple_idContextAttrs<'input> for Simple_idContext<'input>{}

impl<'input, I, H> SubstraitPlanParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn simple_id(&mut self,)
	-> Result<Rc<Simple_idContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Simple_idContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 74, RULE_simple_id);
        let mut _localctx: Rc<Simple_idContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(630);
			_la = recog.base.input.la(1);
			if { !(((((_la - 5)) & !0x3f) == 0 && ((1usize << (_la - 5)) & ((1usize << (NAMED - 5)) | (1usize << (SCHEMA - 5)) | (1usize << (FILTER - 5)) | (1usize << (GROUPING - 5)) | (1usize << (MEASURE - 5)) | (1usize << (SORT - 5)) | (1usize << (COUNT - 5)) | (1usize << (TYPE - 5)) | (1usize << (EMIT - 5)) | (1usize << (ALL - 5)) | (1usize << (ANY - 5)) | (1usize << (COMPARISON - 5)) | (1usize << (SOURCE - 5)) | (1usize << (ROOT - 5)))) != 0) || _la==NULLVAL || _la==IDENTIFIER) } {
				recog.err_handler.recover_inline(&mut recog.base)?;

			}
			else {
				if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
				recog.err_handler.report_match(&mut recog.base);
				recog.base.consume(&mut recog.err_handler);
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}

lazy_static! {
    static ref _ATN: Arc<ATN> =
        Arc::new(ATNDeserializer::new(None).deserialize(_serializedATN.chars()));
    static ref _decision_to_DFA: Arc<Vec<antlr_rust::RwLock<DFA>>> = {
        let mut dfa = Vec::new();
        let size = _ATN.decision_to_state.len();
        for i in 0..size {
            dfa.push(DFA::new(
                _ATN.clone(),
                _ATN.get_decision_state(i),
                i as isize,
            ).into())
        }
        Arc::new(dfa)
    };
}



const _serializedATN:&'static str =
	"\x03\u{608b}\u{a72a}\u{8133}\u{b9ed}\u{417c}\u{3be7}\u{7786}\u{5964}\x03\
	\x4d\u{27b}\x04\x02\x09\x02\x04\x03\x09\x03\x04\x04\x09\x04\x04\x05\x09\
	\x05\x04\x06\x09\x06\x04\x07\x09\x07\x04\x08\x09\x08\x04\x09\x09\x09\x04\
	\x0a\x09\x0a\x04\x0b\x09\x0b\x04\x0c\x09\x0c\x04\x0d\x09\x0d\x04\x0e\x09\
	\x0e\x04\x0f\x09\x0f\x04\x10\x09\x10\x04\x11\x09\x11\x04\x12\x09\x12\x04\
	\x13\x09\x13\x04\x14\x09\x14\x04\x15\x09\x15\x04\x16\x09\x16\x04\x17\x09\
	\x17\x04\x18\x09\x18\x04\x19\x09\x19\x04\x1a\x09\x1a\x04\x1b\x09\x1b\x04\
	\x1c\x09\x1c\x04\x1d\x09\x1d\x04\x1e\x09\x1e\x04\x1f\x09\x1f\x04\x20\x09\
	\x20\x04\x21\x09\x21\x04\x22\x09\x22\x04\x23\x09\x23\x04\x24\x09\x24\x04\
	\x25\x09\x25\x04\x26\x09\x26\x04\x27\x09\x27\x03\x02\x07\x02\x50\x0a\x02\
	\x0c\x02\x0e\x02\x53\x0b\x02\x03\x02\x03\x02\x03\x03\x03\x03\x03\x03\x03\
	\x03\x03\x03\x03\x03\x05\x03\x5d\x0a\x03\x03\x04\x03\x04\x03\x04\x03\x04\
	\x03\x04\x07\x04\x64\x0a\x04\x0c\x04\x0e\x04\x67\x0b\x04\x03\x04\x03\x04\
	\x03\x05\x03\x05\x03\x05\x03\x05\x03\x05\x03\x05\x07\x05\x71\x0a\x05\x0c\
	\x05\x0e\x05\x74\x0b\x05\x03\x06\x03\x06\x03\x06\x03\x06\x03\x06\x07\x06\
	\x7b\x0a\x06\x0c\x06\x0e\x06\x7e\x0b\x06\x03\x06\x03\x06\x03\x07\x03\x07\
	\x03\x07\x03\x07\x03\x07\x03\x07\x03\x07\x03\x07\x07\x07\u{8a}\x0a\x07\x0c\
	\x07\x0e\x07\u{8d}\x0b\x07\x03\x07\x05\x07\u{90}\x0a\x07\x03\x07\x03\x07\
	\x03\x07\x03\x08\x03\x08\x03\x09\x03\x09\x03\x09\x03\x09\x03\x09\x03\x09\
	\x05\x09\u{9d}\x0a\x09\x03\x0a\x03\x0a\x03\x0a\x03\x0a\x03\x0a\x03\x0a\x03\
	\x0a\x03\x0a\x05\x0a\u{a7}\x0a\x0a\x03\x0b\x03\x0b\x03\x0b\x03\x0b\x05\x0b\
	\u{ad}\x0a\x0b\x03\x0b\x03\x0b\x05\x0b\u{b1}\x0a\x0b\x03\x0b\x03\x0b\x05\
	\x0b\u{b5}\x0a\x0b\x03\x0b\x03\x0b\x03\x0b\x03\x0b\x03\x0b\x03\x0b\x03\x0b\
	\x03\x0b\x03\x0b\x03\x0b\x03\x0b\x05\x0b\u{c2}\x0a\x0b\x03\x0c\x03\x0c\x03\
	\x0c\x03\x0c\x03\x0c\x03\x0c\x03\x0c\x05\x0c\u{cb}\x0a\x0c\x03\x0c\x03\x0c\
	\x03\x0c\x03\x0c\x03\x0c\x03\x0c\x03\x0c\x03\x0c\x05\x0c\u{d5}\x0a\x0c\x03\
	\x0c\x03\x0c\x03\x0c\x03\x0c\x03\x0c\x03\x0c\x03\x0c\x03\x0c\x03\x0c\x03\
	\x0c\x03\x0c\x03\x0c\x03\x0c\x03\x0c\x07\x0c\u{e5}\x0a\x0c\x0c\x0c\x0e\x0c\
	\u{e8}\x0b\x0c\x03\x0c\x03\x0c\x03\x0c\x03\x0c\x03\x0c\x03\x0c\x03\x0c\x03\
	\x0c\x03\x0c\x03\x0c\x03\x0c\x03\x0c\x03\x0c\x05\x0c\u{f7}\x0a\x0c\x03\x0d\
	\x03\x0d\x03\x0d\x03\x0d\x03\x0d\x05\x0d\u{fe}\x0a\x0d\x07\x0d\u{100}\x0a\
	\x0d\x0c\x0d\x0e\x0d\u{103}\x0b\x0d\x03\x0d\x03\x0d\x03\x0d\x05\x0d\u{108}\
	\x0a\x0d\x03\x0d\x03\x0d\x03\x0d\x03\x0d\x03\x0d\x03\x0d\x03\x0d\x03\x0d\
	\x03\x0d\x03\x0d\x03\x0d\x03\x0d\x03\x0d\x05\x0d\u{117}\x0a\x0d\x03\x0d\
	\x03\x0d\x03\x0d\x03\x0d\x03\x0d\x03\x0d\x03\x0d\x03\x0d\x07\x0d\u{121}\
	\x0a\x0d\x0c\x0d\x0e\x0d\u{124}\x0b\x0d\x03\x0e\x03\x0e\x03\x0e\x03\x0e\
	\x07\x0e\u{12a}\x0a\x0e\x0c\x0e\x0e\x0e\u{12d}\x0b\x0e\x03\x0e\x03\x0e\x03\
	\x0f\x03\x0f\x03\x0f\x05\x0f\u{134}\x0a\x0f\x03\x0f\x03\x0f\x03\x0f\x05\
	\x0f\u{139}\x0a\x0f\x03\x0f\x03\x0f\x03\x0f\x05\x0f\u{13e}\x0a\x0f\x03\x0f\
	\x03\x0f\x03\x0f\x05\x0f\u{143}\x0a\x0f\x03\x0f\x03\x0f\x03\x0f\x05\x0f\
	\u{148}\x0a\x0f\x03\x0f\x03\x0f\x03\x0f\x05\x0f\u{14d}\x0a\x0f\x03\x0f\x03\
	\x0f\x03\x0f\x05\x0f\u{152}\x0a\x0f\x05\x0f\u{154}\x0a\x0f\x03\x10\x03\x10\
	\x05\x10\u{158}\x0a\x10\x03\x10\x05\x10\u{15b}\x0a\x10\x03\x11\x03\x11\x03\
	\x11\x05\x11\u{160}\x0a\x11\x03\x11\x03\x11\x05\x11\u{164}\x0a\x11\x03\x11\
	\x03\x11\x03\x11\x05\x11\u{169}\x0a\x11\x03\x11\x03\x11\x05\x11\u{16d}\x0a\
	\x11\x03\x11\x05\x11\u{170}\x0a\x11\x03\x11\x05\x11\u{173}\x0a\x11\x03\x11\
	\x03\x11\x03\x11\x05\x11\u{178}\x0a\x11\x03\x11\x03\x11\x05\x11\u{17c}\x0a\
	\x11\x03\x11\x03\x11\x07\x11\u{180}\x0a\x11\x0c\x11\x0e\x11\u{183}\x0b\x11\
	\x03\x11\x05\x11\u{186}\x0a\x11\x03\x12\x03\x12\x03\x12\x03\x12\x07\x12\
	\u{18c}\x0a\x12\x0c\x12\x0e\x12\u{18f}\x0b\x12\x03\x12\x03\x12\x03\x13\x03\
	\x13\x03\x13\x03\x13\x07\x13\u{197}\x0a\x13\x0c\x13\x0e\x13\u{19a}\x0b\x13\
	\x03\x13\x03\x13\x03\x13\x03\x13\x05\x13\u{1a0}\x0a\x13\x03\x14\x03\x14\
	\x03\x14\x03\x14\x03\x15\x03\x15\x03\x15\x03\x15\x07\x15\u{1aa}\x0a\x15\
	\x0c\x15\x0e\x15\u{1ad}\x0b\x15\x03\x15\x03\x15\x03\x15\x03\x15\x05\x15\
	\u{1b3}\x0a\x15\x03\x16\x03\x16\x03\x16\x05\x16\u{1b8}\x0a\x16\x03\x16\x03\
	\x16\x03\x17\x03\x17\x03\x17\x03\x18\x03\x18\x03\x18\x03\x18\x03\x18\x03\
	\x18\x03\x18\x03\x18\x03\x18\x03\x18\x03\x18\x03\x18\x05\x18\u{1cb}\x0a\
	\x18\x03\x19\x03\x19\x03\x19\x03\x19\x03\x19\x03\x19\x03\x19\x03\x19\x03\
	\x19\x03\x19\x03\x19\x03\x19\x03\x19\x03\x19\x03\x19\x03\x19\x03\x19\x03\
	\x19\x05\x19\u{1df}\x0a\x19\x03\x1a\x03\x1a\x07\x1a\u{1e3}\x0a\x1a\x0c\x1a\
	\x0e\x1a\u{1e6}\x0b\x1a\x03\x1a\x03\x1a\x03\x1b\x03\x1b\x03\x1b\x03\x1b\
	\x03\x1b\x03\x1b\x03\x1b\x03\x1b\x07\x1b\u{1f2}\x0a\x1b\x0c\x1b\x0e\x1b\
	\u{1f5}\x0b\x1b\x03\x1b\x05\x1b\u{1f8}\x0a\x1b\x03\x1b\x03\x1b\x05\x1b\u{1fc}\
	\x0a\x1b\x03\x1c\x03\x1c\x03\x1c\x03\x1c\x03\x1c\x03\x1c\x03\x1c\x03\x1c\
	\x07\x1c\u{206}\x0a\x1c\x0c\x1c\x0e\x1c\u{209}\x0b\x1c\x03\x1c\x05\x1c\u{20c}\
	\x0a\x1c\x03\x1c\x05\x1c\u{20f}\x0a\x1c\x03\x1d\x03\x1d\x03\x1d\x03\x1d\
	\x07\x1d\u{215}\x0a\x1d\x0c\x1d\x0e\x1d\u{218}\x0b\x1d\x03\x1d\x03\x1d\x03\
	\x1e\x03\x1e\x03\x1e\x03\x1e\x05\x1e\u{220}\x0a\x1e\x03\x1e\x03\x1e\x03\
	\x1f\x03\x1f\x03\x1f\x03\x20\x03\x20\x03\x20\x03\x20\x07\x20\u{22b}\x0a\
	\x20\x0c\x20\x0e\x20\u{22e}\x0b\x20\x03\x20\x03\x20\x03\x20\x03\x20\x03\
	\x20\x03\x20\x03\x20\x03\x20\x03\x20\x03\x20\x03\x20\x07\x20\u{23b}\x0a\
	\x20\x0c\x20\x0e\x20\u{23e}\x0b\x20\x03\x20\x03\x20\x03\x20\x03\x20\x03\
	\x20\x03\x20\x03\x20\x05\x20\u{247}\x0a\x20\x03\x21\x03\x21\x05\x21\u{24b}\
	\x0a\x21\x03\x21\x03\x21\x07\x21\u{24f}\x0a\x21\x0c\x21\x0e\x21\u{252}\x0b\
	\x21\x03\x21\x03\x21\x03\x22\x03\x22\x03\x22\x03\x22\x05\x22\u{25a}\x0a\
	\x22\x03\x22\x03\x22\x03\x23\x03\x23\x03\x23\x03\x23\x05\x23\u{262}\x0a\
	\x23\x03\x23\x03\x23\x03\x24\x03\x24\x03\x24\x05\x24\u{269}\x0a\x24\x03\
	\x25\x03\x25\x03\x26\x03\x26\x06\x26\u{26f}\x0a\x26\x0d\x26\x0e\x26\u{270}\
	\x03\x26\x07\x26\u{274}\x0a\x26\x0c\x26\x0e\x26\u{277}\x0b\x26\x03\x27\x03\
	\x27\x03\x27\x02\x04\x08\x18\x28\x02\x04\x06\x08\x0a\x0c\x0e\x10\x12\x14\
	\x16\x18\x1a\x1c\x1e\x20\x22\x24\x26\x28\x2a\x2c\x2e\x30\x32\x34\x36\x38\
	\x3a\x3c\x3e\x40\x42\x44\x46\x48\x4a\x4c\x02\x05\x03\x02\x1a\x1b\x03\x02\
	\x1d\x1e\x0b\x02\x07\x08\x0d\x0d\x11\x12\x14\x14\x16\x18\x1d\x1f\x24\x25\
	\x31\x31\x49\x49\x02\u{2bf}\x02\x51\x03\x02\x02\x02\x04\x5c\x03\x02\x02\
	\x02\x06\x5e\x03\x02\x02\x02\x08\x6a\x03\x02\x02\x02\x0a\x75\x03\x02\x02\
	\x02\x0c\u{81}\x03\x02\x02\x02\x0e\u{94}\x03\x02\x02\x02\x10\u{96}\x03\x02\
	\x02\x02\x12\u{a6}\x03\x02\x02\x02\x14\u{c1}\x03\x02\x02\x02\x16\u{f6}\x03\
	\x02\x02\x02\x18\u{116}\x03\x02\x02\x02\x1a\u{125}\x03\x02\x02\x02\x1c\u{153}\
	\x03\x02\x02\x02\x1e\u{155}\x03\x02\x02\x02\x20\u{185}\x03\x02\x02\x02\x22\
	\u{187}\x03\x02\x02\x02\x24\u{19f}\x03\x02\x02\x02\x26\u{1a1}\x03\x02\x02\
	\x02\x28\u{1b2}\x03\x02\x02\x02\x2a\u{1b7}\x03\x02\x02\x02\x2c\u{1bb}\x03\
	\x02\x02\x02\x2e\u{1ca}\x03\x02\x02\x02\x30\u{1de}\x03\x02\x02\x02\x32\u{1e0}\
	\x03\x02\x02\x02\x34\u{1fb}\x03\x02\x02\x02\x36\u{20e}\x03\x02\x02\x02\x38\
	\u{210}\x03\x02\x02\x02\x3a\u{21b}\x03\x02\x02\x02\x3c\u{223}\x03\x02\x02\
	\x02\x3e\u{246}\x03\x02\x02\x02\x40\u{248}\x03\x02\x02\x02\x42\u{255}\x03\
	\x02\x02\x02\x44\u{25d}\x03\x02\x02\x02\x46\u{265}\x03\x02\x02\x02\x48\u{26a}\
	\x03\x02\x02\x02\x4a\u{26c}\x03\x02\x02\x02\x4c\u{278}\x03\x02\x02\x02\x4e\
	\x50\x05\x04\x03\x02\x4f\x4e\x03\x02\x02\x02\x50\x53\x03\x02\x02\x02\x51\
	\x4f\x03\x02\x02\x02\x51\x52\x03\x02\x02\x02\x52\x54\x03\x02\x02\x02\x53\
	\x51\x03\x02\x02\x02\x54\x55\x07\x02\x02\x03\x55\x03\x03\x02\x02\x02\x56\
	\x5d\x05\x06\x04\x02\x57\x5d\x05\x0a\x06\x02\x58\x5d\x05\x0c\x07\x02\x59\
	\x5d\x05\x38\x1d\x02\x5a\x5d\x05\x3c\x1f\x02\x5b\x5d\x05\x40\x21\x02\x5c\
	\x56\x03\x02\x02\x02\x5c\x57\x03\x02\x02\x02\x5c\x58\x03\x02\x02\x02\x5c\
	\x59\x03\x02\x02\x02\x5c\x5a\x03\x02\x02\x02\x5c\x5b\x03\x02\x02\x02\x5d\
	\x05\x03\x02\x02\x02\x5e\x5f\x07\x0a\x02\x02\x5f\x65\x07\x3a\x02\x02\x60\
	\x61\x05\x08\x05\x02\x61\x62\x07\x39\x02\x02\x62\x64\x03\x02\x02\x02\x63\
	\x60\x03\x02\x02\x02\x64\x67\x03\x02\x02\x02\x65\x63\x03\x02\x02\x02\x65\
	\x66\x03\x02\x02\x02\x66\x68\x03\x02\x02\x02\x67\x65\x03\x02\x02\x02\x68\
	\x69\x07\x3b\x02\x02\x69\x07\x03\x02\x02\x02\x6a\x6b\x08\x05\x01\x02\x6b\
	\x6c\x05\x10\x09\x02\x6c\x72\x03\x02\x02\x02\x6d\x6e\x0c\x04\x02\x02\x6e\
	\x6f\x07\x37\x02\x02\x6f\x71\x05\x10\x09\x02\x70\x6d\x03\x02\x02\x02\x71\
	\x74\x03\x02\x02\x02\x72\x70\x03\x02\x02\x02\x72\x73\x03\x02\x02\x02\x73\
	\x09\x03\x02\x02\x02\x74\x72\x03\x02\x02\x02\x75\x76\x05\x0e\x08\x02\x76\
	\x77\x07\x09\x02\x02\x77\x78\x05\x10\x09\x02\x78\x7c\x07\x3a\x02\x02\x79\
	\x7b\x05\x16\x0c\x02\x7a\x79\x03\x02\x02\x02\x7b\x7e\x03\x02\x02\x02\x7c\
	\x7a\x03\x02\x02\x02\x7c\x7d\x03\x02\x02\x02\x7d\x7f\x03\x02\x02\x02\x7e\
	\x7c\x03\x02\x02\x02\x7f\u{80}\x07\x3b\x02\x02\u{80}\x0b\x03\x02\x02\x02\
	\u{81}\u{82}\x07\x25\x02\x02\u{82}\u{83}\x07\x3a\x02\x02\u{83}\u{84}\x07\
	\x27\x02\x02\u{84}\u{85}\x07\x40\x02\x02\u{85}\u{86}\x07\x41\x02\x02\u{86}\
	\u{8b}\x05\x4a\x26\x02\u{87}\u{88}\x07\x3e\x02\x02\u{88}\u{8a}\x05\x4a\x26\
	\x02\u{89}\u{87}\x03\x02\x02\x02\u{8a}\u{8d}\x03\x02\x02\x02\u{8b}\u{89}\
	\x03\x02\x02\x02\u{8b}\u{8c}\x03\x02\x02\x02\u{8c}\u{8f}\x03\x02\x02\x02\
	\u{8d}\u{8b}\x03\x02\x02\x02\u{8e}\u{90}\x07\x3e\x02\x02\u{8f}\u{8e}\x03\
	\x02\x02\x02\u{8f}\u{90}\x03\x02\x02\x02\u{90}\u{91}\x03\x02\x02\x02\u{91}\
	\u{92}\x07\x42\x02\x02\u{92}\u{93}\x07\x3b\x02\x02\u{93}\x0d\x03\x02\x02\
	\x02\u{94}\u{95}\x05\x4a\x26\x02\u{95}\x0f\x03\x02\x02\x02\u{96}\u{9c}\x05\
	\x4a\x26\x02\u{97}\u{98}\x07\x3c\x02\x02\u{98}\u{99}\x07\x08\x02\x02\u{99}\
	\u{9a}\x05\x4a\x26\x02\u{9a}\u{9b}\x07\x3d\x02\x02\u{9b}\u{9d}\x03\x02\x02\
	\x02\u{9c}\u{97}\x03\x02\x02\x02\u{9c}\u{9d}\x03\x02\x02\x02\u{9d}\x11\x03\
	\x02\x02\x02\u{9e}\u{a7}\x05\x4a\x26\x02\u{9f}\u{a0}\x05\x4a\x26\x02\u{a0}\
	\u{a1}\x07\x44\x02\x02\u{a1}\u{a2}\x05\x4a\x26\x02\u{a2}\u{a7}\x03\x02\x02\
	\x02\u{a3}\u{a4}\x05\x4a\x26\x02\u{a4}\u{a5}\x05\x4a\x26\x02\u{a5}\u{a7}\
	\x03\x02\x02\x02\u{a6}\u{9e}\x03\x02\x02\x02\u{a6}\u{9f}\x03\x02\x02\x02\
	\u{a6}\u{a3}\x03\x02\x02\x02\u{a7}\x13\x03\x02\x02\x02\u{a8}\u{a9}\x07\x12\
	\x02\x02\u{a9}\u{ac}\x05\x18\x0d\x02\u{aa}\u{ab}\x07\x37\x02\x02\u{ab}\u{ad}\
	\x05\x20\x11\x02\u{ac}\u{aa}\x03\x02\x02\x02\u{ac}\u{ad}\x03\x02\x02\x02\
	\u{ad}\u{b0}\x03\x02\x02\x02\u{ae}\u{af}\x07\x48\x02\x02\u{af}\u{b1}\x05\
	\x4a\x26\x02\u{b0}\u{ae}\x03\x02\x02\x02\u{b0}\u{b1}\x03\x02\x02\x02\u{b1}\
	\u{b4}\x03\x02\x02\x02\u{b2}\u{b3}\x07\x07\x02\x02\u{b3}\u{b5}\x05\x4a\x26\
	\x02\u{b4}\u{b2}\x03\x02\x02\x02\u{b4}\u{b5}\x03\x02\x02\x02\u{b5}\u{b6}\
	\x03\x02\x02\x02\u{b6}\u{b7}\x07\x39\x02\x02\u{b7}\u{c2}\x03\x02\x02\x02\
	\u{b8}\u{b9}\x07\x0d\x02\x02\u{b9}\u{ba}\x05\x18\x0d\x02\u{ba}\u{bb}\x07\
	\x39\x02\x02\u{bb}\u{c2}\x03\x02\x02\x02\u{bc}\u{bd}\x07\x13\x02\x02\u{bd}\
	\u{be}\x05\x4a\x26\x02\u{be}\u{bf}\x07\x39\x02\x02\u{bf}\u{c2}\x03\x02\x02\
	\x02\u{c0}\u{c2}\x05\x44\x23\x02\u{c1}\u{a8}\x03\x02\x02\x02\u{c1}\u{b8}\
	\x03\x02\x02\x02\u{c1}\u{bc}\x03\x02\x02\x02\u{c1}\u{c0}\x03\x02\x02\x02\
	\u{c2}\x15\x03\x02\x02\x02\u{c3}\u{c4}\x07\x0b\x02\x02\u{c4}\u{f7}\x07\x39\
	\x02\x02\u{c5}\u{c6}\x07\x0c\x02\x02\u{c6}\u{c7}\x05\x4a\x26\x02\u{c7}\u{c8}\
	\x07\x39\x02\x02\u{c8}\u{f7}\x03\x02\x02\x02\u{c9}\u{cb}\x05\x12\x0a\x02\
	\u{ca}\u{c9}\x03\x02\x02\x02\u{ca}\u{cb}\x03\x02\x02\x02\u{cb}\u{cc}\x03\
	\x02\x02\x02\u{cc}\u{cd}\x07\x0d\x02\x02\u{cd}\u{ce}\x05\x18\x0d\x02\u{ce}\
	\u{cf}\x07\x39\x02\x02\u{cf}\u{f7}\x03\x02\x02\x02\u{d0}\u{d1}\x07\x0f\x02\
	\x02\u{d1}\u{d4}\x05\x18\x0d\x02\u{d2}\u{d3}\x07\x07\x02\x02\u{d3}\u{d5}\
	\x05\x4a\x26\x02\u{d4}\u{d2}\x03\x02\x02\x02\u{d4}\u{d5}\x03\x02\x02\x02\
	\u{d5}\u{d6}\x03\x02\x02\x02\u{d6}\u{d7}\x07\x39\x02\x02\u{d7}\u{f7}\x03\
	\x02\x02\x02\u{d8}\u{d9}\x07\x10\x02\x02\u{d9}\u{f7}\x07\x39\x02\x02\u{da}\
	\u{db}\x05\x2c\x17\x02\u{db}\u{dc}\x07\x39\x02\x02\u{dc}\u{f7}\x03\x02\x02\
	\x02\u{dd}\u{de}\x07\x11\x02\x02\u{de}\u{df}\x05\x18\x0d\x02\u{df}\u{e0}\
	\x07\x39\x02\x02\u{e0}\u{f7}\x03\x02\x02\x02\u{e1}\u{e2}\x07\x12\x02\x02\
	\u{e2}\u{e6}\x07\x3a\x02\x02\u{e3}\u{e5}\x05\x14\x0b\x02\u{e4}\u{e3}\x03\
	\x02\x02\x02\u{e5}\u{e8}\x03\x02\x02\x02\u{e6}\u{e4}\x03\x02\x02\x02\u{e6}\
	\u{e7}\x03\x02\x02\x02\u{e7}\u{e9}\x03\x02\x02\x02\u{e8}\u{e6}\x03\x02\x02\
	\x02\u{e9}\u{f7}\x07\x3b\x02\x02\u{ea}\u{f7}\x05\x44\x23\x02\u{eb}\u{ec}\
	\x07\x16\x02\x02\u{ec}\u{ed}\x07\x4a\x02\x02\u{ed}\u{f7}\x07\x39\x02\x02\
	\u{ee}\u{ef}\x07\x17\x02\x02\u{ef}\u{f0}\x05\x4a\x26\x02\u{f0}\u{f1}\x07\
	\x39\x02\x02\u{f1}\u{f7}\x03\x02\x02\x02\u{f2}\u{f3}\x07\x18\x02\x02\u{f3}\
	\u{f4}\x05\x2a\x16\x02\u{f4}\u{f5}\x07\x39\x02\x02\u{f5}\u{f7}\x03\x02\x02\
	\x02\u{f6}\u{c3}\x03\x02\x02\x02\u{f6}\u{c5}\x03\x02\x02\x02\u{f6}\u{ca}\
	\x03\x02\x02\x02\u{f6}\u{d0}\x03\x02\x02\x02\u{f6}\u{d8}\x03\x02\x02\x02\
	\u{f6}\u{da}\x03\x02\x02\x02\u{f6}\u{dd}\x03\x02\x02\x02\u{f6}\u{e1}\x03\
	\x02\x02\x02\u{f6}\u{ea}\x03\x02\x02\x02\u{f6}\u{eb}\x03\x02\x02\x02\u{f6}\
	\u{ee}\x03\x02\x02\x02\u{f6}\u{f2}\x03\x02\x02\x02\u{f7}\x17\x03\x02\x02\
	\x02\u{f8}\u{f9}\x08\x0d\x01\x02\u{f9}\u{fa}\x05\x4a\x26\x02\u{fa}\u{101}\
	\x07\x3c\x02\x02\u{fb}\u{fd}\x05\x18\x0d\x02\u{fc}\u{fe}\x07\x3e\x02\x02\
	\u{fd}\u{fc}\x03\x02\x02\x02\u{fd}\u{fe}\x03\x02\x02\x02\u{fe}\u{100}\x03\
	\x02\x02\x02\u{ff}\u{fb}\x03\x02\x02\x02\u{100}\u{103}\x03\x02\x02\x02\u{101}\
	\u{ff}\x03\x02\x02\x02\u{101}\u{102}\x03\x02\x02\x02\u{102}\u{104}\x03\x02\
	\x02\x02\u{103}\u{101}\x03\x02\x02\x02\u{104}\u{107}\x07\x3d\x02\x02\u{105}\
	\u{106}\x07\x37\x02\x02\u{106}\u{108}\x05\x20\x11\x02\u{107}\u{105}\x03\
	\x02\x02\x02\u{107}\u{108}\x03\x02\x02\x02\u{108}\u{117}\x03\x02\x02\x02\
	\u{109}\u{117}\x05\x1c\x0f\x02\u{10a}\u{117}\x05\x2a\x16\x02\u{10b}\u{10c}\
	\x07\x19\x02\x02\u{10c}\u{117}\x05\x10\x09\x02\u{10d}\u{10e}\x05\x1a\x0e\
	\x02\u{10e}\u{10f}\x07\x1c\x02\x02\u{10f}\u{110}\x07\x19\x02\x02\u{110}\
	\u{111}\x05\x10\x09\x02\u{111}\u{117}\x03\x02\x02\x02\u{112}\u{113}\x09\
	\x02\x02\x02\u{113}\u{114}\x07\x1c\x02\x02\u{114}\u{115}\x07\x19\x02\x02\
	\u{115}\u{117}\x05\x10\x09\x02\u{116}\u{f8}\x03\x02\x02\x02\u{116}\u{109}\
	\x03\x02\x02\x02\u{116}\u{10a}\x03\x02\x02\x02\u{116}\u{10b}\x03\x02\x02\
	\x02\u{116}\u{10d}\x03\x02\x02\x02\u{116}\u{112}\x03\x02\x02\x02\u{117}\
	\u{122}\x03\x02\x02\x02\u{118}\u{119}\x0c\x07\x02\x02\u{119}\u{11a}\x07\
	\x06\x02\x02\u{11a}\u{121}\x05\x20\x11\x02\u{11b}\u{11c}\x0c\x03\x02\x02\
	\u{11c}\u{11d}\x07\x1f\x02\x02\u{11d}\u{11e}\x09\x03\x02\x02\u{11e}\u{11f}\
	\x07\x19\x02\x02\u{11f}\u{121}\x05\x10\x09\x02\u{120}\u{118}\x03\x02\x02\
	\x02\u{120}\u{11b}\x03\x02\x02\x02\u{121}\u{124}\x03\x02\x02\x02\u{122}\
	\u{120}\x03\x02\x02\x02\u{122}\u{123}\x03\x02\x02\x02\u{123}\x19\x03\x02\
	\x02\x02\u{124}\u{122}\x03\x02\x02\x02\u{125}\u{126}\x07\x3c\x02\x02\u{126}\
	\u{12b}\x05\x18\x0d\x02\u{127}\u{128}\x07\x3e\x02\x02\u{128}\u{12a}\x05\
	\x18\x0d\x02\u{129}\u{127}\x03\x02\x02\x02\u{12a}\u{12d}\x03\x02\x02\x02\
	\u{12b}\u{129}\x03\x02\x02\x02\u{12b}\u{12c}\x03\x02\x02\x02\u{12c}\u{12e}\
	\x03\x02\x02\x02\u{12d}\u{12b}\x03\x02\x02\x02\u{12e}\u{12f}\x07\x3d\x02\
	\x02\u{12f}\x1b\x03\x02\x02\x02\u{130}\u{133}\x07\x4a\x02\x02\u{131}\u{132}\
	\x07\x43\x02\x02\u{132}\u{134}\x05\x1e\x10\x02\u{133}\u{131}\x03\x02\x02\
	\x02\u{133}\u{134}\x03\x02\x02\x02\u{134}\u{154}\x03\x02\x02\x02\u{135}\
	\u{138}\x07\x4b\x02\x02\u{136}\u{137}\x07\x43\x02\x02\u{137}\u{139}\x05\
	\x1e\x10\x02\u{138}\u{136}\x03\x02\x02\x02\u{138}\u{139}\x03\x02\x02\x02\
	\u{139}\u{154}\x03\x02\x02\x02\u{13a}\u{13d}\x05\x24\x13\x02\u{13b}\u{13c}\
	\x07\x43\x02\x02\u{13c}\u{13e}\x05\x20\x11\x02\u{13d}\u{13b}\x03\x02\x02\
	\x02\u{13d}\u{13e}\x03\x02\x02\x02\u{13e}\u{154}\x03\x02\x02\x02\u{13f}\
	\u{142}\x05\x28\x15\x02\u{140}\u{141}\x07\x43\x02\x02\u{141}\u{143}\x05\
	\x20\x11\x02\u{142}\u{140}\x03\x02\x02\x02\u{142}\u{143}\x03\x02\x02\x02\
	\u{143}\u{154}\x03\x02\x02\x02\u{144}\u{147}\x07\x31\x02\x02\u{145}\u{146}\
	\x07\x43\x02\x02\u{146}\u{148}\x05\x20\x11\x02\u{147}\u{145}\x03\x02\x02\
	\x02\u{147}\u{148}\x03\x02\x02\x02\u{148}\u{154}\x03\x02\x02\x02\u{149}\
	\u{14c}\x07\x32\x02\x02\u{14a}\u{14b}\x07\x43\x02\x02\u{14b}\u{14d}\x05\
	\x1e\x10\x02\u{14c}\u{14a}\x03\x02\x02\x02\u{14c}\u{14d}\x03\x02\x02\x02\
	\u{14d}\u{154}\x03\x02\x02\x02\u{14e}\u{151}\x07\x33\x02\x02\u{14f}\u{150}\
	\x07\x43\x02\x02\u{150}\u{152}\x05\x1e\x10\x02\u{151}\u{14f}\x03\x02\x02\
	\x02\u{151}\u{152}\x03\x02\x02\x02\u{152}\u{154}\x03\x02\x02\x02\u{153}\
	\u{130}\x03\x02\x02\x02\u{153}\u{135}\x03\x02\x02\x02\u{153}\u{13a}\x03\
	\x02\x02\x02\u{153}\u{13f}\x03\x02\x02\x02\u{153}\u{144}\x03\x02\x02\x02\
	\u{153}\u{149}\x03\x02\x02\x02\u{153}\u{14e}\x03\x02\x02\x02\u{154}\x1d\
	\x03\x02\x02\x02\u{155}\u{157}\x05\x4a\x26\x02\u{156}\u{158}\x07\x47\x02\
	\x02\u{157}\u{156}\x03\x02\x02\x02\u{157}\u{158}\x03\x02\x02\x02\u{158}\
	\u{15a}\x03\x02\x02\x02\u{159}\u{15b}\x05\x22\x12\x02\u{15a}\u{159}\x03\
	\x02\x02\x02\u{15a}\u{15b}\x03\x02\x02\x02\u{15b}\x1f\x03\x02\x02\x02\u{15c}\
	\u{186}\x05\x1e\x10\x02\u{15d}\u{15f}\x07\x34\x02\x02\u{15e}\u{160}\x07\
	\x47\x02\x02\u{15f}\u{15e}\x03\x02\x02\x02\u{15f}\u{160}\x03\x02\x02\x02\
	\u{160}\u{161}\x03\x02\x02\x02\u{161}\u{163}\x07\x45\x02\x02\u{162}\u{164}\
	\x05\x20\x11\x02\u{163}\u{162}\x03\x02\x02\x02\u{163}\u{164}\x03\x02\x02\
	\x02\u{164}\u{165}\x03\x02\x02\x02\u{165}\u{186}\x07\x46\x02\x02\u{166}\
	\u{168}\x07\x35\x02\x02\u{167}\u{169}\x07\x47\x02\x02\u{168}\u{167}\x03\
	\x02\x02\x02\u{168}\u{169}\x03\x02\x02\x02\u{169}\u{16a}\x03\x02\x02\x02\
	\u{16a}\u{16c}\x07\x45\x02\x02\u{16b}\u{16d}\x05\x1e\x10\x02\u{16c}\u{16b}\
	\x03\x02\x02\x02\u{16c}\u{16d}\x03\x02\x02\x02\u{16d}\u{16f}\x03\x02\x02\
	\x02\u{16e}\u{170}\x07\x3e\x02\x02\u{16f}\u{16e}\x03\x02\x02\x02\u{16f}\
	\u{170}\x03\x02\x02\x02\u{170}\u{172}\x03\x02\x02\x02\u{171}\u{173}\x05\
	\x20\x11\x02\u{172}\u{171}\x03\x02\x02\x02\u{172}\u{173}\x03\x02\x02\x02\
	\u{173}\u{174}\x03\x02\x02\x02\u{174}\u{186}\x07\x46\x02\x02\u{175}\u{177}\
	\x07\x36\x02\x02\u{176}\u{178}\x07\x47\x02\x02\u{177}\u{176}\x03\x02\x02\
	\x02\u{177}\u{178}\x03\x02\x02\x02\u{178}\u{179}\x03\x02\x02\x02\u{179}\
	\u{17b}\x07\x45\x02\x02\u{17a}\u{17c}\x05\x20\x11\x02\u{17b}\u{17a}\x03\
	\x02\x02\x02\u{17b}\u{17c}\x03\x02\x02\x02\u{17c}\u{181}\x03\x02\x02\x02\
	\u{17d}\u{17e}\x07\x3e\x02\x02\u{17e}\u{180}\x05\x20\x11\x02\u{17f}\u{17d}\
	\x03\x02\x02\x02\u{180}\u{183}\x03\x02\x02\x02\u{181}\u{17f}\x03\x02\x02\
	\x02\u{181}\u{182}\x03\x02\x02\x02\u{182}\u{184}\x03\x02\x02\x02\u{183}\
	\u{181}\x03\x02\x02\x02\u{184}\u{186}\x07\x46\x02\x02\u{185}\u{15c}\x03\
	\x02\x02\x02\u{185}\u{15d}\x03\x02\x02\x02\u{185}\u{166}\x03\x02\x02\x02\
	\u{185}\u{175}\x03\x02\x02\x02\u{186}\x21\x03\x02\x02\x02\u{187}\u{188}\
	\x07\x45\x02\x02\u{188}\u{18d}\x07\x4a\x02\x02\u{189}\u{18a}\x07\x3e\x02\
	\x02\u{18a}\u{18c}\x07\x4a\x02\x02\u{18b}\u{189}\x03\x02\x02\x02\u{18c}\
	\u{18f}\x03\x02\x02\x02\u{18d}\u{18b}\x03\x02\x02\x02\u{18d}\u{18e}\x03\
	\x02\x02\x02\u{18e}\u{190}\x03\x02\x02\x02\u{18f}\u{18d}\x03\x02\x02\x02\
	\u{190}\u{191}\x07\x46\x02\x02\u{191}\x23\x03\x02\x02\x02\u{192}\u{193}\
	\x07\x3a\x02\x02\u{193}\u{198}\x05\x26\x14\x02\u{194}\u{195}\x07\x3e\x02\
	\x02\u{195}\u{197}\x05\x26\x14\x02\u{196}\u{194}\x03\x02\x02\x02\u{197}\
	\u{19a}\x03\x02\x02\x02\u{198}\u{196}\x03\x02\x02\x02\u{198}\u{199}\x03\
	\x02\x02\x02\u{199}\u{19b}\x03\x02\x02\x02\u{19a}\u{198}\x03\x02\x02\x02\
	\u{19b}\u{19c}\x07\x3b\x02\x02\u{19c}\u{1a0}\x03\x02\x02\x02\u{19d}\u{19e}\
	\x07\x3a\x02\x02\u{19e}\u{1a0}\x07\x3b\x02\x02\u{19f}\u{192}\x03\x02\x02\
	\x02\u{19f}\u{19d}\x03\x02\x02\x02\u{1a0}\x25\x03\x02\x02\x02\u{1a1}\u{1a2}\
	\x05\x1c\x0f\x02\u{1a2}\u{1a3}\x07\x38\x02\x02\u{1a3}\u{1a4}\x05\x1c\x0f\
	\x02\u{1a4}\x27\x03\x02\x02\x02\u{1a5}\u{1a6}\x07\x3a\x02\x02\u{1a6}\u{1ab}\
	\x05\x1c\x0f\x02\u{1a7}\u{1a8}\x07\x3e\x02\x02\u{1a8}\u{1aa}\x05\x1c\x0f\
	\x02\u{1a9}\u{1a7}\x03\x02\x02\x02\u{1aa}\u{1ad}\x03\x02\x02\x02\u{1ab}\
	\u{1a9}\x03\x02\x02\x02\u{1ab}\u{1ac}\x03\x02\x02\x02\u{1ac}\u{1ae}\x03\
	\x02\x02\x02\u{1ad}\u{1ab}\x03\x02\x02\x02\u{1ae}\u{1af}\x07\x3b\x02\x02\
	\u{1af}\u{1b3}\x03\x02\x02\x02\u{1b0}\u{1b1}\x07\x3a\x02\x02\u{1b1}\u{1b3}\
	\x07\x3b\x02\x02\u{1b2}\u{1a5}\x03\x02\x02\x02\u{1b2}\u{1b0}\x03\x02\x02\
	\x02\u{1b3}\x29\x03\x02\x02\x02\u{1b4}\u{1b5}\x05\x4a\x26\x02\u{1b5}\u{1b6}\
	\x07\x3f\x02\x02\u{1b6}\u{1b8}\x03\x02\x02\x02\u{1b7}\u{1b4}\x03\x02\x02\
	\x02\u{1b7}\u{1b8}\x03\x02\x02\x02\u{1b8}\u{1b9}\x03\x02\x02\x02\u{1b9}\
	\u{1ba}\x05\x4a\x26\x02\u{1ba}\x2b\x03\x02\x02\x02\u{1bb}\u{1bc}\x07\x24\
	\x02\x02\u{1bc}\u{1bd}\x05\x4a\x26\x02\u{1bd}\x2d\x03\x02\x02\x02\u{1be}\
	\u{1bf}\x07\x28\x02\x02\u{1bf}\u{1c0}\x07\x38\x02\x02\u{1c0}\u{1cb}\x07\
	\x4b\x02\x02\u{1c1}\u{1c2}\x07\x29\x02\x02\u{1c2}\u{1c3}\x07\x38\x02\x02\
	\u{1c3}\u{1cb}\x07\x4b\x02\x02\u{1c4}\u{1c5}\x07\x2a\x02\x02\u{1c5}\u{1c6}\
	\x07\x38\x02\x02\u{1c6}\u{1cb}\x07\x4b\x02\x02\u{1c7}\u{1c8}\x07\x2b\x02\
	\x02\u{1c8}\u{1c9}\x07\x38\x02\x02\u{1c9}\u{1cb}\x07\x4b\x02\x02\u{1ca}\
	\u{1be}\x03\x02\x02\x02\u{1ca}\u{1c1}\x03\x02\x02\x02\u{1ca}\u{1c4}\x03\
	\x02\x02\x02\u{1ca}\u{1c7}\x03\x02\x02\x02\u{1cb}\x2f\x03\x02\x02\x02\u{1cc}\
	\u{1cd}\x07\x2c\x02\x02\u{1cd}\u{1ce}\x07\x38\x02\x02\u{1ce}\u{1df}\x07\
	\x4a\x02\x02\u{1cf}\u{1d0}\x07\x2d\x02\x02\u{1d0}\u{1d1}\x07\x38\x02\x02\
	\u{1d1}\u{1df}\x07\x4a\x02\x02\u{1d2}\u{1d3}\x07\x2e\x02\x02\u{1d3}\u{1d4}\
	\x07\x38\x02\x02\u{1d4}\u{1df}\x07\x4a\x02\x02\u{1d5}\u{1d6}\x07\x2f\x02\
	\x02\u{1d6}\u{1d7}\x07\x38\x02\x02\u{1d7}\u{1d8}\x07\x3a\x02\x02\u{1d8}\
	\u{1df}\x07\x3b\x02\x02\u{1d9}\u{1da}\x07\x30\x02\x02\u{1da}\u{1db}\x07\
	\x38\x02\x02\u{1db}\u{1dc}\x07\x3a\x02\x02\u{1dc}\u{1df}\x07\x3b\x02\x02\
	\u{1dd}\u{1df}\x05\x2e\x18\x02\u{1de}\u{1cc}\x03\x02\x02\x02\u{1de}\u{1cf}\
	\x03\x02\x02\x02\u{1de}\u{1d2}\x03\x02\x02\x02\u{1de}\u{1d5}\x03\x02\x02\
	\x02\u{1de}\u{1d9}\x03\x02\x02\x02\u{1de}\u{1dd}\x03\x02\x02\x02\u{1df}\
	\x31\x03\x02\x02\x02\u{1e0}\u{1e4}\x07\x3a\x02\x02\u{1e1}\u{1e3}\x05\x30\
	\x19\x02\u{1e2}\u{1e1}\x03\x02\x02\x02\u{1e3}\u{1e6}\x03\x02\x02\x02\u{1e4}\
	\u{1e2}\x03\x02\x02\x02\u{1e4}\u{1e5}\x03\x02\x02\x02\u{1e5}\u{1e7}\x03\
	\x02\x02\x02\u{1e6}\u{1e4}\x03\x02\x02\x02\u{1e7}\u{1e8}\x07\x3b\x02\x02\
	\u{1e8}\x33\x03\x02\x02\x02\u{1e9}\u{1ea}\x07\x10\x02\x02\u{1ea}\u{1fc}\
	\x05\x4a\x26\x02\u{1eb}\u{1ec}\x07\x26\x02\x02\u{1ec}\u{1ed}\x07\x40\x02\
	\x02\u{1ed}\u{1ee}\x07\x41\x02\x02\u{1ee}\u{1f3}\x05\x32\x1a\x02\u{1ef}\
	\u{1f0}\x07\x3e\x02\x02\u{1f0}\u{1f2}\x05\x32\x1a\x02\u{1f1}\u{1ef}\x03\
	\x02\x02\x02\u{1f2}\u{1f5}\x03\x02\x02\x02\u{1f3}\u{1f1}\x03\x02\x02\x02\
	\u{1f3}\u{1f4}\x03\x02\x02\x02\u{1f4}\u{1f7}\x03\x02\x02\x02\u{1f5}\u{1f3}\
	\x03\x02\x02\x02\u{1f6}\u{1f8}\x07\x3e\x02\x02\u{1f7}\u{1f6}\x03\x02\x02\
	\x02\u{1f7}\u{1f8}\x03\x02\x02\x02\u{1f8}\u{1f9}\x03\x02\x02\x02\u{1f9}\
	\u{1fa}\x07\x42\x02\x02\u{1fa}\u{1fc}\x03\x02\x02\x02\u{1fb}\u{1e9}\x03\
	\x02\x02\x02\u{1fb}\u{1eb}\x03\x02\x02\x02\u{1fc}\x35\x03\x02\x02\x02\u{1fd}\
	\u{1fe}\x07\x10\x02\x02\u{1fe}\u{20f}\x05\x4a\x26\x02\u{1ff}\u{200}\x07\
	\x27\x02\x02\u{200}\u{201}\x07\x40\x02\x02\u{201}\u{202}\x07\x41\x02\x02\
	\u{202}\u{207}\x07\x4b\x02\x02\u{203}\u{204}\x07\x3e\x02\x02\u{204}\u{206}\
	\x07\x4b\x02\x02\u{205}\u{203}\x03\x02\x02\x02\u{206}\u{209}\x03\x02\x02\
	\x02\u{207}\u{205}\x03\x02\x02\x02\u{207}\u{208}\x03\x02\x02\x02\u{208}\
	\u{20b}\x03\x02\x02\x02\u{209}\u{207}\x03\x02\x02\x02\u{20a}\u{20c}\x07\
	\x3e\x02\x02\u{20b}\u{20a}\x03\x02\x02\x02\u{20b}\u{20c}\x03\x02\x02\x02\
	\u{20c}\u{20d}\x03\x02\x02\x02\u{20d}\u{20f}\x07\x42\x02\x02\u{20e}\u{1fd}\
	\x03\x02\x02\x02\u{20e}\u{1ff}\x03\x02\x02\x02\u{20f}\x37\x03\x02\x02\x02\
	\u{210}\u{211}\x07\x08\x02\x02\u{211}\u{212}\x05\x4a\x26\x02\u{212}\u{216}\
	\x07\x3a\x02\x02\u{213}\u{215}\x05\x3a\x1e\x02\u{214}\u{213}\x03\x02\x02\
	\x02\u{215}\u{218}\x03\x02\x02\x02\u{216}\u{214}\x03\x02\x02\x02\u{216}\
	\u{217}\x03\x02\x02\x02\u{217}\u{219}\x03\x02\x02\x02\u{218}\u{216}\x03\
	\x02\x02\x02\u{219}\u{21a}\x07\x3b\x02\x02\u{21a}\x39\x03\x02\x02\x02\u{21b}\
	\u{21c}\x05\x4a\x26\x02\u{21c}\u{21f}\x05\x20\x11\x02\u{21d}\u{21e}\x07\
	\x07\x02\x02\u{21e}\u{220}\x05\x4a\x26\x02\u{21f}\u{21d}\x03\x02\x02\x02\
	\u{21f}\u{220}\x03\x02\x02\x02\u{220}\u{221}\x03\x02\x02\x02\u{221}\u{222}\
	\x07\x39\x02\x02\u{222}\x3b\x03\x02\x02\x02\u{223}\u{224}\x07\x24\x02\x02\
	\u{224}\u{225}\x05\x3e\x20\x02\u{225}\x3d\x03\x02\x02\x02\u{226}\u{227}\
	\x07\x21\x02\x02\u{227}\u{228}\x05\x4a\x26\x02\u{228}\u{22c}\x07\x3a\x02\
	\x02\u{229}\u{22b}\x05\x34\x1b\x02\u{22a}\u{229}\x03\x02\x02\x02\u{22b}\
	\u{22e}\x03\x02\x02\x02\u{22c}\u{22a}\x03\x02\x02\x02\u{22c}\u{22d}\x03\
	\x02\x02\x02\u{22d}\u{22f}\x03\x02\x02\x02\u{22e}\u{22c}\x03\x02\x02\x02\
	\u{22f}\u{230}\x07\x3b\x02\x02\u{230}\u{247}\x03\x02\x02\x02\u{231}\u{232}\
	\x07\x20\x02\x02\u{232}\u{233}\x05\x4a\x26\x02\u{233}\u{234}\x07\x3a\x02\
	\x02\u{234}\u{235}\x07\x3b\x02\x02\u{235}\u{247}\x03\x02\x02\x02\u{236}\
	\u{237}\x07\x22\x02\x02\u{237}\u{238}\x05\x4a\x26\x02\u{238}\u{23c}\x07\
	\x3a\x02\x02\u{239}\u{23b}\x05\x36\x1c\x02\u{23a}\u{239}\x03\x02\x02\x02\
	\u{23b}\u{23e}\x03\x02\x02\x02\u{23c}\u{23a}\x03\x02\x02\x02\u{23c}\u{23d}\
	\x03\x02\x02\x02\u{23d}\u{23f}\x03\x02\x02\x02\u{23e}\u{23c}\x03\x02\x02\
	\x02\u{23f}\u{240}\x07\x3b\x02\x02\u{240}\u{247}\x03\x02\x02\x02\u{241}\
	\u{242}\x07\x23\x02\x02\u{242}\u{243}\x05\x4a\x26\x02\u{243}\u{244}\x07\
	\x3a\x02\x02\u{244}\u{245}\x07\x3b\x02\x02\u{245}\u{247}\x03\x02\x02\x02\
	\u{246}\u{226}\x03\x02\x02\x02\u{246}\u{231}\x03\x02\x02\x02\u{246}\u{236}\
	\x03\x02\x02\x02\u{246}\u{241}\x03\x02\x02\x02\u{247}\x3f\x03\x02\x02\x02\
	\u{248}\u{24a}\x07\x04\x02\x02\u{249}\u{24b}\x07\x4d\x02\x02\u{24a}\u{249}\
	\x03\x02\x02\x02\u{24a}\u{24b}\x03\x02\x02\x02\u{24b}\u{24c}\x03\x02\x02\
	\x02\u{24c}\u{250}\x07\x3a\x02\x02\u{24d}\u{24f}\x05\x42\x22\x02\u{24e}\
	\u{24d}\x03\x02\x02\x02\u{24f}\u{252}\x03\x02\x02\x02\u{250}\u{24e}\x03\
	\x02\x02\x02\u{250}\u{251}\x03\x02\x02\x02\u{251}\u{253}\x03\x02\x02\x02\
	\u{252}\u{250}\x03\x02\x02\x02\u{253}\u{254}\x07\x3b\x02\x02\u{254}\x41\
	\x03\x02\x02\x02\u{255}\u{256}\x07\x05\x02\x02\u{256}\u{259}\x05\x46\x24\
	\x02\u{257}\u{258}\x07\x06\x02\x02\u{258}\u{25a}\x05\x4a\x26\x02\u{259}\
	\u{257}\x03\x02\x02\x02\u{259}\u{25a}\x03\x02\x02\x02\u{25a}\u{25b}\x03\
	\x02\x02\x02\u{25b}\u{25c}\x07\x39\x02\x02\u{25c}\x43\x03\x02\x02\x02\u{25d}\
	\u{25e}\x07\x14\x02\x02\u{25e}\u{261}\x05\x18\x0d\x02\u{25f}\u{260}\x07\
	\x15\x02\x02\u{260}\u{262}\x05\x4a\x26\x02\u{261}\u{25f}\x03\x02\x02\x02\
	\u{261}\u{262}\x03\x02\x02\x02\u{262}\u{263}\x03\x02\x02\x02\u{263}\u{264}\
	\x07\x39\x02\x02\u{264}\x45\x03\x02\x02\x02\u{265}\u{266}\x05\x4a\x26\x02\
	\u{266}\u{268}\x07\x38\x02\x02\u{267}\u{269}\x05\x48\x25\x02\u{268}\u{267}\
	\x03\x02\x02\x02\u{268}\u{269}\x03\x02\x02\x02\u{269}\x47\x03\x02\x02\x02\
	\u{26a}\u{26b}\x05\x4a\x26\x02\u{26b}\x49\x03\x02\x02\x02\u{26c}\u{275}\
	\x05\x4c\x27\x02\u{26d}\u{26f}\x07\x43\x02\x02\u{26e}\u{26d}\x03\x02\x02\
	\x02\u{26f}\u{270}\x03\x02\x02\x02\u{270}\u{26e}\x03\x02\x02\x02\u{270}\
	\u{271}\x03\x02\x02\x02\u{271}\u{272}\x03\x02\x02\x02\u{272}\u{274}\x05\
	\x4c\x27\x02\u{273}\u{26e}\x03\x02\x02\x02\u{274}\u{277}\x03\x02\x02\x02\
	\u{275}\u{273}\x03\x02\x02\x02\u{275}\u{276}\x03\x02\x02\x02\u{276}\x4b\
	\x03\x02\x02\x02\u{277}\u{275}\x03\x02\x02\x02\u{278}\u{279}\x09\x04\x02\
	\x02\u{279}\x4d\x03\x02\x02\x02\x49\x51\x5c\x65\x72\x7c\u{8b}\u{8f}\u{9c}\
	\u{a6}\u{ac}\u{b0}\u{b4}\u{c1}\u{ca}\u{d4}\u{e6}\u{f6}\u{fd}\u{101}\u{107}\
	\u{116}\u{120}\u{122}\u{12b}\u{133}\u{138}\u{13d}\u{142}\u{147}\u{14c}\u{151}\
	\u{153}\u{157}\u{15a}\u{15f}\u{163}\u{168}\u{16c}\u{16f}\u{172}\u{177}\u{17b}\
	\u{181}\u{185}\u{18d}\u{198}\u{19f}\u{1ab}\u{1b2}\u{1b7}\u{1ca}\u{1de}\u{1e4}\
	\u{1f3}\u{1f7}\u{1fb}\u{207}\u{20b}\u{20e}\u{216}\u{21f}\u{22c}\u{23c}\u{246}\
	\u{24a}\u{250}\u{259}\u{261}\u{268}\u{270}\u{275}";

