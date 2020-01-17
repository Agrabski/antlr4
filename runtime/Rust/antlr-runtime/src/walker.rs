mod tree;


trait ParseTreeWalker
{
	fn walk(&self, &mut Box<dyn tree::ParseTree>);
}
