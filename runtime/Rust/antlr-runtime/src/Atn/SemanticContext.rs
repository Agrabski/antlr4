use crate::parser::Grammar;

pub trait SemanticContext<RuleEnum, G: Grammar<RuleEnum>>
{
	fn eval(&self, grammar: G, predicateIndex: i64) -> bool;
}
