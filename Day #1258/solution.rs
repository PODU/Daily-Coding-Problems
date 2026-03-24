// Peeking iterator: buffer one element ahead. peek/next/hasNext all O(1) time, O(1) extra space.

struct Iter {
    data: Vec<i32>,
    idx: usize,
}

impl Iter {
    fn new(v: Vec<i32>) -> Self { Iter { data: v, idx: 0 } }
    fn next(&mut self) -> i32 { let v = self.data[self.idx]; self.idx += 1; v }
    fn has_next(&self) -> bool { self.idx < self.data.len() }
}

struct PeekableInterface {
    it: Iter,
    buffer: Option<i32>,
}

impl PeekableInterface {
    fn new(it: Iter) -> Self { PeekableInterface { it, buffer: None } }
    fn peek(&mut self) -> i32 {
        if self.buffer.is_none() {
            self.buffer = Some(self.it.next());
        }
        self.buffer.unwrap()
    }
    fn next(&mut self) -> i32 {
        if let Some(v) = self.buffer.take() { return v; }
        self.it.next()
    }
    fn has_next(&self) -> bool { self.buffer.is_some() || self.it.has_next() }
}

fn main() {
    let mut p = PeekableInterface::new(Iter::new(vec![1, 2, 3]));
    println!("{}", p.peek());
    println!("{}", p.next());
    println!("{}", p.next());
    println!("{}", p.peek());
    println!("{}", p.has_next());
    println!("{}", p.next());
    println!("{}", p.has_next());
}
