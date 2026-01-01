// Day 835: Shortest substring containing all chars in a set.
// Sliding-window min-window: expand right, contract left while valid. O(N) time, O(K) space.
use std::collections::{HashMap, HashSet};

fn shortest_substring(s: &str, charset: &HashSet<char>) -> Option<String> {
    let bytes: Vec<char> = s.chars().collect();
    let mut need: HashMap<char, i32> = HashMap::new();
    for &c in charset {
        need.insert(c, 1);
    }
    let mut have: HashMap<char, i32> = HashMap::new();
    let required = need.len();
    let mut formed = 0usize;
    let mut left = 0usize;
    let mut best: Option<(usize, usize)> = None;
    for right in 0..bytes.len() {
        let ch = bytes[right];
        if need.contains_key(&ch) {
            let e = have.entry(ch).or_insert(0);
            *e += 1;
            if *e == need[&ch] {
                formed += 1;
            }
        }
        while formed == required {
            let len = right - left + 1;
            if best.is_none() || len < best.unwrap().1 {
                best = Some((left, len));
            }
            let lc = bytes[left];
            if need.contains_key(&lc) {
                let e = have.get_mut(&lc).unwrap();
                *e -= 1;
                if *e < need[&lc] {
                    formed -= 1;
                }
            }
            left += 1;
        }
    }
    best.map(|(l, len)| bytes[l..l + len].iter().collect())
}

fn main() {
    let charset: HashSet<char> = ['a', 'e', 'i'].iter().cloned().collect();
    match shortest_substring("figehaeci", &charset) {
        Some(r) => println!("{}", r),
        None => println!("null"),
    }
}
