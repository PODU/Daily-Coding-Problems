// Palindrome pairs: map word->index, split each word, match palindromic halves.
// Time O(N*L^2), Space O(N*L).
use std::collections::{HashMap, HashSet};

fn is_pal(s: &str) -> bool {
    let b = s.as_bytes();
    let (mut l, mut r) = (0isize, b.len() as isize - 1);
    while l < r {
        if b[l as usize] != b[r as usize] {
            return false;
        }
        l += 1;
        r -= 1;
    }
    true
}

fn reverse(s: &str) -> String {
    s.chars().rev().collect()
}

fn palindrome_pairs(words: &[&str]) -> Vec<(usize, usize)> {
    let mut idx: HashMap<&str, usize> = HashMap::new();
    for (i, w) in words.iter().enumerate() {
        idx.insert(w, i);
    }
    let mut seen: HashSet<(usize, usize)> = HashSet::new();
    let mut res: Vec<(usize, usize)> = Vec::new();
    for (i, w) in words.iter().enumerate() {
        let n = w.len();
        for j in 0..=n {
            let prefix = &w[..j];
            let suffix = &w[j..];
            if is_pal(prefix) {
                let rs = reverse(suffix);
                if let Some(&k) = idx.get(rs.as_str()) {
                    if k != i && seen.insert((k, i)) {
                        res.push((k, i));
                    }
                }
            }
            if !suffix.is_empty() && is_pal(suffix) {
                let rp = reverse(prefix);
                if let Some(&k) = idx.get(rp.as_str()) {
                    if k != i && seen.insert((i, k)) {
                        res.push((i, k));
                    }
                }
            }
        }
    }
    res.sort();
    res
}

fn main() {
    let words = ["code", "edoc", "da", "d"];
    let res = palindrome_pairs(&words);
    let parts: Vec<String> = res.iter().map(|p| format!("({}, {})", p.0, p.1)).collect();
    println!("[{}]", parts.join(", "));
}
