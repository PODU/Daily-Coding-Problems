// Day 154: Stack using only a max-heap. Tag each item with an increasing
// priority so the heap always pops the most recent. push/pop O(log n).
use std::collections::BinaryHeap;

struct HeapStack {
    heap: BinaryHeap<(u64, i32)>, // (priority, value); BinaryHeap is a max-heap
    counter: u64,
}

impl HeapStack {
    fn new() -> Self {
        HeapStack { heap: BinaryHeap::new(), counter: 0 }
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
