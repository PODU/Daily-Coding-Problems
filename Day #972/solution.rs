// Day 972: Rearrange string so no two adjacent chars match (else "None").
// Approach: place most frequent chars into even-then-odd slots. Time O(n log k), Space O(n).
use std::collections::HashMap;

fn rearrange(s: &str) -> Option<String> {
    let n = s.len();
    let mut cnt: HashMap<char, usize> = HashMap::new();
    for c in s.chars() {
        *cnt.entry(c).or_insert(0) += 1;
    }
    let mut v: Vec<(char, usize)> = cnt.into_iter().collect();
    v.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));
    if v[0].1 > (n + 1) / 2 {
        return None;
    }
    let mut res = vec![' '; n];
    let mut idx = 0;
    for (ch, c) in v {
        for _ in 0..c {
            res[idx] = ch;
            idx += 2;
            if idx >= n {
                idx = 1;
            }
        }
    }
    Some(res.into_iter().collect())
}

fn main() {
    println!("{}", rearrange("aaabbc").unwrap_or_else(|| "None".to_string())); // ababac
    println!("{}", rearrange("aaab").unwrap_or_else(|| "None".to_string()));   // None
}
