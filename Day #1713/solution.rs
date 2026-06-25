// Unrolled/paged queue: list of fixed-length blocks; head/tail indices. Amortized O(1) per op.
use std::collections::VecDeque;

const BS: usize = 4;

struct BlockQueue {
    blocks: VecDeque<Vec<i32>>,
    head: usize,
    tail: usize,
    sz: usize,
}

impl BlockQueue {
    fn new() -> Self {
        BlockQueue { blocks: VecDeque::new(), head: 0, tail: 0, sz: 0 }
    }
    fn enqueue(&mut self, x: i32) {
        if self.blocks.is_empty() || self.tail == BS {
            self.blocks.push_back(vec![0; BS]);
            self.tail = 0;
        }
        let last = self.blocks.len() - 1;
        self.blocks[last][self.tail] = x;
        self.tail += 1;
        self.sz += 1;
    }
    fn dequeue(&mut self) -> i32 {
        if self.sz == 0 {
            panic!("empty");
        }
        let x = self.blocks[0][self.head];
        self.head += 1;
        self.sz -= 1;
        if self.head == BS || (self.blocks.len() == 1 && self.head == self.tail) {
            self.blocks.pop_front();
            self.head = 0;
            if self.blocks.is_empty() {
                self.tail = 0;
            }
        }
        x
    }
    fn get_size(&self) -> usize { self.sz }
    fn num_blocks(&self) -> usize { self.blocks.len() }
}

fn main() {
    let mut q = BlockQueue::new();
    for i in 1..=10 {
        q.enqueue(i);
    }
    println!("size after enqueue 1..10: {}", q.get_size());
    println!("blocks allocated: {}", q.num_blocks());
    println!("dequeue 3: {} {} {}", q.dequeue(), q.dequeue(), q.dequeue());
    println!("size: {}", q.get_size());
    q.enqueue(11);
    println!("enqueue 11, size: {}", q.get_size());
    let mut rest = Vec::new();
    while q.get_size() > 0 {
        rest.push(q.dequeue().to_string());
    }
    println!("dequeue rest: {}", rest.join(" "));
    println!("size: {}", q.get_size());
}
