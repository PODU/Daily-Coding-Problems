// Day 438: Stack via a max-heap. Each push tags the item with an increasing
// counter; the heap's max counter is the most-recent item. push/pop O(log n).
use std::collections::BinaryHeap;

struct StackFromHeap {
    heap: BinaryHeap<(i64, i32)>, // (counter, value); BinaryHeap is a max-heap
    counter: i64,
}

impl StackFromHeap {
    fn new() -> Self {
        StackFromHeap { heap: BinaryHeap::new(), counter: 0 }
    }
    fn push(&mut self, item: i32) {
        self.heap.push((self.counter, item));
        self.counter += 1;
    }
    fn pop(&mut self) -> i32 {
        self.heap.pop().expect("stack is empty").1
    }
}

fn main() {
    let mut s = StackFromHeap::new();
    s.push(1); s.push(2); s.push(3);
    println!("{}", s.pop()); // 3
    println!("{}", s.pop()); // 2
    s.push(4);
    println!("{}", s.pop()); // 4
    println!("{}", s.pop()); // 1
}
