pub struct MinStack {
    stack: Vec<i32>,
    min_stack: Vec<i32>,
}

impl Default for MinStack {
    fn default() -> Self {
        Self::new()
    }
}

impl MinStack {
    pub fn new() -> Self {
        MinStack {
            stack: Vec::new(),
            min_stack: Vec::new(),
        }
    }

    pub fn push(&mut self, x: i32) {
        self.stack.push(x);
        if self.min_stack.is_empty() || Some(&x) <= self.min_stack.last() {
            self.min_stack.push(x);
        }
    }

    pub fn pop(&mut self) {
        if self.stack.pop() == self.min_stack.last().copied() {
            self.min_stack.pop();
        }
    }

    pub fn top(&self) -> i32 {
        *self.stack.last().unwrap()
    }

    pub fn get_min(&self) -> i32 {
        *self.min_stack.last().unwrap()
    }
}

#[test]
fn test_min_stack() {
    let mut min_stack = MinStack::new();
    min_stack.push(-2);
    min_stack.push(0);
    min_stack.push(-3);
    assert_eq!(min_stack.get_min(), -3);
    min_stack.pop();
    assert_eq!(min_stack.top(), 0);
    assert_eq!(min_stack.get_min(), -2);
}
