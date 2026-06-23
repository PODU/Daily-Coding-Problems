// Deque ("quack") from three stacks: L (top=leftmost), R (top=rightmost), T temp.
// On empty side, move half the other stack across (T as transient buffer) => O(1) amortized.
struct Quack {
    l: Vec<i32>, // top = leftmost
    r: Vec<i32>, // top = rightmost
    t: Vec<i32>, // transient buffer
}

impl Quack {
    fn new() -> Self {
        Quack { l: Vec::new(), r: Vec::new(), t: Vec::new() }
    }

    fn push(&mut self, x: i32) { self.l.push(x); } // add to LEFT end

    fn pop(&mut self) -> i32 { // remove LEFT end
        if self.l.is_empty() {
            self.rebalance_l_from_r();
        }
        self.l.pop().unwrap()
    }

    fn pull(&mut self) -> i32 { // remove RIGHT end
        if self.r.is_empty() {
            self.rebalance_r_from_l();
        }
        self.r.pop().unwrap()
    }

    fn rebalance_l_from_r(&mut self) {
        let size = self.r.len();
        let half = if size / 2 == 0 { 1 } else { size / 2 };
        let keep = size - half;
        for _ in 0..keep {
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

    fn rebalance_r_from_l(&mut self) {
        let size = self.l.len();
        let half = if size / 2 == 0 { 1 } else { size / 2 };
        let keep = size - half;
        for _ in 0..keep {
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
}

fn main() {
    let mut q = Quack::new();
    q.push(1);
    q.push(2);
    q.push(3); // left-to-right: 3,2,1
    println!("{}", q.pop());  // 3
    println!("{}", q.pull()); // 1
    q.push(4);                // now 4,2
    println!("{}", q.pop());  // 4
    println!("{}", q.pull()); // 2
}
