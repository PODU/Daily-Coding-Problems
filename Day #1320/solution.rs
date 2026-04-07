// Longest substring with at most k distinct characters via a sliding window
// with a char-count map. Time O(n), Space O(k).
use std::collections::HashMap;

fn longest_k_distinct(s: &str, k: usize) -> String {
    let b = s.as_bytes();
    let mut cnt: HashMap<u8, i32> = HashMap::new();
    let (mut left, mut best_start, mut best_len) = (0usize, 0usize, 0usize);
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
            best_start = left;
        }
    }
    s[best_start..best_start + best_len].to_string()
}

fn main() {
    let sub = longest_k_distinct("abcba", 2);
    println!("The longest substring with k distinct characters is \"{}\".", sub);
}
