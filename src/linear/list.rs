/// A module implementing a singly linked list using enum-based nodes.
/// 
/// Provides basic operations like push, pop, insert and delete.
pub mod enum_linklist{
        /// A node in the linked list containing an element and a link to the next node.
        #[derive(Debug)]
        pub struct Node{
            /// The element stored in this node
            elem:i32,
            /// Link to the next node in the list
            next:Link,
        }

        impl Node{
            /// Returns a reference to the element in this node
            pub fn elem_ref(&self)->&i32{
                &self.elem
            }
            /// Returns a mutable reference to the element in this node
            pub fn elem_mut(&mut self)->&mut i32{
                &mut self.elem
            }
            /// Returns the element value in this node
            pub fn elem(&self)->i32{
                self.elem
            }
        }
        /// Represents a link to another node or the end of the list
        #[derive(Debug)]
        pub enum Link{
            /// Represents the end of the list
            Null,
            /// Contains a boxed node pointing to the next element
            More(Box<Node>),
        }
        /// A singly linked list implementation using enum-based nodes
        #[derive(Debug)]
        pub struct List{
            /// The head of the linked list
            head:Link
        }
        impl List{
            /// Creates a new empty linked list
            pub fn new() -> Self {
                List { head: Link::Null }
            }
            /// Pushes an element onto the front of the list
            /// 
            /// # Arguments
            /// * `elem` - The element to add to the list
            pub fn push(&mut self,elem:i32){
                let new_node=Box::new(Node{
                    elem,
                    next:std::mem::replace(&mut self.head,Link::Null)
                });
                self.head=Link::More(new_node);
            }
            /// Removes and returns the first element of the list, if any
            /// 
            /// Returns `None` if the list is empty
            pub fn pop(&mut self)->Option<i32>{
                match std::mem::replace(&mut self.head,Link::Null) {
                    Link::Null=>None,
                    Link::More(node)=>{
                        self.head=node.next;
                        Some(node.elem)
                    }
                }
            }
            /// Inserts an element at the specified position in the list
            /// 
            /// # Arguments
            /// * `i` - 1-based index where to insert the element
            /// * `elem` - The element to insert
            /// 
            /// # Returns
            /// `true` if insertion was successful, `false` if index is invalid
            /// 
            /// # Panics
            /// Does not panic but returns false for invalid indices
            pub fn insert(&mut self,i:usize,elem:i32)->bool{
                if i<1 { return false; }
                let mut current=&mut self.head;
                let mut position=0;
                while position<i-1 {
                    match *current {
                        Link::More(ref mut node)=>{
                            current=&mut node.next;
                            position+=1;
                        }
                        Link::Null=>return false,
                    }
                }
                let next=std::mem::replace(current,Link::Null);
                *current=Link::More(Box::new(Node{
                    elem,
                    next,
                }));
                true
            }
            /// Deletes the element at the specified position in the list
            /// 
            /// # Arguments
            /// * `i` - 1-based index of the element to remove
            /// 
            /// # Returns
            /// `true` if deletion was successful, `false` if index is invalid
            pub fn delete(&mut self, i: usize) -> bool {
                if i < 1 {
                    return false;
                }
                if i == 1 {
                    return if let Link::More(node) = std::mem::replace(&mut self.head, Link::Null) {
                        self.head = node.next;
                        true
                    } else {
                        false
                    }
                }
                let mut current = &mut self.head;
                for _ in 0..i-2 {
                    match current {
                        Link::More(node) => current = &mut node.next,
                        Link::Null => return false,
                    }
                }
                match current {
                    Link::More(prev_node) => {
                        let target = std::mem::replace(&mut prev_node.next, Link::Null);
                        if let Link::More(node) = target {
                            prev_node.next = node.next;
                            true
                        } else {
                            false
                        }
                    }
                    Link::Null => false,
                }
            }
        }
    }
 #[cfg(test)]
 mod tests {
     use super::enum_linklist::List;

     #[test]
     fn test_insert_enum_link_list() {
         let mut list = List::new();
         assert!(list.insert(1, 10)); // 插入到空链表头部
         assert_eq!(list.pop(), Some(10));

         list.push(30);
         list.push(10); // 10 -> 30
         assert!(list.insert(2, 20)); // 插入到中间 10 ->20 ->30
         assert_eq!(list.pop(), Some(10));
         assert_eq!(list.pop(), Some(20));
         assert_eq!(list.pop(), Some(30));

         assert!(!list.insert(0, 0)); // 无效索引
         list.push(1);
         assert!(!list.insert(3, 2)); // 索引越界
     }

     #[test]
     fn test_delete_enum_link_list() {
         let mut list = List::new();
         assert!(!list.delete(1)); // 空链表删除失败

         list.push(3);
         list.push(2);
         list.push(1); // 1->2->3
         assert!(list.delete(2)); // 删除中间节点 1->3
         assert_eq!(list.pop(), Some(1));
         assert_eq!(list.pop(), Some(3));

         list.push(5);
         assert!(list.delete(1)); // 删除头节点
         assert_eq!(list.pop(), None);

         list.push(1);
         assert!(!list.delete(2)); // 索引越界
     }
 }
