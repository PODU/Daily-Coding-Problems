// Day 1436: Length of longest subarray with all distinct elements.
// Approach: Sliding window with last-seen index map; shrink left on repeat.
// Time: O(n), Space: O(n).
use std::collections::HashMap;

fn longest_distinct(arr: &[i32]) -> usize {
    let mut last: HashMap<i32, usize> = HashMap::new();
    let mut best = 0;
    let mut left = 0;
    for (right, &v) in arr.iter().enumerate() {
        if let Some(&prev) = last.get(&v) {
            if prev >= left {
                left = prev + 1;
            }
        }
        last.insert(v, right);
        best = best.max(right - left + 1);
    }
    best
}

fn main() {
    println!("{}", longest_distinct(&[5, 1, 3, 5, 2, 3, 4, 1])); // 5
}
