// Running median with two heaps (max-heap lower half, min-heap upper half). O(log n) per insert.
use std::cmp::Reverse;
use std::collections::BinaryHeap;

fn main() {
    let stream = [2, 1, 5, 7, 2, 0, 5];
    let mut lo: BinaryHeap<i32> = BinaryHeap::new();          // max-heap (lower half)
    let mut hi: BinaryHeap<Reverse<i32>> = BinaryHeap::new(); // min-heap (upper half)
    for &x in stream.iter() {
        if lo.is_empty() || x <= *lo.peek().unwrap() {
            lo.push(x);
        } else {
            hi.push(Reverse(x));
        }
        if lo.len() > hi.len() + 1 {
            let t = lo.pop().unwrap();
            hi.push(Reverse(t));
        } else if hi.len() > lo.len() {
            let Reverse(t) = hi.pop().unwrap();
            lo.push(t);
        }
        let m: f64 = if lo.len() == hi.len() {
            let Reverse(h) = *hi.peek().unwrap();
            (*lo.peek().unwrap() as f64 + h as f64) / 2.0
        } else {
            *lo.peek().unwrap() as f64
        };
        if m.fract() == 0.0 {
            println!("{}", m as i64);
        } else {
            println!("{}", m);
        }
    }
}
