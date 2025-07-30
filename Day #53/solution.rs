// Day 53: FIFO queue from two stacks. Amortized O(1) per op.
// in-stack receives pushes; out-stack serves pops (refilled when empty).
struct Queue {
    in_stack: Vec<i32>,
    out_stack: Vec<i32>,
}

impl Queue {
    fn new() -> Self {
        Queue { in_stack: Vec::new(), out_stack: Vec::new() }
    }

    fn enqueue(&mut self, x: i32) {
        self.in_stack.push(x);
    }

    fn dequeue(&mut self) -> Option<i32> {
        if self.out_stack.is_empty() {
            while let Some(v) = self.in_stack.pop() {
                self.out_stack.push(v);
            }
        }
        self.out_stack.pop()
    }
}

fn main() {
    let mut q = Queue::new();
    q.enqueue(1);
    q.enqueue(2);
    q.enqueue(3);
    println!("{}", q.dequeue().unwrap()); // 1
    println!("{}", q.dequeue().unwrap()); // 2
    q.enqueue(4);
    println!("{}", q.dequeue().unwrap()); // 3
    println!("{}", q.dequeue().unwrap()); // 4
}
