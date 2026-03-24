// Day 1261: Palindrome pairs.
// Hashmap of words + prefix/suffix palindrome splits. O(n*k^2) time, O(n*k) space.
use std::collections::HashMap;

fn is_pal(s: &[u8], mut i: i32, mut j: i32) -> bool {
    while i < j {
        if s[i as usize] != s[j as usize] {
            return false;
        }
        i += 1;
        j -= 1;
    }
    true
}

fn palindrome_pairs(words: &[&str]) -> Vec<(usize, usize)> {
    let idx: HashMap<&str, usize> = words.iter().enumerate().map(|(i, &w)| (w, i)).collect();
    let mut res: Vec<(usize, usize)> = Vec::new();
    for (i, &w) in words.iter().enumerate() {
        let b = w.as_bytes();
        let n = b.len();
        for j in 0..=n {
            if is_pal(b, 0, j as i32 - 1) {
                let back: String = w[j..].chars().rev().collect();
                if let Some(&k) = idx.get(back.as_str()) {
                    if k != i {
                        res.push((k, i));
                    }
                }
            }
            if j != n && is_pal(b, j as i32, n as i32 - 1) {
                let back: String = w[..j].chars().rev().collect();
                if let Some(&k) = idx.get(back.as_str()) {
                    if k != i {
                        res.push((i, k));
                    }
                }
            }
        }
    }
    res.sort();
    res.dedup();
    res
}

fn main() {
    let res = palindrome_pairs(&["code", "edoc", "da", "d"]);
    let parts: Vec<String> = res.iter().map(|(a, b)| format!("({}, {})", a, b)).collect();
    println!("[{}]", parts.join(", "));
}
