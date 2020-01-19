use crate::tree::ParseTree;
use crate::tree::Token;
use crate::TokenStream::TokenStream;

pub trait Grammar<RuleEnum>
{
	fn sempred(&self, rule: RuleEnum, predicateIndex: i64) -> bool;
}


pub struct ParserState
{
	_tokenSource : Box<mut TokenStream>
}

impl ParserState
{
	fn setState(&mut self, atnState: i64)
	fn consume(&self, context: Box<dyn ParseTree> ) -> Token
	{
		let token = _tokenSource->LT(1);
	}
}


