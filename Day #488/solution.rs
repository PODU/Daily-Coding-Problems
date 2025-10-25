// Day 488: Queue backed by a set of fixed-length arrays (blocks).
// Blocks of size B chained; head/tail indices roll over to next block.
// enqueue/dequeue amortized O(1), get_size O(1). Space O(n).
use std::collections::VecDeque;

const B: usize = 4; // fixed block length

struct BlockQueue {
    blocks: VecDeque<[i32; B]>, // set of fixed-length arrays
    head: usize,
    tail: usize,
    sz: usize,
}

impl BlockQueue {
    fn new() -> Self {
        BlockQueue { blocks: VecDeque::new(), head: 0, tail: 0, sz: 0 }
    }

    fn enqueue(&mut self, x: i32) {
        if self.blocks.is_empty() || self.tail == B { // allocate a new fixed block
            self.blocks.push_back([0; B]);
            self.tail = 0;
        }
        let last = self.blocks.len() - 1;
        self.blocks[last][self.tail] = x;
        self.tail += 1;
        self.sz += 1;
    }

    fn dequeue(&mut self) -> i32 {
        assert!(self.sz > 0, "empty");
        let x = self.blocks[0][self.head];
        self.head += 1;
        self.sz -= 1;
        if self.head == B { // front block exhausted -> free it
            self.blocks.pop_front();
            self.head = 0;
        }
        if self.blocks.len() == 1 && self.head == self.tail { // single block consumed
            self.blocks.clear();
            self.head = 0;
            self.tail = 0;
        }
        x
    }

    fn get_size(&self) -> usize {
        self.sz
    }
}

fn main() {
    let mut q = BlockQueue::new();
    for i in 1..=6 {
        q.enqueue(i); // 1..6
    }
    println!("size={}", q.get_size()); // 6
    println!("deq={}", q.dequeue());   // 1
    println!("deq={}", q.dequeue());   // 2
    q.enqueue(7);
    q.enqueue(8);
    println!("size={}", q.get_size()); // 6
    let mut out: Vec<String> = Vec::new();
    while q.get_size() > 0 {
        out.push(q.dequeue().to_string()); // 3 4 5 6 7 8
    }
    println!("{}", out.join(" "));
}
