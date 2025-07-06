pub mod array;
pub mod list_node;
pub mod stack;
pub mod string;
pub mod queue;
//！**此trait中所有关于位置的返回值都是位序，即从1开始，而非0开始，如果返回为0表示失败或者无效。**
pub trait List<T>
where T:Clone
{
        fn init_list()->Self;   //初始化
        fn destroy_list(self);     //销毁
        fn clear_list(&mut self); //清空
        fn list_empty(&self)->bool; //判断是否为空
        fn list_length(&self)->usize; //返回长度
        fn get_elem(&self,i:usize)->Option<T>;  //获取元素
        fn locate_elem(&self,e:T)->usize;   //返回第一个值为e的元素的位置
        fn prior_elem(&self,cur_e:T,_pre_e:&T)->Option<T>; //返回当前元素e的前一个元素
        fn next_elem(&self,cur_e:T,_next_e:&T)->Option<T>;  //返回当前元素e的下一个元素
        fn list_insert(&mut self,i:usize,e:T)->Result<(),crate::Err>;  //插入元素
        fn list_delete(&mut self,i:usize)->Result<(),crate::Err>;  //删除元素
        fn traverse_list(&self);    //遍历
    }
