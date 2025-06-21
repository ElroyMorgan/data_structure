/// 自定义字符串结构体，基于字符切片实现
/// 
/// # 示例
/// ```
/// use data_structure::linear::string::String;
/// let chars = ['H', 'e', 'l', 'l', 'o'];
/// let s = String::new(&chars);
/// ```
#[derive(Clone)]
pub struct String<'a>{
    /// 字符切片
    pub ch: &'a [char],
    /// 字符串长度
    pub len: usize,
}
impl<'a> String<'a> {
    /// 创建一个新的字符串实例
    ///
    /// # 参数
    /// - `ch`: 字符切片
    ///
    /// # 返回值
    /// 返回新的String实例
    ///
    /// # 示例
    /// ```
    /// use data_structure::linear::string::String;
    /// let s = String::new(&['R', 'u', 's', 't']);
    /// ```
    pub fn new(ch: &'a [char]) -> String<'a> {
        String {
            ch,
            len: ch.len()
        }
    }
    /// BF算法实现字符串匹配
    ///
    /// # 参数
    /// - `other`: 要匹配的子串
    /// - `pos`: 开始匹配的位置
    ///
    /// # 返回值
    /// 返回匹配位置的索引，如果未找到返回0
    ///
    /// # 示例
    /// ```
    /// use data_structure::linear::string::String;
    /// let s = String::new(&['a','b','c','d','e']);
    /// let sub = String::new(&['c','d']);
    /// assert_eq!(s.index_BF(&sub, 0), 2);
    /// ```
    #[allow(non_snake_case)]
    pub fn index_BF(&self, other: &String, pos: usize) -> usize {
        let mut i = pos;
        let mut j = 0;
        while i < self.len && j < other.len {
            if self.ch[i] == other.ch[j] {
                i += 1;
                j += 1;
            } else {
                i = i - j + 1;
                j = 0;
            }
        }
        if j == other.len {
            i - j
        } else {
            0
        }
    }
}

#[cfg(test)]
mod test{
    use crate::linear::string;

    #[test]
    fn test_index_bf() {
        let s1 = string::String::new(&['a','b','a','b','c','a','b','c','a','c','b','a','b']);
        let s2 = string::String::new(&['a','b','c','a','c']);
        
        // 测试能找到子串的情况
        assert_eq!(s1.index_BF(&s2, 0), 5);
        
        // 测试找不到子串的情况
        let s3 = string::String::new(&['x','y','z']);
        assert_eq!(s1.index_BF(&s3, 0), 0);
        
        // 测试从指定位置开始查找
        let s2_clone = s2.clone();
        assert_eq!(s1.index_BF(&s2_clone, 6), 0);
    }
}
