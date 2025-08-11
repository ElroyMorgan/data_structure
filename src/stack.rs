pub struct Stack{
    data:Vec<char>,
    top:usize, 
}

impl Stack {
    pub fn new(length:usize)->Stack{
        Stack { data:Vec::with_capacity(length), top:0 }
    }
    pub fn is_empty(&self)->bool{
        self.top==0
    }
    pub fn push(&mut self,c:char)->bool{
        if self.top ==  self.data.capacity(){
            return false;
        }
        self.data.push(c);
        self.top +=1;
        true
    }
    pub fn pop(&mut self)->Result<char,&str>{
        if self.is_empty(){
            return Err("此栈为空！");
        }
        self.top -=1;
        let del=self.data.pop().unwrap();
        Ok(del)
    }
    pub fn len(&self)->usize{
        self.top
    }
    pub fn peek(&self) -> Result<&char, &str> {
        if self.is_empty() {
            return Err("此栈为空！");
        }
        Ok(&self.data[self.top - 1])
    }
    pub fn peek_mut(&mut self) -> Result<&mut char, &str> {
        if self.is_empty() {
            return Err("此栈为空！");
        }
        let index = self.top - 1;
        Ok(&mut self.data[index])
    }
}

pub fn brack_check(s:&String)->bool{
    let mut s_stack=Stack::new(s.len());
    for c in s.chars(){
        if c =='(' || c =='[' || c =='{' {
            s_stack.push(c);
        }else {
            if s_stack.is_empty(){
                return false;
            }
            let top_elem=s_stack.pop().unwrap();
            if c == ')' && top_elem != '(' {
                return false;
            }
            if c == ']' && top_elem != '[' {
                return false;
            }
            if c == '}' && top_elem != '{' {
                return false;
            }
        }
    }
    s_stack.is_empty()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test2_stack() {
        let mut s=Stack::new(10);
        let mut is=s.is_empty();
        assert_eq!(true,is);
        s.push('Z');
        is=s.is_empty();
        assert_eq!(false,is);
        s.push('V');
        assert_eq!(s.len(),2);
        assert_eq!('V',s.pop().unwrap());
        
        // 测试 peek 和 peek_mut
        let mut s = Stack::new(5);
        s.push('A');
        s.push('B');
        assert_eq!(Ok(&'B'), s.peek());
        if let Ok(top) = s.peek_mut() {
            *top = 'C';
        }
        assert_eq!(Ok(&'C'), s.peek());
        assert_eq!('C', s.pop().unwrap());
        assert_eq!('A', s.pop().unwrap());
        assert_eq!(Err("此栈为空！"), s.peek());
        assert_eq!(Err("此栈为空！"), s.peek_mut());
    }
    #[test]
    fn test_brack_check(){
        let s=String::from("(({[]}))");
        assert_eq!(true,brack_check(&s));
    }
}