// Day 858: Running median of a stream.
// Approach: max-heap (lower half) + min-heap (upper half), balanced sizes.
// Time: O(n log n) total, Space: O(n).
use std::cmp::Reverse;
use std::collections::BinaryHeap;

fn main() {
    let stream = [2, 1, 5, 7, 2, 0, 5];
    let mut lo: BinaryHeap<i64> = BinaryHeap::new();          // max-heap
    let mut hi: BinaryHeap<Reverse<i64>> = BinaryHeap::new(); // min-heap

    for &x in &stream {
        let x = x as i64;
        if lo.is_empty() || x <= *lo.peek().unwrap() {
            lo.push(x);
        } else {
            hi.push(Reverse(x));
        }
        if lo.len() > hi.len() + 1 {
            hi.push(Reverse(lo.pop().unwrap()));
        } else if hi.len() > lo.len() {
            lo.push(hi.pop().unwrap().0);
        }
        let med: f64 = if lo.len() > hi.len() {
            *lo.peek().unwrap() as f64
        } else {
            (*lo.peek().unwrap() + hi.peek().unwrap().0) as f64 / 2.0
        };
        if med.fract() == 0.0 {
            println!("{}", med as i64);
        } else {
            println!("{}", med);
        }
    }
}
