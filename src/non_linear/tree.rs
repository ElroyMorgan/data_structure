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
	fn _in_order_traverse_ref<'a>(&'a self,result:&mut Vec<&'a T>){
		if let Some(left)=&self.left {
			left._in_order_traverse_ref(result);
		}
		result.push(self.get_data());
		if let Some(right) = &self.right {
			right._in_order_traverse_ref(result);
		}
	}
	pub fn in_order_traverse_mut(&self)->Vec<&T>{
		let mut result=Vec::new();
		self._in_order_traverse_ref(&mut result);
		result
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
	pub fn in_order_traverse(&self) -> Vec<&T> {
		let mut result = Vec::new();
		if let Some(root) = &self.0 {
			root._in_order_traverse_ref(&mut result);
		}
		result
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
	#[test]
	fn test_in_order_traversal() {
		// 构建测试用的二叉树:
		//       1
		//      / \
		//     2   3
		//    / \
		//   4   5

		let mut root = BinaryNode::new(1);

		// 先创建左子节点2
		root.left_mut(2);

		// 在左子节点2上添加左子节点4和右子节点5
		if let Some(left) = &mut root.left {
			left.left_mut(4).right_mut(5);
			//left.right_mut(5);
		}

		// 创建右子节点3
		root.right_mut(3);

		let mut tree = BinaryTree::with_root(1);
		tree.set_root(root);

		// 测试中序遍历
		let traversal = tree.in_order_traverse();
		assert_eq!(traversal, vec![&4, &2, &5, &1, &3]);
	}

	#[test]
	fn test_empty_tree_traversal() {
		let empty_tree = BinaryTree::<i32>::new();
		let traversal = empty_tree.in_order_traverse();
		assert!(traversal.is_empty());
	}

	#[test]
	fn test_single_node_traversal() {
		let tree = BinaryTree::with_root(42);
		let traversal = tree.in_order_traverse();
		assert_eq!(traversal, vec![&42]);
	}
}