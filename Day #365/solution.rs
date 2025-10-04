// Day 365: "Quack" deque (push/pop left, pull right) from three stacks.
// l holds the left part (left end on top), r the right part (right end on top);
// tmp is a transient helper used only to re-split when one side empties.
// Rebalance moves k after k ops -> O(1) amortized, O(1) extra memory.

struct Quack {
    l: Vec<i32>,
    r: Vec<i32>,
    tmp: Vec<i32>,
}

impl Quack {
    fn new() -> Self {
        Quack { l: Vec::new(), r: Vec::new(), tmp: Vec::new() }
    }

    fn push(&mut self, x: i32) {
        self.l.push(x);
    }

    // refill == true -> fill l from r; refill == false -> fill r from l.
    fn rebalance(&mut self, to_left: bool) {
        let n = if to_left { self.r.len() } else { self.l.len() };
        let to_count = (n + 1) / 2;
        for _ in 0..(n - to_count) {
            let v = if to_left { self.r.pop() } else { self.l.pop() }.unwrap();
            self.tmp.push(v);
        }
        for _ in 0..to_count {
            if to_left {
                let v = self.r.pop().unwrap();
                self.l.push(v);
            } else {
                let v = self.l.pop().unwrap();
                self.r.push(v);
            }
        }
        while let Some(v) = self.tmp.pop() {
            if to_left {
                self.r.push(v);
            } else {
                self.l.push(v);
            }
        }
    }

    fn pop(&mut self) -> i32 {
        if self.l.is_empty() {
            self.rebalance(true);
        }
        self.l.pop().unwrap()
    }

    fn pull(&mut self) -> i32 {
        if self.r.is_empty() {
            self.rebalance(false);
        }
        self.r.pop().unwrap()
    }
}

fn main() {
    let mut q = Quack::new();
    q.push(1);
    q.push(2);
    q.push(3);
    println!("{}", q.pop());  // 3
    println!("{}", q.pull()); // 1
    q.push(4);
    q.push(5);
    println!("{}", q.pull()); // 2
    println!("{}", q.pop());  // 5
    println!("{}", q.pop());  // 4
}
