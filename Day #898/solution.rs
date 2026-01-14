// Stack via max-heap: tag each push with increasing priority; heap pops highest priority (most recent). O(log n)/op, O(n) space.
use std::collections::BinaryHeap;

struct HeapStack {
    heap: BinaryHeap<(u64, i32)>, // (priority, value); BinaryHeap is a max-heap
    counter: u64,
}

impl HeapStack {
    fn new() -> Self {
        HeapStack { heap: BinaryHeap::new(), counter: 0 }
    }
    fn push(&mut self, v: i32) {
        self.heap.push((self.counter, v));
        self.counter += 1;
    }
    fn pop(&mut self) -> i32 {
        self.heap.pop().expect("pop from empty stack").1
    }
}

fn main() {
    let mut s = HeapStack::new();
    s.push(1);
    s.push(2);
    s.push(3);
    println!("{}", s.pop()); // 3
    println!("{}", s.pop()); // 2
    s.push(4);
    println!("{}", s.pop()); // 4
    println!("{}", s.pop()); // 1
}
