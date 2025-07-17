 pub mod enum_linklist{
        #[derive(Debug)]
        pub struct Node{
            elem:i32,
            next:Link,
        }

        impl Node{
            pub fn elem_ref(&self)->&i32{
                &self.elem
            }
            pub fn elem_mut(&mut self)->&mut i32{
                &mut self.elem
            }
            pub fn elem(&self)->i32{
                self.elem
            }
        }
        #[derive(Debug)]
        pub enum Link{
            Null,
            More(Box<Node>),
        }
        #[derive(Debug)]
        pub struct List{
            head:Link
        }
        impl List{
            pub fn new() -> Self {
                List { head: Link::Null } // 修复1: 初始化为空链表
            }
            pub fn push(&mut self,elem:i32){
                let new_node=Box::new(Node{
                    elem,
                    next:std::mem::replace(&mut self.head,Link::Null)
                });
                self.head=Link::More(new_node);
            }
            pub fn pop(&mut self)->Option<i32>{
                match std::mem::replace(&mut self.head,Link::Null) {
                    Link::Null=>None,
                    Link::More(node)=>{
                        self.head=node.next;
                        Some(node.elem)
                    }
                }
            }
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
                        //Link::More(ref mut node) => current = &mut node.next,
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
