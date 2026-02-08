// PeekableIterator: wrap an iterator and cache one element ahead so peek() returns
// the next value without consuming it. O(1) per op, O(1) extra space.

struct PeekableInterface<I: Iterator> {
    it: I,
    cached: Option<I::Item>,
}

impl<I: Iterator> PeekableInterface<I>
where
    I::Item: Copy,
{
    fn new(it: I) -> Self {
        PeekableInterface { it, cached: None }
    }
    fn has_next(&mut self) -> bool {
        if self.cached.is_some() {
            return true;
        }
        self.cached = self.it.next();
        self.cached.is_some()
    }
    fn next(&mut self) -> Option<I::Item> {
        if let Some(v) = self.cached.take() {
            return Some(v);
        }
        self.it.next()
    }
    fn peek(&mut self) -> Option<I::Item> {
        if self.cached.is_none() {
            self.cached = self.it.next();
        }
        self.cached
    }
}

fn main() {
    let mut it = PeekableInterface::new(vec![1, 2, 3].into_iter());
    println!("peek() -> {}", it.peek().unwrap());
    println!("next() -> {}", it.next().unwrap());
    println!("peek() -> {}", it.peek().unwrap());
    println!("next() -> {}", it.next().unwrap());
    println!("next() -> {}", it.next().unwrap());
    println!("hasNext() -> {}", it.has_next());
}
