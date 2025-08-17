// Wrap an iterator and buffer one element for peek(). next/hasNext/peek all O(1).
// Time O(1) per op; Space O(1).

struct SimpleIterator {
    data: Vec<i32>,
    idx: usize,
}

impl SimpleIterator {
    fn new(data: Vec<i32>) -> Self {
        SimpleIterator { data, idx: 0 }
    }
    fn next(&mut self) -> i32 {
        let v = self.data[self.idx];
        self.idx += 1;
        v
    }
    fn has_next(&self) -> bool {
        self.idx < self.data.len()
    }
}

struct PeekableInterface {
    it: SimpleIterator,
    buffer: Option<i32>,
}

impl PeekableInterface {
    fn new(it: SimpleIterator) -> Self {
        PeekableInterface { it, buffer: None }
    }
    fn peek(&mut self) -> i32 {
        if self.buffer.is_none() {
            self.buffer = Some(self.it.next());
        }
        self.buffer.unwrap()
    }
    fn next(&mut self) -> i32 {
        if let Some(v) = self.buffer.take() {
            v
        } else {
            self.it.next()
        }
    }
    fn has_next(&self) -> bool {
        self.buffer.is_some() || self.it.has_next()
    }
}

fn main() {
    let mut p = PeekableInterface::new(SimpleIterator::new(vec![1, 2, 3]));
    println!(
        "peek={} next={} peek={} next={} next={} hasNext={}",
        p.peek(),
        p.next(),
        p.peek(),
        p.next(),
        p.next(),
        p.has_next()
    );
    // peek=1 next=1 peek=2 next=2 next=3 hasNext=false
}
