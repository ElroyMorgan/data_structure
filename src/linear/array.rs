use crate::linear::List;  // 确保 `List` 在当前作用域

/// 静态线性表的顺序存储结构
///
/// # 示例
///
/// ```
/// use data_structure::linear::array::SqList;
/// use data_structure::linear::List;
/// let list: SqList<i32> = SqList::init_list();
/// ```
#[derive(Clone, Debug)]
pub struct SqList<T>
where T:PartialEq+Eq
{
    element:[Option<T>;100],
    length:usize, //位序
}

impl<T> PartialEq for SqList<T> where T: PartialEq + Eq {
    fn eq(&self, other: &Self) -> bool {
        self.length == other.length &&
            self.element.iter().take(self.length).eq(other.element.iter().take(other.length))
    }
}

impl<T> List<T> for SqList<T>
where T: Clone + Eq + std::fmt::Debug
{
    /// 初始化一个空的线性表
    ///
    /// # 返回值
    ///
    /// 返回一个新的空 `SqList` 实例
    ///
    /// # 示例
    ///
    /// ```
    /// use data_structure::linear::array::SqList;
    /// use data_structure::linear::List;
    /// let list: SqList<i32> = SqList::init_list();
    /// ```
    fn init_list() -> Self {
        Self {
            element: std::array::from_fn(|_| None),
            length: 0,
        }
    }

    /// 销毁线性表
    ///
    /// # 参数
    ///
    /// * `self` - 要销毁的线性表
    ///
    /// # 示例
    ///
    /// ```
    /// use data_structure::linear::array::SqList;
    /// use data_structure::linear::List;
    /// let list: SqList<i32> = SqList::init_list();
    /// list.destroy_list();
    /// ```
    fn destroy_list(self) {
        drop(self)
    }

    /// 清空线性表中的所有元素
    ///
    /// # 参数
    ///
    /// * `&mut self` - 对线性表的可变引用
    ///
    /// # 示例
    ///
    /// ```
    /// use data_structure::linear::array::SqList;
    /// use data_structure::linear::List;
    /// let mut list: SqList<i32> = SqList::init_list();
    /// list.list_insert(1, 42).unwrap();
    /// list.clear_list();
    /// ```
    fn clear_list(&mut self){
        for elem in self.element.iter_mut().take(self.length) {
            *elem=None;
        }
        self.length=0;
    }

    /// 检查线性表是否为空
    ///
    /// # 参数
    ///
    /// * `&self` - 对线性表的不可变引用
    ///
    /// # 返回值
    ///
    /// 如果线性表为空返回 `true`，否则返回 `false`
    ///
    /// # 示例
    ///
    /// ```
    /// use data_structure::linear::array::SqList;
    /// use data_structure::linear::List;
    /// let list: SqList<i32> = SqList::init_list();
    /// assert!(list.list_empty());
    /// ```
    fn list_empty(&self) -> bool {
        self.length==0
    }

    /// 获取线性表的当前长度
    ///
    /// # 参数
    ///
    /// * `&self` - 对线性表的不可变引用
    ///
    /// # 返回值
    ///
    /// 返回线性表中元素的数量
    ///
    /// # 示例
    ///
    /// ```
    /// use data_structure::linear::array::SqList;
    /// use data_structure::linear::List;
    /// let mut list: SqList<i32> = SqList::init_list();
    /// list.list_insert(1, 42).unwrap();
    /// assert_eq!(list.list_length(), 1);
    /// ```
    fn list_length(&self) -> usize {
        self.length
    }

    /// 获取线性表中指定位置的元素
    ///
    /// # 参数
    ///
    /// * `&self` - 对线性表的不可变引用
    /// * `i` - 元素的位置（从1开始）
    ///
    /// # 返回值
    ///
    /// 如果位置有效返回 `Some(T)`，否则返回 `None`
    ///
    /// # 示例
    ///
    /// ```
    /// use data_structure::linear::array::SqList;
    /// use data_structure::linear::List;
    /// let mut list: SqList<i32> = SqList::init_list();
    /// list.list_insert(1, 42).unwrap();
    /// assert_eq!(list.get_elem(1), Some(42));
    /// ```
    fn get_elem(&self, i: usize) -> Option<T> {
        if i == 0 || i > self.length {
            return None;
        }
        self.element[i-1].clone()
    }

    /// 查找指定元素在线性表中的位置
    ///
    /// # 参数
    ///
    /// * `&self` - 对线性表的不可变引用
    /// * `e` - 要查找的元素
    ///
    /// # 返回值
    ///
    /// 如果找到返回元素的位置（从1开始），否则返回0
    ///
    /// # 示例
    ///
    /// ```
    /// use data_structure::linear::array::SqList;
    /// use data_structure::linear::List;
    /// let mut list: SqList<i32> = SqList::init_list();
    /// list.list_insert(1, 42).expect("TODO: panic message");
    /// assert_eq!(list.locate_elem(42), 1);
    /// ```
    fn locate_elem(&self, e: T) -> usize {
        for (i,element) in self.element.iter().enumerate().take(self.length) {
            if let Some(value)=element{
                if *value==e{
                    return i+1;
                }
            }
        }
        0
    }

    /// 获取指定元素的前驱元素
    ///
    /// # 参数
    ///
    /// * `&self` - 对线性表的不可变引用
    /// * `cur_e` - 当前元素
    /// * `_pre_e` - 用于输出前驱元素的参数（未使用）
    ///
    /// # 返回值
    ///
    /// 如果存在前驱元素返回 `Some(T)`，否则返回 `None`
    ///
    /// # 示例
    ///
    /// ```
    /// use data_structure::linear::array::SqList;
    /// use data_structure::linear::List;
    /// let mut list: SqList<i32> = SqList::init_list();
    /// list.list_insert(1, 42).expect("TODO: panic message");
    /// list.list_insert(2, 73).expect("TODO: panic message");
    /// assert_eq!(list.prior_elem(73, &42), Some(42));
    /// ```
    fn prior_elem(&self, cur_e: T, _pre_e: &T) ->Option<T> {
        for (i,element) in self.element.iter().enumerate().take(self.length) {
            if i == 0 {
                if let Some(value)=element{
                    if *value==cur_e{
                        return None;
                    }
                }
            } else {
                if let Some(value)=element{
                    if *value==cur_e{
                        return self.element[i-1].clone();
                    }
                }
            }
        }
        None
    }

    /// 获取指定元素的后继元素
    ///
    /// # 参数
    ///
    /// * `&self` - 对线性表的不可变引用
    /// * `cur_e` - 当前元素
    /// * `_next_e` - 用于输出后继元素的参数（未使用）
    ///
    /// # 返回值
    ///
    /// 如果存在后继元素返回 `Some(T)`，否则返回 `None`
    ///
    /// # 示例
    ///
    /// ```
    /// use data_structure::linear::array::SqList;
    /// use data_structure::linear::List;
    /// let mut list: SqList<i32> = SqList::init_list();
    /// list.list_insert(1, 42).expect("TODO: panic message");
    /// list.list_insert(2, 73).expect("TODO: panic message");
    /// assert_eq!(list.next_elem(42, &73), Some(73));
    /// ```
    fn next_elem(&self, cur_e: T, _next_e: &T) -> Option<T> {
        for (i,element) in self.element.iter().enumerate().take(self.length-1) {
            if let Some(value)=element{
                if *value==cur_e{
                    return self.element[i+1].clone();
                }
            }
        }
        None
    }

    /// 在指定位置插入元素
    ///
    /// # 参数
    ///
    /// * `&mut self` - 对线性表的可变引用
    /// * `i` - 插入位置（从1开始）
    /// * `e` - 要插入的元素
    ///
    /// # 返回值
    ///
    /// 如果操作成功返回 `Ok(())`，否则返回错误信息
    ///
    /// # 示例
    ///
    /// ```
    /// use data_structure::linear::array::SqList;
    /// use data_structure::linear::List;
    /// let mut list: SqList<i32> = SqList::init_list();
    /// assert!(list.list_insert(1, 42).is_ok());
    /// ```
    fn list_insert(&mut self, i: usize, e: T)->Result<(),crate::Err> {
        if i == 0 || i > self.length + 1 {
            return Err(crate::Err::IndexErr);
        }
        if self.length >= 100 {
            return Err(crate::Err::FullErr);
        }
        let idx = i - 1;
        for j in (idx..self.length).rev() {
            self.element[j + 1] = self.element[j].clone();
        }
        self.element[idx] = Some(e);
        self.length += 1;
        Ok(())
    }

    /// 删除指定位置的元素
    ///
    /// # 参数
    ///
    /// * `&mut self` - 对线性表的可变引用
    /// * `i` - 删除位置（从1开始）
    ///
    /// # 返回值
    ///
    /// 如果操作成功返回 `Ok(())`，否则返回错误信息
    ///
    /// # 示例
    ///
    /// ```
    /// use data_structure::linear::array::SqList;
    /// use data_structure::linear::List;
    /// let mut list: SqList<i32> = SqList::init_list();
    /// list.list_insert(1, 42).expect("TODO: panic message");
    /// assert!(list.list_delete(1).is_ok());
    /// ```
    fn list_delete(&mut self, i: usize)->Result<(),crate::Err> {
        if i == 0 || i > self.length {
            return Err(crate::Err::IndexErr);
        }
        let idx = i - 1;
        if idx == self.length - 1 {
            self.element[idx] = None;
        } else {
            for j in idx..self.length - 1 {
                self.element[j] = self.element[j + 1].clone();
            }
        }
        self.element[self.length - 1] = None;
        self.length -= 1;
        Ok(())
    }

    /// 遍历并打印线性表中的所有元素
    ///
    /// # 参数
    ///
    /// * `&self` - 对线性表的不可变引用
    ///
    /// # 示例
    ///
    /// ```
    /// use data_structure::linear::array::SqList;
    /// use data_structure::linear::List;
    /// let mut list: SqList<i32> = SqList::init_list();
    /// list.list_insert(1, 42).expect("TODO: panic message");
    /// list.list_insert(2, 73).expect("TODO: panic message");
    /// list.traverse_list();
    /// ```
    fn traverse_list(&self) {
        for i in 0..self.length {
            if let Some(value) = &self.element[i] {
                println!("{:?}", value);
            }
        }
    }
}

