use std::ops::{Deref, DerefMut};

pub struct BinaryNode<T>{
	data:T,
	left:Option<Box<BinaryNode<T>>>,
	right:Option<Box<BinaryNode<T>>>,
}
impl<T> BinaryNode<T>
where T:Clone,
{
	pub fn new(data:T)->Self{
		Self{
			data,
			left:None,
			right:None,
		}
	}
	pub fn left(&mut self,data:T)->&Self{
		self.left = Some(Box::new(BinaryNode::new(data)));
		self
	}
	pub fn right(&mut self,data:T)->&Self{
		self.right = Some(Box::new(BinaryNode::new(data)));
		self
	}
	pub fn left_mut(&mut self,data:T)->&mut Self{
		self.left = Some(Box::new(BinaryNode::new(data)));
		self
	}
	pub fn right_mut(&mut self,data:T)->&mut Self{
		self.right = Some(Box::new(BinaryNode::new(data)));
		self
	}
	pub fn node(&mut self)->&mut Self{
		self
	}
	pub fn get_data(&self)->&T{
		&self.data
	}
	pub fn mut_data(&mut self)->&mut T{
		&mut self.data
	}
}
pub struct BinaryTree<T>(Option<Box<BinaryNode<T>>>);
impl<T: Clone> BinaryTree<T>{
	pub fn new() -> BinaryTree<T>{
		BinaryTree(None)
	}
	pub fn set_root(&mut self,root:BinaryNode<T>){
		self.0=Some(Box::new(root));
	}
	pub fn with_root(data:T)->BinaryTree<T>{
		BinaryTree(Some(Box::new(BinaryNode::new(data))))
	}
	pub fn if_empty(&self)->bool{
		self.0.is_none()
	}
}
impl<T> Deref for BinaryTree<T>{
	type Target = Option<Box<BinaryNode<T>>>;
	fn deref(&self) -> &Self::Target {
		&self.0
	}
}
impl<T> DerefMut for BinaryTree<T> {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.0
	}
}
#[cfg(test)]
mod tests{
	use crate::non_linear::tree::{BinaryNode, BinaryTree};

	#[test]
	fn test_binary_tree(){
		let mut tree = BinaryTree::new();
		assert_eq!(tree.if_empty(), true);
		tree.set_root(BinaryNode::new(1));
		let value=tree.0.as_ref().unwrap().data;
		assert_eq!(value,1);
		tree.0.as_mut().unwrap().left(2);
		let next_value=tree.0.as_ref().unwrap().left.as_ref().unwrap().data;
		assert_eq!(next_value,2);
		// if let Some(node)=tree.0{
		// 	if let Some(left)=node.left{
		// 		assert_eq!(left.data,2);
		// 	}
		// }
		if let Some(root)=tree.as_mut(){
			root.left.as_mut().unwrap().left(5);
			let next2=root.left.as_mut().unwrap().left.as_mut().unwrap().data;
			assert_eq!(next2,5)
		}
	}
}