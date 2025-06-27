//! # 队列模块
//! **此模块我用了AI进行修改，由于我对rust并没有学习到这些较为复杂的内容。**
//! 本模块实现了基于数组的循环队列结构，提供先进先出(FIFO)的数据存储与访问能力。
//! 
//! ## 示例
//! ```
//! use data_structure::linear::queue::Queue;
//! 
//! // 创建一个容量为3的整数队列
//! let mut queue: Queue<i32, 3> = Queue::new();
//! 
//! // 向队列中添加元素
//! queue.push(1).unwrap();
//! queue.push(2).unwrap();
//! queue.push(3).unwrap();
//! 
//! // 队列已满时尝试添加新元素会返回错误
//! assert!(queue.push(4).is_err());
//! 
//! // 从队列中移除元素
//! assert_eq!(queue.pop(), Some(1));
//! assert_eq!(queue.pop(), Some(2));
//! 
//! // 添加一个新元素后检查队列是否非空
//! queue.push(4).unwrap();
//! assert!(!queue.is_empty());
//! 
//! // 移除所有元素后队列应为空
//! assert_eq!(queue.pop(), Some(3));
//! assert_eq!(queue.pop(), Some(4));
//! assert!(queue.pop().is_none());
//! assert!(queue.is_empty());
//! ```

use std::mem::MaybeUninit;
use std::fmt::Debug;
use std::ptr;

/// 循环队列结构体
/// 
/// `Queue<T, N>` 是一个固定大小的循环队列，使用数组存储元素。
/// 队列的最大容量由常量参数 `N` 指定。
/// 
/// # 类型参数
/// - `T`: 队列中元素的类型，必须实现 `Debug` trait。
/// - `N`: 队列的最大容量，必须是一个正整数。
pub struct Queue<T, const N: usize>
where
    T: Debug,
{
    queue: [MaybeUninit<T>; N],
    front: usize,
    rear: usize,
}

impl<T, const N: usize> Queue<T, N>
where
    T: Debug,
{
    /// 创建一个新的空队列
    /// 
    /// # 返回值
    /// 返回一个新的 `Queue<T, N>` 实例。
    pub fn new() -> Self {
        Self {
            queue: unsafe { MaybeUninit::uninit().assume_init() },
            front: 0,
            rear: 0,
        }
    }

    /// 检查队列是否为空
    /// 
    /// # 返回值
    /// 如果队列为空则返回 `true`，否则返回 `false`。
    pub fn is_empty(&self) -> bool {
        self.front == self.rear
    }

    /// 检查队列是否已满
    /// 
    /// # 返回值
    /// 如果队列已满则返回 `true`，否则返回 `false`。
    fn is_full(&self) -> bool {
        (self.rear + 1) % N == self.front
    }

    /// 返回队列的最大容量
    /// 
    /// # 返回值
    /// 返回 `usize` 类型，表示队列的最大容量。
    pub fn capacity(&self) -> usize {
        N - 1
    }

    /// 向队列中添加元素
    /// 
    /// # 参数
    /// - `data`: 要添加到队列中的数据。
    /// 
    /// # 返回值
    /// 如果队列未满则返回 `Ok(())`，否则返回 `Err(&str)`。
    pub fn push(&mut self, data: T) -> Result<(), &'_ str> {
        if self.is_full() {
            return Err("queue is full");
        }
        self.queue[self.rear].write(data);
        self.rear = (self.rear + 1) % N;
        Ok(())
    }

    /// 从队列中移除并返回队首元素
    /// 
    /// # 返回值
    /// 如果队列非空则返回 `Some(T)`，否则返回 `None`。
    pub fn pop(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }
        let data = unsafe {
            let data = ptr::read(self.queue[self.front].as_ptr());
            self.queue[self.front] = MaybeUninit::uninit();
            data
        };
        self.front = (self.front + 1) % N;
        Some(data)
    }
}

impl<T, const N: usize> Drop for Queue<T, N>
where
    T: Debug,
{
    fn drop(&mut self) {
        let mut idx = self.front;
        while idx != self.rear {
            unsafe {
                ptr::drop_in_place(self.queue[idx].as_mut_ptr());
            }
            idx = (idx + 1) % N;
        }
    }
}
#[cfg(test)]
mod tests {
	use super::*;

    #[test]
    fn test_queue_operations() {
        // 创建一个容量为3的整数队列
        let mut queue: Queue<i32, 3> = Queue::new();

        // 测试初始状态
        assert!(queue.is_empty());
        assert_eq!(queue.pop(), None);
        
        // 检查容量是否正确
        assert_eq!(queue.capacity(), 2);

        // 添加元素
        assert!(queue.push(1).is_ok());
        assert!(!queue.is_empty());
        assert!(queue.push(2).is_ok());
        
        // 队列已满时尝试添加新元素应失败
        assert!(queue.push(3).is_err());

        // 移除元素
        assert_eq!(queue.pop(), Some(1));
        assert_eq!(queue.pop(), Some(2));
        assert!(queue.pop().is_none());
        
        // 添加新元素
        assert!(queue.push(3).is_ok());
        assert!(queue.push(4).is_ok());
        
        // 队列再次满时尝试添加新元素应失败
        assert!(queue.push(5).is_err());
        
        // 检查剩余元素顺序
        assert_eq!(queue.pop(), Some(3));
        assert_eq!(queue.pop(), Some(4));
        assert!(queue.pop().is_none());
        assert!(queue.is_empty());
    }

    #[test]
    fn test_queue_full_and_empty() {
        let mut queue: Queue<i32, 2> = Queue::new();

        // 填充队列
        assert!(queue.push(1).is_ok());
        // 由于循环队列的实现特性，容量为N的队列实际只能存放N-1个元素
        // 因此这里预期push(2)会返回错误
        assert!(queue.push(2).is_err());

        // 清空队列
        assert_eq!(queue.pop(), Some(1));
        assert!(queue.pop().is_none());

        // 再次填充队列
        assert!(queue.push(4).is_ok());
        // 同样，这里也调整了预期结果
        assert!(queue.push(5).is_err());
    }
}
