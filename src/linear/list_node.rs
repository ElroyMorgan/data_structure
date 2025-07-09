/// 链表节点结构体
///
/// # 示例
/// ```
/// use data_structure::linear::list_node::ListNode;
/// let mut head = ListNode::<i32>::new();
/// head.push(1);
/// head.push(2);
/// ```
pub struct ListNode<T> {
    /// 节点存储的数据，哨兵节点为None
    pub data: Option<T>,
    /// 指向下一个节点的指针
    pub next: Option<Box<ListNode<T>>>,
}

impl<T> ListNode<T> {
    /// 创建一个新的哨兵节点(不包含数据)
    ///
    /// # 示例
    /// ```
    /// use data_structure::linear::list_node::ListNode;
    /// let sentinel = ListNode::<i32>::new();
    /// assert!(sentinel.data.is_none());
    /// ```
    pub fn new() -> Self {
        ListNode {
            data: None,
            next: None,
        }
    }

    /// 获取指定索引(0-based)节点的不可变引用
    /// 如果索引越界则返回None
    ///
    /// # 示例
    /// ```
    /// use data_structure::linear::list_node::ListNode;
    /// let mut head = ListNode::new();
    /// head.push(42);
    /// assert_eq!(head.get(1).unwrap().data, Some(42));
    /// ```
    pub fn get(&self, index: usize) -> Option<&Self> {
        let mut current = self;
        for _ in 0..index {
            current = current.next.as_ref()?;
        }
        Some(current)
    }

    /// 获取指定索引(0-based)节点的可变引用
    /// 如果索引越界则返回None
    ///
    /// # 示例
    /// ```
    /// use data_structure::linear::list_node::ListNode;
    /// let mut head = ListNode::new();
    /// head.push(42);
    /// *head.get_mut(1).unwrap().data.as_mut().unwrap() = 24;
    /// ```
    pub fn get_mut(&mut self, index: usize) -> Option<&mut Self> {
        let mut current = self;
        let mut count = 0;
        // 边界检查：当 index 为 0 时直接返回当前节点
        if index == 0 {
            return Some(current);
        }
        while count < index - 1 {
            // 使用 as_mut()? 自动处理 None 情况
            current = current.next.as_mut()?;
            count += 1;
        }
        // 最后一次移动需要特别检查
        current.next.as_mut().map(|node| &mut **node)
    }

    /// 删除链表尾部节点
    ///
    /// # 示例
    /// ```
    /// use data_structure::linear::list_node::ListNode;
    /// let mut head = ListNode::new();
    /// head.push(1);
    /// head.push(2);
    /// head.pop_tail();
    /// assert_eq!(head.length(), 1);
    /// ```
    pub fn pop_tail(&mut self) {
        if self.next.is_none() {
            return;
        }
        
        let mut current = self;
        while current.next.as_ref().unwrap().next.is_some() {
            current = current.next.as_mut().unwrap();
        }
        current.next = None;
    }

    /// 在链表尾部添加新节点
    ///
    /// # 示例
    /// ```
    /// use data_structure::linear::list_node::ListNode;
    /// let mut head = ListNode::new();
    /// head.push(42);
    /// assert!(head.next.is_some());
    /// ```
    pub fn push(&mut self, data: T) {
        let mut tail = self;
        while tail.next.is_some() {
            tail = tail.next.as_mut().unwrap();
        }
        tail.next = Some(Box::new(ListNode {
            data: Some(data),
            next: None,
        }));
    }

    /// 在指定位置插入新节点
    ///
    /// # 示例
    /// ```
    /// use data_structure::linear::list_node::ListNode;
    /// let mut head = ListNode::new();
    /// head.push(1);
    /// head.push(3);
    /// head.insert(1, 2);
    /// assert_eq!(head.get(1).unwrap().data, Some(2));
    /// ```
    pub fn insert(&mut self, index: usize, data: T){
        let current= self.get_mut(index-1).unwrap();
        let new_node=Box::new(ListNode{
           data:Some(data), 
            //take临时取出current.next的所有权
            next:current.next.take(),
        });
         current.next=Some(new_node);
    }

