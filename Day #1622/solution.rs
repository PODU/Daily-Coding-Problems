// Day 1622: Longest substring with at most k distinct characters.
// Sliding window with a count map. Time O(n), Space O(k).
use std::collections::HashMap;

fn longest_k_distinct(s: &str, k: usize) -> String {
    if k == 0 {
        return String::new();
    }
    let b = s.as_bytes();
    let mut cnt: HashMap<u8, i32> = HashMap::new();
    let (mut left, mut best_l, mut best_len) = (0usize, 0usize, 0usize);
    for right in 0..b.len() {
        *cnt.entry(b[right]).or_insert(0) += 1;
        while cnt.len() > k {
            let e = cnt.get_mut(&b[left]).unwrap();
            *e -= 1;
            if *e == 0 {
                cnt.remove(&b[left]);
            }
            left += 1;
        }
        if right - left + 1 > best_len {
            best_len = right - left + 1;
            best_l = left;
        }
    }
    s[best_l..best_l + best_len].to_string()
}

fn main() {
    println!("\"{}\"", longest_k_distinct("abcba", 2));
}
