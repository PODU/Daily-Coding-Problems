// Queue via fixed-length array blocks (capacity 4). VecDeque of blocks; enqueue to tail, dequeue from head.
// Amortized O(1) per op; O(n) space.
use std::collections::VecDeque;

const CAP: usize = 4;

struct BlockQueue {
    blocks: VecDeque<Vec<i32>>,
    head_idx: usize,
    tail_count: usize,
    size: usize,
}

impl BlockQueue {
    fn new() -> Self {
        BlockQueue { blocks: VecDeque::new(), head_idx: 0, tail_count: 0, size: 0 }
    }
    fn enqueue(&mut self, v: i32) {
        if self.blocks.is_empty() || self.tail_count == CAP {
            self.blocks.push_back(vec![0; CAP]);
            self.tail_count = 0;
        }
        let last = self.blocks.back_mut().unwrap();
        last[self.tail_count] = v;
        self.tail_count += 1;
        self.size += 1;
    }
    fn dequeue(&mut self) -> i32 {
        assert!(self.size > 0, "empty");
        let v = self.blocks[0][self.head_idx];
        self.head_idx += 1;
        self.size -= 1;
        if self.head_idx == CAP || (self.blocks.len() == 1 && self.head_idx == self.tail_count) {
            self.blocks.pop_front();
            self.head_idx = 0;
            if self.blocks.is_empty() {
                self.tail_count = 0;
            }
        }
        v
    }
    fn get_size(&self) -> usize {
        self.size
    }
}

fn main() {
    let mut q = BlockQueue::new();
    for i in 1..=10 {
        q.enqueue(i);
    }
    let mut dq = Vec::new();
    for _ in 0..3 {
        dq.push(q.dequeue().to_string());
    }
    println!("Dequeued: {}", dq.join(" "));
    println!("Size: {}", q.get_size());
}
