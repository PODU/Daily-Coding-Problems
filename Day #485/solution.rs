// Day 485: Longest substring with at most k distinct characters.
// Sliding window + count map; expand right, shrink left when distinct > k. Time O(n), Space O(k).
use std::collections::HashMap;

fn longest_k_distinct(s: &str, k: usize) -> usize {
    if k == 0 {
        return 0;
    }
    let bytes = s.as_bytes();
    let mut count: HashMap<u8, usize> = HashMap::new();
    let mut left = 0usize;
    let mut best = 0usize;
    for right in 0..bytes.len() {
        *count.entry(bytes[right]).or_insert(0) += 1;
        while count.len() > k {
            let lc = bytes[left];
            let e = count.get_mut(&lc).unwrap();
            *e -= 1;
            if *e == 0 {
                count.remove(&lc);
            }
            left += 1;
        }
        best = best.max(right - left + 1);
    }
    best
}

fn main() {
    println!("{}", longest_k_distinct("abcba", 2)); // 3
}
