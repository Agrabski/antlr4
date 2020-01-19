use crate::tree::Token;

pub struct TokenStream
{
	_tokens : Vec<Token>;
	_currentTokenIndex : u64;
}

impl TokenStream
{
	fn LT(&mut self, k: i64) -> Option<Token>
	{
		if(k == 0)
			return None;
		if(k < 0)
			return LB(-k);
		let mut i = _currentTokenIndex;
		let mut n = 1;
		while(n<k)
			if(sync(i+1))
				i = nextTokenOnChannel(i+1, channel);
			n++;
		return _tokens[i];
	}

	fn LB(&mut self, k: i64) -> Option<Token>
	{

	}

	fn nextTokenOnChannel(i: i64, channel: i64) -> u64
	{
		sync(i);
		if (i >= size())
			return size() - 1;
		let mut token = _tokens[i];
		while(token->getChannel()!=channel)
		{
			if(token->getType()==Token::EOF)
				return i;
			i++;
			sync(i);
			token = _tokens[i];
		}
		return i;
	}

	fn sync(i: i64) -> bool
	{
		
	}

}