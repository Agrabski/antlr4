mod tree;


#[cfg(test)]
mod tests 
{
	use crate::tree::TerminalNode;
	use crate::tree::ParseTree;
	#[test]
	fn it_works()
	{
		let x = TerminalNode
		{
			token: crate::tree::Token 
			{
				text: "a".to_string()
			}
		};
		assert_eq!(x.text(),"a".to_string());
		assert_eq!(2 + 2, 4);
	}
}