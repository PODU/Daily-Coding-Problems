// Day 489: Longest subarray with all distinct elements.
// Sliding window + last-seen map; move left past previous occurrence. Time O(n), Space O(n).
use std::collections::HashMap;

fn longest_distinct_subarray(a: &[i64]) -> usize {
    let mut last: HashMap<i64, usize> = HashMap::new();
    let mut left = 0usize;
    let mut best = 0usize;
    for right in 0..a.len() {
        if let Some(&prev) = last.get(&a[right]) {
            if prev >= left {
                left = prev + 1;
            }
        }
        last.insert(a[right], right);
        best = best.max(right - left + 1);
    }
    best
}

fn main() {
    let a = [5, 1, 3, 5, 2, 3, 4, 1];
    println!("{}", longest_distinct_subarray(&a)); // 5
}
