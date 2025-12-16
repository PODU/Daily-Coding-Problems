// Max stack: each entry stores the running max so far, giving O(1) push/pop/max.
// Time: O(1) per operation, Space: O(n).

struct MaxStack {
    st: Vec<(i32, i32)>, // (value, max_so_far)
}

impl MaxStack {
    fn new() -> Self {
        MaxStack { st: Vec::new() }
    }
    fn push(&mut self, v: i32) {
        let mx = match self.st.last() {
            Some(&(_, m)) => m.max(v),
            None => v,
        };
        self.st.push((v, mx));
    }
    fn pop(&mut self) -> Option<i32> {
        self.st.pop().map(|(v, _)| v)
    }
    fn max(&self) -> Option<i32> {
        self.st.last().map(|&(_, m)| m)
    }
}

fn main() {
    let mut s = MaxStack::new();
    s.push(1);
    s.push(3);
    s.push(2);
    println!("max: {}", s.max().unwrap()); // 3
    println!("pop: {}", s.pop().unwrap()); // 2
    println!("max: {}", s.max().unwrap()); // 3
    println!("pop: {}", s.pop().unwrap()); // 3
    println!("max: {}", s.max().unwrap()); // 1
}
