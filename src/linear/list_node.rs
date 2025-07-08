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

#[cfg(test)]
mod tests {
    use super::ListNode;
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
