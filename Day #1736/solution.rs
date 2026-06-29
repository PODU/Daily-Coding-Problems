// FIFO queue via two stacks (in/out); dequeue moves in->out when out empty. Amortized O(1) per op, O(n) space.

struct MyQueue {
    in_stack: Vec<i32>,
    out_stack: Vec<i32>,
}

impl MyQueue {
    fn new() -> Self {
        MyQueue { in_stack: Vec::new(), out_stack: Vec::new() }
    }
    fn enqueue(&mut self, x: i32) {
        self.in_stack.push(x);
    }
    fn dequeue(&mut self) -> i32 {
        if self.out_stack.is_empty() {
            while let Some(v) = self.in_stack.pop() {
                self.out_stack.push(v);
            }
        }
        self.out_stack.pop().expect("queue empty")
    }
}

fn main() {
    let mut q = MyQueue::new();
    q.enqueue(1);
    q.enqueue(2);
    q.enqueue(3);
    println!("{}", q.dequeue());
    q.enqueue(4);
    println!("{}", q.dequeue());
    println!("{}", q.dequeue());
    println!("{}", q.dequeue());
}
