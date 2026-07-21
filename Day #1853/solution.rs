// Day 1853: Stack with O(1) push/pop/max.
// Keep an auxiliary stack of running maxima alongside the data stack. All ops O(1) time, O(n) space.

struct MaxStack {
    data: Vec<i32>,
    maxs: Vec<i32>,
}

impl MaxStack {
    fn new() -> Self {
        MaxStack { data: Vec::new(), maxs: Vec::new() }
    }
    fn push(&mut self, v: i32) {
        self.data.push(v);
        let m = match self.maxs.last() {
            Some(&prev) if prev > v => prev,
            _ => v,
        };
        self.maxs.push(m);
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
    s.push(1); s.push(5); s.push(3);
    println!("{}", s.max().unwrap()); // 5
    println!("{}", s.pop().unwrap()); // 3
    println!("{}", s.max().unwrap()); // 5
    println!("{}", s.pop().unwrap()); // 5
    println!("{}", s.max().unwrap()); // 1
}
