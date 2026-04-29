// Day 1442: Implement a stack using only a max-heap.
// Approach: tag each item with an increasing counter as its key; the heap's
// max key is always the most recently pushed item. push/pop O(log n).
use std::collections::BinaryHeap;

struct Stack {
    heap: BinaryHeap<(i64, i32)>, // (counter, value); BinaryHeap is a max-heap
    counter: i64,
}

impl Stack {
    fn new() -> Self {
        Stack { heap: BinaryHeap::new(), counter: 0 }
    }
    fn push(&mut self, item: i32) {
        self.heap.push((self.counter, item));
        self.counter += 1;
    }
    fn pop(&mut self) -> i32 {
        self.heap.pop().expect("pop from empty stack").1
    }
}

fn main() {
    let mut s = Stack::new();
    s.push(1);
    s.push(2);
    s.push(3);
    println!("{} {} {}", s.pop(), s.pop(), s.pop()); // 3 2 1
}
