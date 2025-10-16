// Day 443: FIFO queue from two stacks. Amortized O(1) per op: push onto `inn`,
// pop from `out`, refilling `out` from `inn` when empty.

struct QueueTwoStacks {
    inn: Vec<i32>,
    out: Vec<i32>,
}

impl QueueTwoStacks {
    fn new() -> Self {
        QueueTwoStacks { inn: Vec::new(), out: Vec::new() }
    }
    fn enqueue(&mut self, x: i32) {
        self.inn.push(x);
    }
    fn dequeue(&mut self) -> i32 {
        if self.out.is_empty() {
            while let Some(v) = self.inn.pop() {
                self.out.push(v);
            }
        }
        self.out.pop().expect("queue is empty")
    }
}

fn main() {
    let mut q = QueueTwoStacks::new();
    q.enqueue(1); q.enqueue(2); q.enqueue(3);
    println!("{}", q.dequeue()); // 1
    println!("{}", q.dequeue()); // 2
    q.enqueue(4);
    println!("{}", q.dequeue()); // 3
    println!("{}", q.dequeue()); // 4
}
