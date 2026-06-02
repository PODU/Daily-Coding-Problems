// Stack via a single MAX-heap keyed by a monotonic counter; newest key is largest so pops first (LIFO).
// push/pop O(log n), space O(n). Rust BinaryHeap is a max-heap by default.
use std::collections::BinaryHeap;

struct StackViaHeap {
    // BinaryHeap orders by tuple; (counter, value) -> largest counter pops first.
    heap: BinaryHeap<(u64, i32)>,
    counter: u64,
}

impl StackViaHeap {
    fn new() -> Self {
        StackViaHeap { heap: BinaryHeap::new(), counter: 0 }
    }
    fn push(&mut self, x: i32) {
        self.heap.push((self.counter, x));
        self.counter += 1;
    }
    fn pop(&mut self) -> i32 {
        match self.heap.pop() {
            Some((_, v)) => v,
            None => panic!("pop from empty stack"),
        }
    }
}

fn main() {
    let mut s = StackViaHeap::new();
    s.push(1);
    s.push(2);
    s.push(3);
    let mut out: Vec<i32> = Vec::new();
    out.push(s.pop()); // 3
    out.push(s.pop()); // 2
    s.push(4);
    out.push(s.pop()); // 4
    out.push(s.pop()); // 1

    let parts: Vec<String> = out.iter().map(|v| v.to_string()).collect();
    println!("{}", parts.join(" "));
}
