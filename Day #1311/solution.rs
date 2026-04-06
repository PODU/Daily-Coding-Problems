// Quack (deque) via three stacks. On underflow of one end, rebalance by moving
// half the elements through the third stack -> O(1) amortized, O(1) extra memory.

struct Quack {
    l: Vec<i32>, // left stack, top = leftmost
    r: Vec<i32>, // right stack, top = rightmost
    t: Vec<i32>, // temp
}

impl Quack {
    fn new() -> Self { Quack { l: vec![], r: vec![], t: vec![] } }

    fn rebalance(src: &mut Vec<i32>, dst: &mut Vec<i32>, tmp: &mut Vec<i32>) {
        let n = src.len();
        let k = n / 2; // elements that stay in src
        for _ in 0..k { let v = src.pop().unwrap(); tmp.push(v); }
        for _ in 0..(n - k) { let v = src.pop().unwrap(); dst.push(v); }
        for _ in 0..k { let v = tmp.pop().unwrap(); src.push(v); }
    }

    fn push(&mut self, x: i32) { self.l.push(x); }
    fn pop(&mut self) -> i32 {
        if self.l.is_empty() { Self::rebalance(&mut self.r, &mut self.l, &mut self.t); }
        self.l.pop().unwrap()
    }
    fn pull(&mut self) -> i32 {
        if self.r.is_empty() { Self::rebalance(&mut self.l, &mut self.r, &mut self.t); }
        self.r.pop().unwrap()
    }
}

fn main() {
    let mut q = Quack::new();
    q.push(1); q.push(2); q.push(3);
    println!("{}", q.pop());  // 3
    println!("{}", q.pull()); // 1
    q.push(4);
    println!("{}", q.pull()); // 2
    println!("{}", q.pop());  // 4
}
