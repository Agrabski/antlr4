pub trait ParseTree
{
	fn children(& mut self) -> Option<&mut Vec<Box<dyn ParseTree>>>;
	fn children_const(& self) -> Option<&Vec<Box<dyn ParseTree>>>;
	fn text(&self) -> String;
}


pub struct TerminalNode
{
	pub token: Token
}

impl ParseTree for TerminalNode
{
	fn children(& mut self) -> Option<&mut Vec<Box<dyn ParseTree>>>
	{
		return None;
	}

	fn children_const(& self) -> Option<&Vec<Box<dyn ParseTree>>>
	{
		return None;
	}

	fn text(&self) -> String
	{
		return self.token.text.clone();
	}
}

pub struct Token
{
	pub text: String,
	_type: i64
}

impl Token
{
	fn new(text : &String, type: i64) -> Token
	{
		return Token
		{
			text: text.clone(),
			_type: type
		};
	}

	fn getType(self&) -> i64
	{
		return self._type;
	}
}