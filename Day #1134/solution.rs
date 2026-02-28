// Running median with two heaps (max-heap low, min-heap high). O(log n) per insert.
use std::collections::BinaryHeap;
use std::cmp::Reverse;

fn fmt_median_pair(a: i64, b: i64) -> String {
    let sum = a + b;
    if sum % 2 == 0 {
        (sum / 2).to_string()
    } else {
        (sum as f64 / 2.0).to_string()
    }
}

fn main() {
    let nums = [2, 1, 5, 7, 2, 0, 5];
    let mut low: BinaryHeap<i64> = BinaryHeap::new();          // max-heap
    let mut high: BinaryHeap<Reverse<i64>> = BinaryHeap::new(); // min-heap

    for &x in nums.iter() {
        let x = x as i64;
        if low.is_empty() || x <= *low.peek().unwrap() {
            low.push(x);
        } else {
            high.push(Reverse(x));
        }
        if low.len() > high.len() + 1 {
            let v = low.pop().unwrap();
            high.push(Reverse(v));
        } else if high.len() > low.len() {
            let Reverse(v) = high.pop().unwrap();
            low.push(v);
        }

        if low.len() == high.len() {
            let a = *low.peek().unwrap();
            let Reverse(b) = *high.peek().unwrap();
            println!("{}", fmt_median_pair(a, b));
        } else {
            println!("{}", low.peek().unwrap());
        }
    }
}
