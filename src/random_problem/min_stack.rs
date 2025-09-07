struct MinStack {
    stack: Vec<(i32, i32)>,
}

impl MinStack {
    fn new() -> Self {
        MinStack { stack: Vec::new() }
    }
    fn push(&mut self, val: i32) {
        let min = if let Some(&(_, current_min)) = self.stack.last() {
            current_min.min(val)
        } else {
            val
        };
        self.stack.push((val, min));
    }
    fn pop(&mut self) {
        self.stack.pop();
    }
    fn top(&self) -> i32 {
        self.stack.last().map_or(0, |&(val, _)| val)
    }
    fn get_min(&self) -> i32 {
        self.stack.last().map_or(0, |&(_, min)| min)
    }
}

fn main() {
    let mut obj = MinStack::new();
    obj.push(-2);
    obj.push(0);
    obj.pop();
    println!("{}", obj.top());
    println!("{}", obj.get_min());
}
