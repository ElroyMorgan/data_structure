pub mod array;
pub mod list_node;
pub mod stack;
pub mod string;
pub mod queue;
pub mod list;

/// **此trait中所有关于位置的返回值都是位序，即从1开始，而非0开始，如果返回为0表示失败或者无效。**
pub trait List<T>
where T: Clone
{
    /// 初始化一个空列表。
    ///
    /// # 返回值
    ///
    /// - 返回一个新的列表实例。
    ///
    /// # 示例
    ///
    /// ```text
    /// let list = ArrayList::<i32>::init_list();
    /// ```
    fn init_list() -> Self; // 初始化

    /// 销毁列表并释放其占用的资源。
    ///
    /// # 参数
    ///
    /// - `self` - 被销毁的列表实例。
    ///
    /// # 注意
    ///
    /// 此操作不可逆，调用后列表将无法再使用。
    fn destroy_list(self); // 销毁

    /// 清空列表中的所有元素。
    ///
    /// # 参数
    ///
    /// - `&mut self` - 可变引用的列表实例。
    ///
    /// # 示例
    ///
    /// ```text
    /// let mut list = ArrayList::<i32>::init_list();
    /// list.clear_list();
    /// ```
    fn clear_list(&mut self); // 清空

    /// 判断列表是否为空。
    ///
    /// # 参数
    ///
    /// - `&self` - 不可变引用的列表实例。
    ///
    /// # 返回值
    ///
    /// - 如果列表为空，则返回 `true`；否则返回 `false`。
    ///
    /// # 示例
    ///
    /// ```text
    /// let list = ArrayList::<i32>::init_list();
    /// assert!(list.list_empty());
    /// ```
    fn list_empty(&self) -> bool; // 判断是否为空

    /// 获取列表的长度。
    ///
    /// # 参数
    ///
    /// - `&self` - 不可变引用的列表实例。
    ///
    /// # 返回值
    ///
    /// - 返回列表中元素的数量。
    ///
    /// # 示例
    ///
    /// ```text
    /// let mut list = ArrayList::<i32>::init_list();
    /// list.list_insert(1, 42).unwrap();
    /// assert_eq!(list.list_length(), 1);
    /// ```
    fn list_length(&self) -> usize; // 返回长度

    /// 获取指定位置的元素。
    ///
    /// # 参数
    ///
    /// - `&self` - 不可变引用的列表实例。
    /// - `i` - 元素的位置（位序，从1开始）。
    ///
    /// # 返回值
    ///
    /// - 如果位置有效，则返回对应元素的 `Some(T)`；
    /// - 否则返回 `None`。
    ///
    /// # 示例
    ///
    /// ```text
    /// let mut list = ArrayList::<i32>::init_list();
    /// list.list_insert(1, 42).unwrap();
    /// assert_eq!(list.get_elem(1), Some(42));
    /// ```
    fn get_elem(&self, i: usize) -> Option<T>; // 获取元素

    /// 查找第一个值为 `e` 的元素的位置。
    ///
    /// # 参数
    ///
    /// - `&self` - 不可变引用的列表实例。
    /// - `e` - 要查找的元素值。
    ///
    /// # 返回值
    ///
    /// - 如果找到元素，则返回其位置（位序，从1开始）；
    /// - 否则返回 `0` 表示未找到。
    ///
    /// # 示例
    ///
    /// ```text
    /// let mut list = ArrayList::<i32>::init_list();
    /// list.list_insert(1, 42).unwrap();
    /// assert_eq!(list.locate_elem(42), 1);
    /// ```
    fn locate_elem(&self, e: T) -> usize; // 返回第一个值为e的元素的位置

    /// 获取当前元素的前一个元素。
    ///
    /// # 参数
    ///
    /// - `&self` - 不可变引用的列表实例。
    /// - `cur_e` - 当前元素。
    /// - `_pre_e` - 用于占位的前一个元素引用（实际未使用）。
    ///
    /// # 返回值
    ///
    /// - 如果存在前一个元素，则返回其值的 `Some(T)`；
    /// - 否则返回 `None`。
    ///
    /// # 示例
    ///
    /// ```text
    /// let mut list = ArrayList::<i32>::init_list();
    /// list.list_insert(1, 42).unwrap();
    /// list.list_insert(2, 84).unwrap();
    /// assert_eq!(list.prior_elem(84, &42), Some(42));
    /// ```
    fn prior_elem(&self, cur_e: T, _pre_e: &T) -> Option<T>; // 返回当前元素e的前一个元素

    /// 获取当前元素的下一个元素。
    ///
    /// # 参数
    ///
    /// - `&self` - 不可变引用的列表实例。
    /// - `cur_e` - 当前元素。
    /// - `_next_e` - 用于占位的下一个元素引用（实际未使用）。
    ///
    /// # 返回值
    ///
    /// - 如果存在下一个元素，则返回其值的 `Some(T)`；
    /// - 否则返回 `None`。
    ///
    /// # 示例
    ///
    /// ```text
    /// let mut list = ArrayList::<i32>::init_list();
    /// list.list_insert(1, 42).unwrap();
    /// list.list_insert(2, 84).unwrap();
    /// assert_eq!(list.next_elem(42, &84), Some(84));
    /// ```
    fn next_elem(&self, cur_e: T, _next_e: &T) -> Option<T>; // 返回当前元素e的下一个元素

    /// 在指定位置插入元素。
    ///
    /// # 参数
    ///
    /// - `&mut self` - 可变引用的列表实例。
    /// - `i` - 插入位置（位序，从1开始）。
    /// - `e` - 要插入的元素。
    ///
    /// # 返回值
    ///
    /// - 如果插入成功，则返回 `Ok(())`；
    /// - 如果位置无效或其他错误发生，则返回 `Err(crate::Err)`。
    ///
    /// # 示例
    ///
    /// ```text
    /// let mut list = ArrayList::<i32>::init_list();
    /// assert!(list.list_insert(1, 42).is_ok());
    /// ```
    fn list_insert(&mut self, i: usize, e: T) -> Result<(), crate::Err>; // 插入元素

    /// 删除指定位置的元素。
    ///
    /// # 参数
    ///
    /// - `&mut self` - 可变引用的列表实例。
    /// - `i` - 删除位置（位序，从1开始）。
    ///
    /// # 返回值
    ///
    /// - 如果删除成功，则返回 `Ok(())`；
    /// - 如果位置无效或其他错误发生，则返回 `Err(crate::Err)`。
    ///
    /// # 示例
    ///
    /// ```text
    /// let mut list = ArrayList::<i32>::init_list();
    /// list.list_insert(1, 42).unwrap();
    /// assert!(list.list_delete(1).is_ok());
    /// ```
    fn list_delete(&mut self, i: usize) -> Result<(), crate::Err>; // 删除元素

    /// 遍历列表并打印所有元素。
    ///
    /// # 参数
    ///
    /// - `&self` - 不可变引用的列表实例。
    ///
    /// # 示例
    ///
    /// ```text
    /// let mut list = ArrayList::<i32>::init_list();
    /// list.list_insert(1, 42).unwrap();
    /// list.traverse_list(); // 输出：[42]
    /// ```
    fn traverse_list(&self); // 遍历
}
