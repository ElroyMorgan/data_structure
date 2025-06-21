
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