// Day 1413: shortest substring of s containing all characters in a set.
// Approach: sliding window, expand right then shrink left while valid. O(n) time, O(k) space.
use std::collections::HashMap;

fn shortest_substring(s: &str, want: &[char]) -> Option<String> {
    let bytes: Vec<char> = s.chars().collect();
    let mut need: HashMap<char, i32> = HashMap::new();
    for &c in want {
        *need.entry(c).or_insert(0) += 1;
    }
    let required = need.len();
    let mut win: HashMap<char, i32> = HashMap::new();
    let mut formed = 0;
    let mut best_len = usize::MAX;
    let mut best_l = 0;
    let mut l = 0;
    for r in 0..bytes.len() {
        let c = bytes[r];
        if need.contains_key(&c) {
            let e = win.entry(c).or_insert(0);
            *e += 1;
            if *e == need[&c] {
                formed += 1;
            }
        }
        while formed == required {
            if r - l + 1 < best_len {
                best_len = r - l + 1;
                best_l = l;
            }
            let lc = bytes[l];
            if need.contains_key(&lc) {
                let e = win.entry(lc).or_insert(0);
                *e -= 1;
                if *e < need[&lc] {
                    formed -= 1;
                }
            }
            l += 1;
        }
    }
    if best_len == usize::MAX {
        None
    } else {
        Some(bytes[best_l..best_l + best_len].iter().collect())
    }
}

fn main() {
    match shortest_substring("figehaeci", &['a', 'e', 'i']) {
        Some(x) => println!("{}", x), // aeci
        None => println!("null"),
    }
}
