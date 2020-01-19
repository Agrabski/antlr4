use crate::tree::ParseTree;

trait ParseTreeWalker
{
	fn walk(&self, tree: &mut Box<dyn ParseTree>);
}
