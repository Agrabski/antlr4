mod tree;
mod Atn;
mod parser;
mod walker;
mod TokenStream;


#[cfg(test)]
mod tests 
{
	use crate::Atn::SemanticContext;
	use crate::tree::TerminalNode;
	use crate::tree::ParseTree;
	#[test]
	fn it_works()
	{
		let x = TerminalNode
		{
			token: crate::tree::Token 
			{
				text: "a".to_string(),
				_type: 5
			}
		};
		assert_eq!(x.text(),"a".to_string());
		assert_eq!(2 + 2, 4);
	}
}