// Peekable iterator wrapper: cache one element ahead. peek()/next()/hasNext() O(1) time, O(1) space.
struct Peekable<I: Iterator> {
    it: I,
    cached: Option<I::Item>,
}

impl<I: Iterator> Peekable<I>
where
    I::Item: Clone,
{
    fn new(it: I) -> Self {
        Peekable { it, cached: None }
    }
    fn has_next(&mut self) -> bool {
        if self.cached.is_some() {
            return true;
        }
        self.cached = self.it.next();
        self.cached.is_some()
    }
    fn peek(&mut self) -> Option<I::Item> {
        if self.cached.is_none() {
            self.cached = self.it.next();
        }
        self.cached.clone()
    }
    fn next(&mut self) -> Option<I::Item> {
        if self.cached.is_some() {
            return self.cached.take();
        }
        self.it.next()
    }
}

fn main() {
    let mut p = Peekable::new(vec![1, 2, 3].into_iter());
    println!("peek()    -> {}", p.peek().unwrap());
    println!("next()    -> {}", p.next().unwrap());
    println!("peek()    -> {}", p.peek().unwrap());
    println!("hasNext() -> {}", p.has_next());
    println!("next()    -> {}", p.next().unwrap());
    println!("next()    -> {}", p.next().unwrap());
    println!("hasNext() -> {}", p.has_next());
}
