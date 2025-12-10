// Day 727: Running median over a stream.
// Approach: Max-heap for lower half, min-heap (via Reverse) for upper half, balanced.
// Time: O(log n) per element, Space: O(n).
use std::collections::BinaryHeap;
use std::cmp::Reverse;

fn print_median(m: f64) {
    if m.fract() == 0.0 {
        println!("{}", m as i64);
    } else {
        println!("{:.1}", m);
    }
}

fn main() {
    let stream = [2, 1, 5, 7, 2, 0, 5];
    let mut lo: BinaryHeap<i32> = BinaryHeap::new();          // max-heap
    let mut hi: BinaryHeap<Reverse<i32>> = BinaryHeap::new(); // min-heap
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
        if lo.len() == hi.len() {
            print_median((*lo.peek().unwrap() + hi.peek().unwrap().0) as f64 / 2.0);
        } else {
            print_median(*lo.peek().unwrap() as f64);
        }
    }
}
