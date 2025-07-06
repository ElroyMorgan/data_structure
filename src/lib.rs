pub mod linear;
pub mod sorting;
pub mod non_linear;

#[derive(Debug)]
pub enum Err{
    IndexErr,
    FullErr,
}

#[cfg(test)]
mod tests {
    use crate::linear::array::SqList;
    use crate::linear::List;


    #[test]
    fn test1() {
        let mut arr: SqList<char> = SqList::init_list();
        arr.list_insert(1, 'a').expect("TODO: panic message");
        arr.list_insert(2, 'b').expect("TODO: panic message");
        arr.list_insert(3, 'c').expect("TODO: panic message");
        // assert_eq!(arr.get_element(1), Ok('a'));
        // assert_eq!(arr.length(), 3);
        // let index = arr.locate_index('c').unwrap();
        // assert_eq!(index, 3);
        // let result = arr.get_element(4).unwrap();
        // println!("{:?}", result);
        // assert_eq!(result, 'c');
    }
}