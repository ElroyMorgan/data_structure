/// 基于向量的栈数据结构实现
///
/// 用于存储字符类型的数据，支持基本的栈操作：
/// - 压栈(push)
/// - 弹栈(pop)
/// - 查看栈顶(peek)
/// - 检查是否为空(is_empty)
/// - 获取当前长度(len)
pub struct Stack {
    data: Vec<char>,
    top: usize,
}

impl Stack {
    /// 创建一个新的栈实例
    ///
    /// # 参数
    /// * `length` - 栈的初始容量
    ///
    /// # 返回
    /// 返回一个初始化的栈实例
    pub fn new(length: usize) -> Stack {
        Stack {
            data: Vec::with_capacity(length),
            top: 0,
        }
    }
    
    /// 检查栈是否为空
    ///
    /// # 返回
    /// 如果栈为空则返回 `true`，否则返回 `false`
    pub fn is_empty(&self) -> bool {
        self.top == 0
    }
    
    /// 将元素压入栈顶
    ///
    /// # 参数
    /// * `c` - 要压入的字符
    ///
    /// # 返回
    /// 如果压入成功返回 `true`，如果栈已满返回 `false`
    pub fn push(&mut self, c: char) -> bool {
        if self.top == self.data.capacity() {
            return false;
        }
        self.data.push(c);
        self.top += 1;
        true
    }
    
    /// 从栈顶弹出元素
    ///
    /// # 返回
    /// 成功时返回 `Ok(弹出元素)`，栈为空时返回 `Err("此栈为空！")`
    pub fn pop(&mut self) -> Result<char, &str> {
        if self.is_empty() {
            return Err("此栈为空！");
        }
        self.top -= 1;
        let del = self.data.pop().unwrap();
        Ok(del)
    }
    
    /// 获取栈中元素数量
    ///
    /// # 返回
    /// 当前栈中的元素个数
    pub fn len(&self) -> usize {
        self.top
    }
    
    /// 查看栈顶元素（不可变引用）
    ///
    /// # 返回
    /// 成功时返回 `Ok(栈顶元素引用)`，栈为空时返回 `Err("此栈为空！")`
    pub fn peek(&self) -> Result<&char, &str> {
        if self.is_empty() {
            return Err("此栈为空！");
        }
        Ok(&self.data[self.top - 1])
    }
    
    /// 查看并修改栈顶元素（可变引用）
    ///
    /// # 返回
    /// 成功时返回 `Ok(栈顶元素可变引用)`，栈为空时返回 `Err("此栈为空！")`
    pub fn peek_mut(&mut self) -> Result<&mut char, &str> {
        if self.is_empty() {
            return Err("此栈为空！");
        }
        let index = self.top - 1;
        Ok(&mut self.data[index])
    }
}

/// 检查字符串中的括号是否匹配
///
/// # 参数
/// * `s` - 要检查的字符串
///
/// # 返回
/// 如果所有括号正确匹配返回 `true`，否则返回 `false`
pub fn brack_check(s: &String) -> bool {
    let mut s_stack = Stack::new(s.len());
    for c in s.chars() {
        if c == '(' || c == '[' || c == '{' {
            s_stack.push(c);
        } else {
            if s_stack.is_empty() {
                return false;
            }
            let top_elem = s_stack.pop().unwrap();
            if c == ')' && top_elem != '(' {
                return false;
            }
            if c == ']' && top_elem != '[' {
                return false;
            }
            if c == '}' && top_elem != '{' {
                return false;
            }
        }
    }
    s_stack.is_empty()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test2_stack() {
        let mut s=Stack::new(10);
        let mut is=s.is_empty();
        assert_eq!(true,is);
        s.push('Z');
        is=s.is_empty();
        assert_eq!(false,is);
        s.push('V');
        assert_eq!(s.len(),2);
        assert_eq!('V',s.pop().unwrap());
        
        // 测试 peek 和 peek_mut
        let mut s = Stack::new(5);
        s.push('A');
        s.push('B');
        assert_eq!(Ok(&'B'), s.peek());
        if let Ok(top) = s.peek_mut() {
            *top = 'C';
        }
        assert_eq!(Ok(&'C'), s.peek());
        assert_eq!('C', s.pop().unwrap());
        assert_eq!('A', s.pop().unwrap());
        assert_eq!(Err("此栈为空！"), s.peek());
        assert_eq!(Err("此栈为空！"), s.peek_mut());
    }
    #[test]
    fn test_brack_check(){
        let s=String::from("(({[]}))");
        assert_eq!(true,brack_check(&s));
    }
}