/// 顺序栈实现
/// 
/// 使用 Vec 作为底层存储，提供栈的基本操作
/// 
/// # 泛型参数
/// - `T`: 栈中元素的类型
pub struct SequentialStack<T>{
    capacity:usize,
    data:Vec<T>,
    top:usize,
}

impl<T> SequentialStack<T>{
    /// 创建一个新的顺序栈
    /// 
    /// # 参数
    /// - `capacity`: 栈的初始容量
    /// 
    /// # 返回值
    /// 返回一个新的 SequentialStack 实例
    pub fn new(capacity:usize)->SequentialStack<T>{
        SequentialStack { capacity, data:Vec::with_capacity(capacity), top:0 }
    }
    /// 检查栈是否为空
    /// 
    /// # 返回值
    /// 如果栈为空返回 true，否则返回 false
    pub fn is_empty(&self)->bool{
        self.top==0
    }
    /// 检查栈是否已满
    /// 
    /// # 返回值
    /// 如果栈已满返回 true，否则返回 false
    pub fn is_full(&self)->bool{
        self.top>=self.capacity
    }
    /// 将元素压入栈顶
    /// 
    /// # 参数
    /// - `item`: 要压入栈的元素
    /// 
    /// # 返回值
    /// - Ok(()) 表示成功
    /// - Err("栈满！！！") 如果栈已满
    pub fn push(&mut self,item:T)->Result<(),&'static str>{
        if self.is_full(){
            return Err("栈满！！！");
        }
        self.data.push(item);
        self.top +=1;
        Ok(())
    }
    /// 弹出栈顶元素
    /// 
    /// # 返回值
    /// - Ok(T) 包含弹出的元素
    /// - Err("空栈！！！") 如果栈为空
    pub fn pop(&mut self)->Result<T,&'static str>{
        if self.is_empty() {
            return Err("空栈！！！");
        }
        self.top -=1;
        Ok(self.data.pop().unwrap())
    }
    /// 获取栈顶元素的不可变引用
    /// 
    /// # 返回值
    /// - Some(&T) 包含栈顶元素的引用
    /// - None 如果栈为空
    pub fn get_top(&self)->Option<&T>{
        if self. is_empty(){
            return None;
        }
        Some(&self.data[self.top-1])
    }
    /// 获取栈顶元素的可变引用
    /// 
    /// # 返回值
    /// - Some(&mut T) 包含栈顶元素的可变引用
    /// - None 如果栈为空
    pub fn get_top_mut(&mut self)->Option<&mut T>{
        if self.is_empty() {
            return None;
        }
        Some(&mut self.data[self.top-1])
    }
}
#[cfg(test)]
mod test{
    use crate::linear::stack::SequentialStack;
    #[test]
    fn stack(){
        let mut stack=SequentialStack::<i32>::new(10);
        for i in 0..10 {
            let item=10+i;
            stack.push(item).unwrap();
        }
        assert_eq!(stack.top,10);
        assert_eq!(stack.top,stack.data.len());
        let item=stack.pop().unwrap();
        assert_eq!(item,19);
        assert_eq!(stack.top,9);
        assert_eq!(stack.top,stack.data.len());
        assert_eq!(*stack.get_top().unwrap(),18);
        let item=stack.get_top_mut().unwrap();
        *item +=100;
        assert_eq!(*item,118);
    }
}
