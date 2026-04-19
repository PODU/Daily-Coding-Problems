// FIFO queue via two stacks (in/out); amortized O(1) per op, O(n) space.

struct MyQueue {
    inn: Vec<i32>,
    out: Vec<i32>,
}

impl MyQueue {
    fn new() -> Self { MyQueue { inn: Vec::new(), out: Vec::new() } }
    fn enqueue(&mut self, x: i32) { self.inn.push(x); }
    fn dequeue(&mut self) -> i32 {
        if self.out.is_empty() {
            while let Some(v) = self.inn.pop() { self.out.push(v); }
        }
        self.out.pop().unwrap()
    }
}

fn main() {
    let mut q = MyQueue::new();
    q.enqueue(1); q.enqueue(2); q.enqueue(3);
    println!("{}", q.dequeue()); // 1
    q.enqueue(4);
    println!("{}", q.dequeue()); // 2
    println!("{}", q.dequeue()); // 3
    println!("{}", q.dequeue()); // 4
}
