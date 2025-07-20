use std::ptr::null_mut;

/// 使用原始指针实现的链表节点
/// 
/// # 注意
/// 这是一个不安全的实现，使用了原始指针(raw pointer)来管理内存
/// 需要手动管理内存释放，通过实现Drop trait来确保不会内存泄漏
pub struct NodeRaw {
    /// 节点存储的数据
    pub data: i32,
    /// 指向下一个节点的原始指针
    next: *mut NodeRaw,
}

impl NodeRaw {
    /// 创建一个新的链表头节点
    /// 
    /// # 返回值
    /// 返回一个初始化的NodeRaw实例，data为0，next指针为null
    pub fn new() -> Self {
        Self { data: 0, next: std::ptr::null_mut() }
    }

    /// 在指定位置插入新节点
    /// 
    /// # 参数
    /// - `i`: 要插入的位置(从1开始)
    /// - `e`: 要插入的数据
    /// 
    /// # 返回值
    /// - Ok(()) 插入成功
    /// - Err(String) 插入失败，包含错误信息
    /// 
    /// # 错误
    /// - 如果位置i小于1，返回"位序从1开始"
    /// - 如果位置i超过链表长度，返回"插入位置超出链表长度"
    pub fn insert(&mut self, i: isize, e: i32) -> Result<(), String> {
        if i < 1 {
            return Err("位序从1开始".to_string());
        }
        
        // 找到插入位置的前一个节点
        let mut current = self as *mut NodeRaw;
        
        // 对于位置i，我们需要找到第i-1个节点作为插入点的前驱
        // 头节点算作位置0，第一个数据节点是位置1
        for _ in 1..i {
            unsafe {
                if (*current).next.is_null() {
                    return Err("插入位置超出链表长度".to_string());
                }
                current = (*current).next;
            }
        }
        
        // 创建新节点
        let new_node = Box::new(NodeRaw {
            data: e,
            next: std::ptr::null_mut(),
        });
        let new_node_ptr = Box::into_raw(new_node);
        
        // 插入新节点
        unsafe {
            (*new_node_ptr).next = (*current).next;
            (*current).next = new_node_ptr;
        }
        
        Ok(())
    }

    /// 删除指定位置的节点
    /// 
    /// # 参数
    /// - `i`: 要删除的位置(从1开始)
    /// 
    /// # 返回值
    /// - Some(`Box<NodeRaw>`) 删除成功，返回被删除的节点
    /// - None 删除失败(位置无效或超出链表长度)
    pub fn delete(&mut self, i: isize) -> Option<Box<NodeRaw>> {
        if i < 1 {
            return None;
        }
        let mut current = &raw mut *self;
        let mut pos = 0;
        while pos < i - 1 {
            unsafe {
                if current.is_null() || (*current).next.is_null() {
                    return None;
                }
                current = (*current).next;
            }
            pos += 1;
        }
        let del = unsafe {
            let del = (*current).next;
            (*current).next = (*del).next;
            (*del).next = null_mut();
            let del = Box::from_raw(del);
            del
        };
        Some(del)
    }
    pub fn get_ref(&mut self,i:isize)->Option<&i32>{
        if i<1 {
            return None
        }
        let mut current=&raw mut *self;
        for _ in 0..i {
            unsafe {
                if current.is_null() {
                    return None;
                }
                current =(*current).next;
            }
        }
        unsafe {
            if current.is_null() {
                return None;
            }
            let curr=&(*current).data;
            Some(curr)
        }
        
    }
    
    pub fn get_mut(&mut self,i:isize)->Option<&mut i32>{
        if i<1 {
            return None
        }
        let mut current=&raw mut *self;
        for _ in 0..i {
            unsafe {
                if current.is_null() {
                    return None;
                }
                current =(*current).next;
            }
        }
        unsafe {
            if current.is_null() {
                return None;
            }
            let curr=&mut (*current).data;
            Some(curr)
        }
        
    }

    pub fn insert_next_node(node:*mut NodeRaw,e:i32)->Result<(),LinkErr> {
        if node.is_null() {
                return Err(LinkErr::Node);
            }
        let new_node=Box::into_raw(Box::new(NodeRaw{
            data:e,
            next:std::ptr::null_mut(),
        }));
        unsafe {
            (*new_node).next=(*node).next;
            (*node).next=new_node;
        }
        Ok(())
    }
}
#[derive(Debug)]
pub enum LinkErr {
    Node,
    Index,
}
/// 测试模块
#[cfg(test)]
mod tests {
    use super::*;

    /// 测试新建链表
    #[test]
    fn test_new_list() {
        let list = NodeRaw::new();
        assert_eq!(list.data, 0);
        assert!(list.next.is_null());
    }

    /// 测试在头部插入节点
    #[test]
    fn test_insert_at_head() {
        let mut list = NodeRaw::new();
        list.insert(1, 10).unwrap();
        unsafe {
            assert_eq!((*list.next).data, 10);
            assert!((*list.next).next.is_null());
        }
    }

    /// 测试在指定位置插入节点
    #[test]
    fn test_insert_at_position() {
        let mut list = NodeRaw::new();
        list.insert(1, 10).unwrap();
        list.insert(2, 20).unwrap();
        unsafe {
            assert_eq!((*list.next).data, 10);
            assert_eq!((*(*list.next).next).data, 20);
        }
    }

    /// 测试在无效位置插入节点
    #[test]
    fn test_insert_invalid_position() {
        let mut list = NodeRaw::new();
        assert!(list.insert(0, 10).is_err());
        assert!(list.insert(2, 20).is_err());
    }

    /// 测试删除节点
    #[test]
    fn test_delete_node() {
        let mut list = NodeRaw::new();
        list.insert(1, 10).unwrap();
        list.insert(2, 20).unwrap();
        let deleted = list.delete(1).unwrap();
        assert_eq!(deleted.data, 10);
        unsafe {
            assert_eq!((*list.next).data, 20);
        }
    }

    /// 测试在无效位置删除节点
    #[test]
    fn test_delete_invalid_position() {
        let mut list = NodeRaw::new();
        assert!(list.delete(0).is_none());
        assert!(list.delete(2).is_none());
    }

    /// 测试获取节点引用
    #[test]
    fn test_get_ref() {
        let mut list = NodeRaw::new();
        list.insert(1, 10).unwrap();
        list.insert(2, 20).unwrap();
        list.insert(3, 30).unwrap();

        // 测试获取第一个节点
        let first = list.get_ref(1).unwrap();
        assert_eq!(*first, 10);

        // 测试获取第二个节点
        let second = list.get_ref(2).unwrap();
        assert_eq!(*second, 20);

        // 测试获取第三个节点
        let third = list.get_ref(3).unwrap();
        assert_eq!(*third, 30);

        // 测试获取超出链表长度的位置
        assert!(list.get_ref(4).is_none());

        // 测试获取位置小于1的情况
        assert!(list.get_ref(0).is_none());
    }
}

/// 实现Drop trait以确保正确释放链表内存
impl Drop for NodeRaw {
    /// 释放链表中的所有节点
    /// 
    /// 这个方法会在NodeRaw被丢弃时自动调用
    /// 它会遍历整个链表并安全释放每个节点
    fn drop(&mut self) {
        // 迭代释放链表中的所有节点
        while !self.next.is_null() {
            unsafe {
                let node_to_drop = self.next;
                self.next = (*node_to_drop).next;
                // 先断开连接，再释放
                (*node_to_drop).next = std::ptr::null_mut();
                drop(Box::from_raw(node_to_drop));
            }
        }
    }
}