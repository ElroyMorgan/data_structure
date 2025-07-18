#[warn(dead_code)]
pub struct NodeRaw{
    data:i32,
    next:*mut NodeRaw,
}
impl NodeRaw {
    pub fn new()->Self {
        Self { data: 0, next:std::ptr::null_mut() }
    }
    pub fn insert(&mut self,i:isize,e:i32)->Result<(), String>{
        if i<1 {
            return Err("位序从1开始".to_string());
        }
        let mut pos=0;
        let mut s = &raw mut *self;
        while pos<i-1 {
            if s.is_null() {
            return Err("遍历中遇到空指针".to_string());
        }
            unsafe {
                if (*s).next.is_null() {
                return Err(format!("位置 {} 超出链表范围", i));
            }
                s=(*s).next;
            }
            pos+=1;
        }
        let e=Box::new(NodeRaw{
            data:e,
            next:std::ptr::null_mut(),
        });
        let e=Box::into_raw(e);
        unsafe {
            (*e).next=(*s).next;
            (*s).next=e;
        }
        Ok(())
    }
}

impl Drop for NodeRaw {
    fn drop(&mut self) {
        let mut curr = self.next;
        while !curr.is_null() {
            unsafe {
                let next = (*curr).next;
                drop(Box::from_raw(curr));
                curr = next;
            }
        }
    }
}