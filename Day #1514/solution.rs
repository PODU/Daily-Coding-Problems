// Sliding window with last-seen index map; advance left past duplicates, track max window length.
// Time: O(n), Space: O(n).
use std::collections::HashMap;

fn longest_distinct(a: &[i32]) -> usize {
    let mut last: HashMap<i32, usize> = HashMap::new();
    let mut best = 0usize;
    let mut left = 0usize;
    for (r, &v) in a.iter().enumerate() {
        if let Some(&prev) = last.get(&v) {
            if prev >= left {
                left = prev + 1;
            }
        }
        last.insert(v, r);
        best = best.max(r - left + 1);
    }
    best
}

fn main() {
    let a = [5, 1, 3, 5, 2, 3, 4, 1];
    println!("{}", longest_distinct(&a));
}
