// Day 510: All index pairs (i,j) where words[i]+words[j] is a palindrome.
// Hash map of reversed words + prefix/suffix palindrome checks. Time O(N*L^2).
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

fn palindrome_pairs(words: &[&str]) -> Vec<(usize, usize)> {
    let mut rev: HashMap<String, usize> = HashMap::new();
    for (i, w) in words.iter().enumerate() {
        rev.insert(w.chars().rev().collect(), i);
    }
    let mut seen: HashSet<(usize, usize)> = HashSet::new();
    let mut result: Vec<(usize, usize)> = Vec::new();
    for (i, w) in words.iter().enumerate() {
        let n = w.len();
        for cut in 0..=n {
            if is_pal(&w[..cut]) {
                if let Some(&j) = rev.get(&w[cut..]) {
                    if j != i && seen.insert((j, i)) {
                        result.push((j, i));
                    }
                }
            }
            if cut != n && is_pal(&w[cut..]) {
                if let Some(&j) = rev.get(&w[..cut]) {
                    if j != i && seen.insert((i, j)) {
                        result.push((i, j));
                    }
                }
            }
        }
    }
    result.sort();
    result
}

fn main() {
    let words = ["code", "edoc", "da", "d"];
    let pairs = palindrome_pairs(&words);
    let parts: Vec<String> = pairs.iter().map(|p| format!("({}, {})", p.0, p.1)).collect();
    println!("[{}]", parts.join(", "));
}
