// Max stack with O(1) push/pop/max using an auxiliary stack of running maxima.
// All operations O(1) time; O(n) space.
struct MaxStack {
    data: Vec<i32>,
    maxs: Vec<i32>,
}

impl MaxStack {
    fn new() -> Self {
        MaxStack { data: Vec::new(), maxs: Vec::new() }
    }
    fn push(&mut self, val: i32) {
        self.data.push(val);
        match self.maxs.last() {
            Some(&m) if m > val => self.maxs.push(m),
            _ => self.maxs.push(val),
        }
    }
    fn pop(&mut self) -> Option<i32> {
        self.maxs.pop();
        self.data.pop()
    }
    fn max(&self) -> Option<i32> {
        self.maxs.last().copied()
    }
}

fn main() {
    let mut s = MaxStack::new();
    for v in [1, 5, 3, 9, 2] {
        s.push(v);
    }
    println!("max: {}", s.max().unwrap());
    println!("pop: {}", s.pop().unwrap());
    println!("pop: {}", s.pop().unwrap());
    println!("max: {}", s.max().unwrap());
}
