// Peekable iterator: cache one element ahead so peek() returns next without advancing.
// next/peek/has_next all O(1).

struct Peekable<I: Iterator> {
    it: I,
    cached: Option<I::Item>,
}

impl<I: Iterator> Peekable<I> {
    fn new(it: I) -> Self { Peekable { it, cached: None } }

    fn has_next(&mut self) -> bool {
        if self.cached.is_none() {
            self.cached = self.it.next();
        }
        self.cached.is_some()
    }

    fn peek(&mut self) -> Option<&I::Item> {
        if self.cached.is_none() {
            self.cached = self.it.next();
        }
        self.cached.as_ref()
    }

    fn next(&mut self) -> Option<I::Item> {
        if self.cached.is_some() {
            self.cached.take()
        } else {
            self.it.next()
        }
    }
}

fn main() {
    let mut it = Peekable::new(vec![1, 2, 3, 4].into_iter());
    println!("{}", it.peek().unwrap());   // 1
    println!("{}", it.next().unwrap());   // 1
    println!("{}", it.next().unwrap());   // 2
    println!("{}", it.peek().unwrap());   // 3
    println!("{}", it.next().unwrap());   // 3
    println!("{}", it.has_next());        // true
    println!("{}", it.next().unwrap());   // 4
    println!("{}", it.has_next());        // false
}
