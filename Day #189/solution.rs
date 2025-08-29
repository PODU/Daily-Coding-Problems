// Day 189: Longest contiguous subarray with all distinct elements.
// Sliding window with last-seen map. Time O(n), Space O(n).
use std::collections::HashMap;

fn longest_distinct(a: &[i32]) -> usize {
    let mut last: HashMap<i32, usize> = HashMap::new();
    let mut best = 0;
    let mut start = 0;
    for (i, &x) in a.iter().enumerate() {
        if let Some(&p) = last.get(&x) {
            if p >= start {
                start = p + 1;
            }
        }
        last.insert(x, i);
        best = best.max(i - start + 1);
    }
    best
}

fn main() {
    println!("{}", longest_distinct(&[5, 1, 3, 5, 2, 3, 4, 1]));
}
