// Longest substring with at most k distinct chars: sliding window + count map.
// Time: O(n), Space: O(k).
use std::collections::HashMap;

fn longest_k_distinct(s: &str, k: usize) -> usize {
    if k == 0 {
        return 0;
    }
    let b = s.as_bytes();
    let mut cnt: HashMap<u8, usize> = HashMap::new();
    let mut left = 0;
    let mut best = 0;
    for right in 0..b.len() {
        *cnt.entry(b[right]).or_insert(0) += 1;
        while cnt.len() > k {
            let lc = b[left];
            let e = cnt.get_mut(&lc).unwrap();
            *e -= 1;
            if *e == 0 {
                cnt.remove(&lc);
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