pub struct ArrayList<T> {
    pub element: [Option<T>; 100],
    pub length: usize,
}

impl<T> ArrayList<T>
where
    T: Copy + PartialEq + std::fmt::Debug,
{
    /// 创建一个新的空 `ArrayList`
    ///
    /// # 示例
    ///
    /// ```
    /// use data_structure::linear::array::ArrayList;
    /// let list: ArrayList<i32> = ArrayList::new();
    /// ```
    pub fn new() -> Self {
        Self {
            element: [None; 100],
            length: 0,
        }
    }

    /// 获取指定位置的元素
    ///
    /// # 参数
    ///
    /// * `index` - 要获取的元素位置(1-based)
    ///
    /// # 返回值
    ///
    /// `Result<T, &str>` - 成功返回元素，失败返回错误信息
    ///
    /// # 错误
    ///
    /// - 如果索引为0或超出当前长度，返回"数组越界"
    /// - 如果元素不存在，返回"元素不存在!"
    pub fn get_element(&self, index: usize) -> Result<T, &'static str> {
        if index == 0 {
            return Err("数组越界");
        }
        let idx = index - 1;
        if idx >= self.length {
            return Err("数组越界");
        };
        match self.element[idx] {
            Some(value) => Ok(value),
            None => Err("元素不存在!"),
        }
    }

    /// 在指定位置插入元素
    ///
    /// # 参数
    ///
    /// * `position` - 插入位置(1-based)
    /// * `element` - 要插入的元素
    ///
    /// # 返回值
    ///
    /// `Result<(), &str>` - 成功返回`Ok(())`，失败返回错误信息
    ///
    /// # 错误
    ///
    /// - 如果位置无效，返回"数组越界"
    /// - 如果数组已满，返回"数组已满"
    pub fn insert(&mut self, position: usize, element: T) -> Result<(), &'static str> {
        if position == 0 || position > self.length + 1 {
            return Err("数组越界");
        };
        let index = position - 1;
        if self.length >= 100 {
            return Err("数组已满");
        };
        for i in (index..self.length).rev() {
            self.element[i + 1] = self.element[i];
        };
        self.element[index] = Some(element);
        self.length += 1;
        Ok(())
    }

    /// 查找元素的位置
    ///
    /// # 参数
    ///
    /// * `element` - 要查找的元素
    ///
    /// # 返回值
    ///
    /// `Result<usize, &str>` - 成功返回位置(1-based)，失败返回"查找失败"
    pub fn locate_index(&self, element: T) -> Result<usize, &'static str> {
        for i in 0..self.length {
            if let Some(value) = self.element[i] {
                if value == element {
                    return Ok(i + 1);
                }
            }
        }
        Err("查找失败")
    }

    /// 删除指定位置的元素
    ///
    /// # 参数
    ///
    /// * `index` - 要删除的位置(1-based)
    ///
    /// # 返回值
    ///
    /// `Result<(), &str>` - 成功返回`Ok(())`，失败返回"数组操作越界!"
    pub fn delete(&mut self, index: usize) -> Result<(), &'static str> {
        if index == 0 || index > self.length {
            return Err("数组操作越界!");
        };
        for index in index..self.length {
            self.element[index - 1] = self.element[index];
        }
        self.element[self.length - 1] = None;
        self.length -= 1;
        Ok(())
    }

    /// 清空列表
    pub fn clear(&mut self) {
        for index in 0..self.length {
            self.element[index] = None;
        };
        self.length = 0;
    }

    /// 获取列表长度
    pub fn length(&self) -> usize {
        self.length
    }

    /// 检查列表是否非空
    ///
    /// # 返回值
    ///
    /// 如果列表不为空返回`true`，否则返回`false`
    pub fn empty(&self) -> bool {
        self.length != 0
    }

    /// 获取元素的前驱
    ///
    /// # 参数
    ///
    /// * `cur_e` - 当前元素
    ///
    /// # 返回值
    ///
    /// `Result<T, &str>` - 成功返回前驱元素，失败返回错误信息
    ///
    /// # 错误
    ///
    /// - 如果列表为空，返回"数组为空"
    /// - 如果是第一个元素，返回"当前元素为第一个元素,没有前驱"
    /// - 如果元素不存在，返回"元素不存在!"
    pub fn prior_element(&self, cur_e: T) -> Result<T, &'static str> {
        if self.empty() {
            return Err("数组为空");
        };
        if self.element[0] == Some(cur_e) {
            return Err("当前元素为第一个元素,没有前驱");
        };
        for index in 1..self.length {
            if self.element[index] == Some(cur_e) {
                return Ok(self.element[index - 1].unwrap());
            };
        };
        Err("元素不存在!")
    }

    /// 获取元素后继
    ///
    /// # 参数
    ///
    /// * `cur_e` - 当前元素
    ///
    /// # 返回值
    ///
    /// `Result<T, &str>` - 成功返回后继元素，失败返回错误信息
    ///
    /// # 错误
    ///
    /// - 如果列表为空，返回"数组为空"
    /// - 如果是最后一个元素，返回"当前元素为最后一个元素,没有后继"
    /// - 如果元素不存在，返回"元素不存在!"
    pub fn next_element(&self, cur_e: T) -> Result<T, &'static str> {
        if self.empty() {
            return Err("数组为空");
        }
        if self.element[self.length - 1] == Some(cur_e) {
            return Err("当前元素为最后一个元素,没有后继");
        }
        for index in 0..self.length - 1 {
            if self.element[index] == Some(cur_e) {
                return Ok(self.element[index + 1].unwrap());
            }
        }
        Err("元素不存在!")
    }

    /// 遍历并打印所有元素
    pub fn traverse(&self) {
        for index in 0..self.length {
            println!("{:?}", self.element[index].unwrap());
        }
    }
}