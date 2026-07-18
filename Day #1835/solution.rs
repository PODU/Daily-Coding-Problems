// Running median with two heaps (max-heap for lower half, min-heap for upper).
// O(log n) per element.
use std::collections::BinaryHeap;
use std::cmp::Reverse;

fn main() {
    let stream = [2, 1, 5, 7, 2, 0, 5];
    let mut lo: BinaryHeap<i64> = BinaryHeap::new();           // max-heap (lower half)
    let mut hi: BinaryHeap<Reverse<i64>> = BinaryHeap::new();  // min-heap (upper half)

    for &x in stream.iter() {
        let x = x as i64;
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

        let m: f64 = if lo.len() > hi.len() {
            *lo.peek().unwrap() as f64
        } else {
            (*lo.peek().unwrap() + hi.peek().unwrap().0) as f64 / 2.0
        };
        if (m - m.round()).abs() < 1e-9 {
            println!("{}", m.round() as i64);
        } else {
            println!("{}", m);
        }
    }
}