    /// 删除指定位置的节点
    ///
    /// # 示例
    /// ```
    /// use data_structure::linear::list_node::ListNode;
    /// let mut head = ListNode::new();
    /// head.push(1);
    /// head.push(2);
    /// head.remove(1);
    /// assert_eq!(head.length(), 1);
    /// ```
    pub fn remove(&mut self, index: usize)  {
        let current = self.get_mut(index - 1).unwrap();
        let removed_node = current.next.take().unwrap();
        current.next = removed_node.next;
    }

    /// 获取链表长度(不包含哨兵节点)
    ///
    /// # 示例
    /// ```
    /// use data_structure::linear::list_node::ListNode;
    /// let mut head = ListNode::new();
    /// head.push(1);
    /// head.push(2);
    /// assert_eq!(head.length(), 2);
    /// ```
    pub fn length(&mut self)-> usize{
        let mut count:usize = 0;
        let mut current = self;
        while current.next.is_some(){
            count+=1;
            current=current.next.as_mut().unwrap();
        }
        count
    }
}
pub mod circular_list {
    use std::mem::swap;

    type ElemType=i32;
    //type LinkList=Box<LNode>;
    ///带头结点的单链表结构体
    pub struct LNode {
        pub data: ElemType,
        pub next: *mut LNode,
    }
    ///```rust
    ///use data_structure::linear::list_node::circular_list;
    ///use data_structure::linear::list_node::circular_list::{init_list, LNode};
    ///let mut list:*mut LNode=std::ptr::null_mut();
    /// unsafe {init_list(&mut list);}
    ///```
    type LinkList=*mut LNode;
    /// 初始化一个带头节点的单链表
    ///
    /// # 参数
    /// - `list`: 一个指向链表头节点的可变引用指针
    ///
    /// # 返回值
    /// 如果初始化成功返回 `true`，否则返回 `false`
    ///
    /// # 示例
    /// ```rust
    /// use data_structure::linear::list_node::circular_list::{init_list, LNode};
    /// let mut list: *mut LNode = std::ptr::null_mut();
    /// unsafe {
    ///     assert!(init_list(&mut list));
    ///     assert!(!list.is_null());
    ///     assert_eq!((*list).data, 0);
    /// }
    /// ```
    pub unsafe fn init_list(list:&mut *mut LNode)->bool{
        *list=Box::into_raw(Box::new(LNode{
            data:0,
            next:std::ptr::null_mut(),
        }));
        if (*list).is_null() {
            false
        }else {
            unsafe {
                (**list).next= std::ptr::null_mut();
                true
            }
        }
    }
    /// 在循环链表中插入一个新节点
    ///
    /// # 参数
    /// - `list`: 一个指向链表头节点的可变引用指针
    /// - `i`: 插入位置的索引（从1开始）
    /// - `e`: 要插入的数据
    ///
    /// # 返回值
    /// 如果插入成功返回 `true`，否则返回 `false`
    ///
    /// # 示例
    /// ```rust
    /// use data_structure::linear::list_node::circular_list::{init_list, list_insert, LNode};
    /// let mut list: *mut LNode = std::ptr::null_mut();
    /// unsafe {
    ///     init_list(&mut list);
    ///     assert!(list_insert(&mut list, 1, 42));
    ///     assert_eq!((*(*list).next).data, 42);
    /// }
    /// ```
    pub fn list_insert(list: &mut LinkList, i:usize, e:ElemType) ->bool{
        let node=Box::into_raw(Box::new(LNode{
            data:e,
            next:std::ptr::null_mut(),
        }));
        if i==0{ return false; }
        let mut current=*list;
        for _ in 0..i-1 {
          unsafe {
              current=(*current).next;
          }
        }
        unsafe {
            (*node).next=(*current).next;
            (*current).next=node;
        }
        true
    }
    /// 删除循环链表中的指定节点
    ///
    /// # 参数
    /// - `list`: 一个指向链表头节点的可变引用指针
    /// - `i`: 要删除节点的位置索引（从1开始）
    ///
    /// # 返回值
    /// 返回被删除的节点包装在 `Box` 中
    ///
    /// # 示例
    /// ```rust
    /// use data_structure::linear::list_node::circular_list::{init_list, list_insert, delete, LNode};
    /// let mut list: *mut LNode = std::ptr::null_mut();
    /// unsafe {
    ///     init_list(&mut list);
    ///     list_insert(&mut list, 1, 10);
    ///     list_insert(&mut list, 2, 20);
    ///     let deleted_node = delete(&mut list, 1);
    ///     assert_eq!(deleted_node.data, 10);
    ///     assert_eq!((*(*list).next).data, 20);
    /// }
    /// ```
    pub fn delete(list:&mut LinkList,i:usize)->Box<LNode>{
        let mut current=*list;
        for _ in 0..i {
            unsafe {
                if current.is_null() {
                    panic!("Index out of bounds");
                }
                current=(*current).next;
            }
        }
        unsafe {
            if current.is_null() || (*current).next.is_null() {
                panic!("Invalid node to delete (null or last node)");
            }
            swap(&mut (*current).data,&mut (*(*current).next).data);
            let value=Box::from_raw((*current).next);
            // value.next= std::ptr::null_mut();
            (*current).next=(*(*current).next).next;
            value
        }
    }

}
#[cfg(test)]
mod tests {
    use super::{ListNode, circular_list::{init_list, list_insert, delete, LNode}};

