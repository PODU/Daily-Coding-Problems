// Day 1327: Queue backed by a deque of fixed-length blocks (chunks).
// enqueue appends to the tail block (new block when full); dequeue pops from the head block. Amortized O(1).
use std::collections::VecDeque;

const BLOCK: usize = 4;

struct BlockQueue {
    blocks: VecDeque<Vec<i32>>,
    head: usize, // read index in the front block
    size: usize,
}

impl BlockQueue {
    fn new() -> Self {
        BlockQueue { blocks: VecDeque::new(), head: 0, size: 0 }
    }

    fn enqueue(&mut self, x: i32) {
        if self.blocks.is_empty() || self.blocks.back().unwrap().len() == BLOCK {
            self.blocks.push_back(Vec::with_capacity(BLOCK));
        }
        self.blocks.back_mut().unwrap().push(x);
        self.size += 1;
    }

    fn dequeue(&mut self) -> i32 {
        assert!(self.size > 0, "empty");
        let x = self.blocks.front().unwrap()[self.head];
        self.head += 1;
        self.size -= 1;
        if self.head == self.blocks.front().unwrap().len() {
            self.blocks.pop_front();
            self.head = 0;
        }
        x
    }

    fn get_size(&self) -> usize { self.size }
}

fn main() {
    let mut q = BlockQueue::new();
    for i in 1..=5 {
        q.enqueue(i);
    }
    println!("{}", q.dequeue());  // 1
    println!("{}", q.dequeue());  // 2
    println!("{}", q.get_size()); // 3
}
