// Day 437: Shortest substring containing all chars of a set via sliding window.
// O(N) time, O(set) space.
use std::collections::{HashMap, HashSet};

fn shortest_substring(s: &str, chars: &HashSet<char>) -> Option<String> {
    if chars.is_empty() {
        return Some(String::new());
    }
    let bytes: Vec<char> = s.chars().collect();
    let mut need: HashMap<char, i32> = chars.iter().map(|&c| (c, 0)).collect();
    let required = chars.len();
    let mut formed = 0usize;
    let mut best: Option<(usize, usize)> = None; // (start, len)
    let mut l = 0usize;
    for r in 0..bytes.len() {
        let c = bytes[r];
        if let Some(v) = need.get_mut(&c) {
            if *v == 0 {
                formed += 1;
            }
            *v += 1;
        }
        while formed == required {
            let len = r - l + 1;
            if best.map_or(true, |(_, bl)| len < bl) {
                best = Some((l, len));
            }
            let lc = bytes[l];
            if let Some(v) = need.get_mut(&lc) {
                *v -= 1;
                if *v == 0 {
                    formed -= 1;
                }
            }
            l += 1;
        }
    }
    best.map(|(st, len)| bytes[st..st + len].iter().collect())
}

fn main() {
    let chars: HashSet<char> = ['a', 'e', 'i'].into_iter().collect();
    match shortest_substring("figehaeci", &chars) {
        Some(res) => println!("\"{}\"", res), // "aeci"
        None => println!("null"),
    }
}