    #[test]
    fn test_init_list() {
        let mut list: *mut LNode = std::ptr::null_mut();
        unsafe {
            assert!(init_list(&mut list));
            assert!(!list.is_null());
            assert_eq!((*list).data,0);
            assert_eq!((*list).next, std::ptr::null_mut());
        }
    }

    #[test]
    fn test_list_insert() {
        let mut list: *mut LNode = std::ptr::null_mut();
        unsafe {
            init_list(&mut list);
            assert!(list_insert(&mut list, 1, 42));
            assert_eq!((*(*list).next).data, 42);
        }
    }

    #[test]
    fn test_delete() {
        let mut list: *mut LNode = std::ptr::null_mut();
        unsafe {
            init_list(&mut list);
            list_insert(&mut list, 1, 10);
            list_insert(&mut list, 2, 20);
            list_insert(&mut list, 3, 30);

            let deleted_node = delete(&mut list, 1);
            assert_eq!(deleted_node.data, 10);
            assert_eq!((*(*list).next).data, 20);
        }
    }

    #[test]
    fn test_sentinel_head() {
        let mut head: Box<ListNode<i32>> = Box::new(ListNode::new());
        for i in 1..6 {
            head.push(i);
        }
        assert!(head.data.is_none());
        assert_eq!(head.next.unwrap().data, Some(1))
    }
    #[test]
    fn test_get_mut() {
        // 建议恢复被注释的测试代码
        let mut head: Box<ListNode<i32>> = Box::new(ListNode::new());
        for i in 1..6 {
            head.push(i);
        }
        let value = head.get_mut(1).unwrap();
        assert_eq!(value.data, Some(1));
        // 恢复测试代码：
        let value2 = head.get_mut(2).unwrap();
        assert_eq!(value2.data, Some(2));
        let zero = head.get(0).unwrap();
        assert!(zero.data.is_none());
    }
    #[test]
    fn test_length(){
         let mut list: Box<ListNode<i32>> = Box::new(ListNode::new());
        list.push(1);
        list.push(2);
        list.push(3);
        list.push(4);
        assert_eq!( list.length(),4);
    }

}
