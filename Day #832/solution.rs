// Longest subarray with all distinct elements.
// Sliding window with last-seen index map. Time: O(N), Space: O(N).
use std::collections::HashMap;

fn longest_distinct(a: &[i32]) -> usize {
    let mut last: HashMap<i32, usize> = HashMap::new();
    let mut best = 0usize;
    let mut start = 0usize;
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
    let a = [5, 1, 3, 5, 2, 3, 4, 1];
    println!("{}", longest_distinct(&a));
}
