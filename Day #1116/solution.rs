// Day 1116 - Quack: push/pop left, pull right, using three stacks.
// Two stacks L (top=leftmost) and R (top=rightmost); rebalance by splitting the
// other in half via scratch stack T. Amortized O(1) per op, O(1) extra memory.

struct Quack {
    l: Vec<i32>,
    r: Vec<i32>,
    t: Vec<i32>,
}

impl Quack {
    fn new() -> Self {
        Quack { l: Vec::new(), r: Vec::new(), t: Vec::new() }
    }
    fn push(&mut self, x: i32) {
        self.l.push(x);
    }
    fn rebalance_to_l(&mut self) {
        let m = self.r.len();
        let right_count = m / 2;
        for _ in 0..right_count {
            let v = self.r.pop().unwrap();
            self.t.push(v);
        }
        while let Some(v) = self.r.pop() {
            self.l.push(v);
        }
        while let Some(v) = self.t.pop() {
            self.r.push(v);
        }
    }
    fn rebalance_to_r(&mut self) {
        let m = self.l.len();
        let left_count = m / 2;
        for _ in 0..left_count {
            let v = self.l.pop().unwrap();
            self.t.push(v);
        }
        while let Some(v) = self.l.pop() {
            self.r.push(v);
        }
        while let Some(v) = self.t.pop() {
            self.l.push(v);
        }
    }
    fn pop(&mut self) -> i32 {
        if self.l.is_empty() {
            self.rebalance_to_l();
        }
        self.l.pop().unwrap()
    }
    fn pull(&mut self) -> i32 {
        if self.r.is_empty() {
            self.rebalance_to_r();
        }
        self.r.pop().unwrap()
    }
}

fn main() {
    let mut q = Quack::new();
    for x in [1, 2, 3] {
        q.push(x);
    }
    println!("pop: {}", q.pop());   // 3
    println!("pull: {}", q.pull()); // 1
    for x in [4, 5] {
        q.push(x);
    }
    println!("pull: {}", q.pull()); // 2
    println!("pop: {}", q.pop());   // 5
    println!("pull: {}", q.pull()); // 4
}
