pub mod linear;
pub mod sorting;
pub mod non_linear;
pub mod stack;

/// 表示数据结构操作中常见的错误类型
///
/// # 变体
///
/// - `IndexErr` - 索引越界错误，通常发生在访问、插入或删除元素时索引无效
/// - `FullErr`  - 容器已满错误，通常发生在向固定容量容器插入新元素时超出限制
#[derive(Debug)]
pub enum Err {
    IndexErr,
    FullErr,
}

#[cfg(test)]
mod tests {
    use crate::linear::array::ArrayList;

    #[test]
    fn test1() {
        let mut arr: ArrayList<char> = ArrayList::new();
        arr.insert(1, 'a').expect("TODO: panic message");
        arr.insert(2, 'b').expect("TODO: panic message");
        arr.insert(3, 'c').expect("TODO: panic message");
        // assert_eq!(arr.get_element(1), Ok('a'));
        // assert_eq!(arr.length(), 3);
        // let index = arr.locate_index('c').unwrap();
        // assert_eq!(index, 3);
        // let result = arr.get_element(4).unwrap();
        // println!("{:?}", result);
        // assert_eq!(result, 'c');
    }
}