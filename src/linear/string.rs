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

fn next<T>(pattern: &[T]) -> Vec<usize>
where
    T: PartialEq + Eq,
{
    let mut next = vec![0; pattern.len()];
    if pattern.is_empty() {
        return next;
    }

    next[0] = 0;
    let mut i = 1;
    let mut j = 0;

    while i < pattern.len() {
        if pattern[i] == pattern[j] {
            j += 1;
            next[i] = j;
            i += 1;
        } else if j == 0 {
            next[i] = 0;
            i += 1;
        } else {
            j = next[j - 1];
        }
    }

    next
}

/// KMP算法实现字符串匹配
///
/// # 参数
/// - `main`: 主串
/// - `pattern`: 模式串
///
/// # 返回值
/// 返回匹配位置的索引(Some(usize))，如果未找到返回None
///
/// # 示例
/// ```
/// use data_structure::linear::string::index_KMP;
/// 
/// // 能找到子串的情况
/// assert_eq!(index_KMP(&['a','b','c','d','e'], &['c','d']), Some(2));
/// 
/// // 找不到子串的情况
/// assert_eq!(index_KMP(&['a','b','c'], &['x','y','z']), None);
/// 
/// // 空模式串的情况
/// assert_eq!(index_KMP(&['a','b','c'], &[]), Some(0));
/// ```
#[allow(non_snake_case)]
pub fn index_KMP<T>(main:&[T],pattern:&[T])->Option<usize>
where T:PartialEq+Eq,
{
    if pattern.is_empty(){
        return Some(0);
    }
    let next=next(pattern);
    let mut i=0usize;
    let mut j=0usize;
    while i<main.len() && j<pattern.len() {
        if j==0 || main[i]==pattern[j] {
            i+=1;
            j+=1;
        }else {
            j=next[j-1];
        }
    }
    if j==pattern.len() {
        Some(i-j)
    }else {
        None
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

    #[test]
    fn test_index_kmp() {
        // 测试能找到子串的情况
        assert_eq!(
            string::index_KMP(&['a','b','a','b','c','a','b','c','a','c','b','a','b'], &['a','b','c','a','c']),
            Some(5)
        );
        
        // 测试找不到子串的情况
        assert_eq!(
            string::index_KMP(&['a','b','c'], &['x','y','z']),
            None
        );
        
        // 测试空模式串
        assert_eq!(
            string::index_KMP(&['a','b','c'], &[]),
            Some(0)
        );
    }
}
