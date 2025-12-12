// Quack (push/pop left, pull right) via three stacks.
// On underflow of one side, rebalance by moving half from the other side using a temp stack.
// Amortized O(1) per operation, O(1) extra memory beyond the three stacks.

struct Quack {
    front: Vec<i32>, // top = leftmost
    back: Vec<i32>,  // top = rightmost
    temp: Vec<i32>,
}

impl Quack {
    fn new() -> Self {
        Quack { front: vec![], back: vec![], temp: vec![] }
    }
    fn push(&mut self, x: i32) {
        self.front.push(x);
    }
    fn refill_front(&mut self) {
        let n = self.back.len();
        let left_count = (n + 1) / 2;
        let right_count = n - left_count;
        for _ in 0..right_count {
            let v = self.back.pop().unwrap();
            self.temp.push(v);
        }
        for _ in 0..left_count {
            let v = self.back.pop().unwrap();
            self.front.push(v);
        }
        while let Some(v) = self.temp.pop() {
            self.back.push(v);
        }
    }
    fn refill_back(&mut self) {
        let n = self.front.len();
        let right_count = (n + 1) / 2;
        let left_count = n - right_count;
        for _ in 0..left_count {
            let v = self.front.pop().unwrap();
            self.temp.push(v);
        }
        for _ in 0..right_count {
            let v = self.front.pop().unwrap();
            self.back.push(v);
        }
        while let Some(v) = self.temp.pop() {
            self.front.push(v);
        }
    }
    fn pop(&mut self) -> i32 {
        if self.front.is_empty() {
            self.refill_front();
        }
        self.front.pop().expect("empty")
    }
    fn pull(&mut self) -> i32 {
        if self.back.is_empty() {
            self.refill_back();
        }
        self.back.pop().expect("empty")
    }
}

fn main() {
    let mut q = Quack::new();
    q.push(1);
    q.push(2);
    q.push(3);
    println!("pop: {}", q.pop());   // 3
    println!("pull: {}", q.pull()); // 1
    q.push(4);
    println!("pull: {}", q.pull()); // 2
    println!("pop: {}", q.pop());   // 4
}
