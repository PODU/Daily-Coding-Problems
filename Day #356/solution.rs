// Queue built from a deque of fixed-capacity blocks (cap 4). Track head/tail offset and an O(1) size.
// enqueue/dequeue/get_size all amortized O(1) time; O(n) space.
use std::collections::VecDeque;

const CAP: usize = 4;

struct BlockQueue {
    blocks: VecDeque<Vec<i32>>,
    head_off: usize, // offset within front block
    tail_off: usize, // next write offset within back block
    sz: usize,
}

impl BlockQueue {
    fn new() -> Self {
        BlockQueue { blocks: VecDeque::new(), head_off: 0, tail_off: 0, sz: 0 }
    }

    fn enqueue(&mut self, v: i32) {
        if self.blocks.is_empty() || self.tail_off == CAP {
            self.blocks.push_back(vec![0; CAP]);
            self.tail_off = 0;
        }
        let last = self.blocks.back_mut().unwrap();
        last[self.tail_off] = v;
        self.tail_off += 1;
        self.sz += 1;
    }

    fn dequeue(&mut self) -> i32 {
        let v = self.blocks.front().unwrap()[self.head_off];
        self.head_off += 1;
        self.sz -= 1;
        if self.head_off == CAP {
            // front block fully consumed
            self.blocks.pop_front();
            self.head_off = 0;
        }
        v
    }

    fn get_size(&self) -> usize {
        self.sz
    }
}

fn main() {
    let mut q = BlockQueue::new();
    for v in [1, 2, 3, 4, 5] {
        q.enqueue(v);
    }
    println!("size={}", q.get_size());
    println!("{}", q.dequeue());
    println!("{}", q.dequeue());
    println!("{}", q.dequeue());
    println!("size={}", q.get_size());
}
