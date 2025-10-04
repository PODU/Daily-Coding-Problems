// Day 367: Lazily merge two sorted iterators into one sorted iterator.
// A custom Iterator peeks each source via Peekable and yields the smaller head;
// nothing is buffered into memory. Time O(n+m), Space O(1).
use std::iter::Peekable;

struct Merge<A: Iterator<Item = i32>, B: Iterator<Item = i32>> {
    a: Peekable<A>,
    b: Peekable<B>,
}

impl<A: Iterator<Item = i32>, B: Iterator<Item = i32>> Iterator for Merge<A, B> {
    type Item = i32;
    fn next(&mut self) -> Option<i32> {
        match (self.a.peek(), self.b.peek()) {
            (Some(&x), Some(&y)) => {
                if x <= y { self.a.next() } else { self.b.next() }
            }
            (Some(_), None) => self.a.next(),
            (None, Some(_)) => self.b.next(),
            (None, None) => None,
        }
    }
}

fn merge_iterators<A: Iterator<Item = i32>, B: Iterator<Item = i32>>(a: A, b: B) -> Merge<A, B> {
    Merge { a: a.peekable(), b: b.peekable() }
}

fn main() {
    let foo = vec![5, 10, 15].into_iter();
    let bar = vec![3, 8, 9].into_iter();
    for num in merge_iterators(foo, bar) {
        println!("{}", num); // 3 5 8 9 10 15
    }
}
