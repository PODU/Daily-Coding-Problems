// Day 491: Running median of a stream.
// Two heaps: a max-heap keeps the lower half, a min-heap the upper half; rebalance so
// the lower half has equal size or one extra, so the median is its top.
// Time: O(log n) per element, Space: O(n).
use std::cmp::Reverse;
use std::collections::BinaryHeap;

fn main() {
    let stream = [2, 1, 5, 7, 2, 0, 5];
    let mut lo: BinaryHeap<i32> = BinaryHeap::new(); // max-heap: lower half
    let mut hi: BinaryHeap<Reverse<i32>> = BinaryHeap::new(); // min-heap: upper half
    for &x in &stream {
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
        let m = if lo.len() == hi.len() {
            (*lo.peek().unwrap() as f64 + hi.peek().unwrap().0 as f64) / 2.0
        } else {
            *lo.peek().unwrap() as f64
        };
        // 2, 1.5, 2, 3.5, 2, 2, 2
        if m.fract() == 0.0 {
            println!("{}", m as i64);
        } else {
            println!("{}", m);
        }
    }
}
