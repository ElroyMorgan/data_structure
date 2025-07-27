use std::ptr;
// 定义栈节点结构
#[derive(Debug)]
pub struct Node<T> {
    pub data: T,
    next: *mut Node<T>,
}

// 链栈结构
#[derive(Debug)]
pub struct LinkStack<T> {
    top: *mut Node<T>,
    size: usize,
}

impl<T> LinkStack<T> {
    // 创建空栈
    pub fn new() -> Self {
        LinkStack {
            top: ptr::null_mut(),
            size: 0,
        }
    }

    // 判断栈是否为空
    pub fn is_empty(&self) -> bool {
        self.top.is_null()
    }

    // 获取栈大小
    pub fn size(&self) -> usize {
        self.size
    }

    // 入栈操作
    pub fn push(&mut self, data: T) {
        // 创建新节点
        let new_node = Box::into_raw(Box::new(Node {
            data,
            next: self.top,
        }));

        // 更新栈顶指针
        self.top = new_node;
        self.size += 1;
    }

    // 出栈操作
    pub fn pop(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }

        // 安全地解引用裸指针
        unsafe {
            let top_node = Box::from_raw(self.top);
            self.top = top_node.next;
            self.size -= 1;
            Some(top_node.data)
        }
    }
}

// 实现Drop trait释放内存
impl<T> Drop for LinkStack<T> {
    fn drop(&mut self) {
        while let Some(_) = self.pop() {
            // 持续弹出直到栈为空
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_stack() {
        let stack: LinkStack<i32> = LinkStack::new();
        assert!(stack.is_empty());
        assert_eq!(stack.size(), 0);
    }

    #[test]
    fn test_push_and_size() {
        let mut stack = LinkStack::new();
        stack.push(1);
        assert!(!stack.is_empty());
        assert_eq!(stack.size(), 1);

        stack.push(2);
        stack.push(3);
        assert_eq!(stack.size(), 3);
    }

    #[test]
    fn test_pop_order() {
        let mut stack = LinkStack::new();
        stack.push(1);
        stack.push(2);
        stack.push(3);

        assert_eq!(stack.pop(), Some(3));
        assert_eq!(stack.pop(), Some(2));
        assert_eq!(stack.size(), 1);

        stack.push(4);
        assert_eq!(stack.pop(), Some(4));
        assert_eq!(stack.pop(), Some(1));
        assert_eq!(stack.pop(), None);
        assert!(stack.is_empty());
    }

    #[test]
    fn test_pop_empty_stack() {
        let mut stack: LinkStack<String> = LinkStack::new();
        assert_eq!(stack.pop(), None);
    }

    #[test]
    fn test_drop_behavior() {
        // 测试Drop trait是否正确释放内存
        // 通过重复创建和销毁大栈来检测内存泄漏
        for _ in 0..1000 {
            let mut stack = LinkStack::new();
            for i in 0..1000 {
                stack.push(i);
            }
            // 栈超出作用域时应自动释放所有节点
        }
    }

    #[test]
    fn test_generic_data_types() {
        let mut string_stack = LinkStack::new();
        string_stack.push(String::from("hello"));
        string_stack.push(String::from("world"));
        assert_eq!(string_stack.pop(), Some(String::from("world")));

        let mut float_stack = LinkStack::new();
        float_stack.push(3.14);
        float_stack.push(2.71);
        assert_eq!(float_stack.pop(), Some(2.71));
    }
}