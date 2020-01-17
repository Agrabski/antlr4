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
	pub text: String
}

impl Token
{
	fn new(text : &String) -> Token
	{
		return Token
		{
			text: text.clone()
		};
	}
}