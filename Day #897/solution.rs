// Queue via two stacks: enqueue->in_stack; dequeue moves all to out_stack when empty.
// Amortized O(1) per op, Space O(n).
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
    fn dequeue(&mut self) -> i32 {
        if self.out_stack.is_empty() {
            while let Some(v) = self.in_stack.pop() {
                self.out_stack.push(v);
            }
        }
        self.out_stack.pop().unwrap()
    }
}

fn main() {
    let mut q = Queue::new();
    q.enqueue(1);
    q.enqueue(2);
    println!("{}", q.dequeue());
    q.enqueue(3);
    println!("{}", q.dequeue());
    println!("{}", q.dequeue());
}
