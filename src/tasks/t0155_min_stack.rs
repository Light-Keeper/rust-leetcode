// https://leetcode.com/problems/min-stack/
struct MinStack {
    stack: Vec<(i32, i32)>
}


impl MinStack {
    fn new() -> Self {
        Self { stack: vec![] }
    }
    
    fn push(&mut self, val: i32) {
        let min = self.stack.last().map(|x| x.0).unwrap_or(val).min(val);
        self.stack.push((min, val));
    }
    
    fn pop(&mut self) {
        self.stack.pop();
    }
    
    fn top(&self) -> i32 {
        self.stack.last().unwrap().1
    }
    
    fn get_min(&self) -> i32 {
        self.stack.last().unwrap().0
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test() {
        let mut min_stack = super::MinStack::new();
        min_stack.push(-2);
        min_stack.push(0);
        min_stack.push(-3);
        let a = min_stack.get_min(); // return -3
        min_stack.pop();
        let b = min_stack.top();    // return 0
        let c = min_stack.get_min(); // return -2

        assert_eq!(a, -3);
        assert_eq!(b, 0);
        assert_eq!(c, -2);
    }
}