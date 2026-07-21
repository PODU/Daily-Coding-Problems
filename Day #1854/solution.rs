// Day 1854: Shortest substring containing all chars in a set.
// Sliding window with a need-counter; expand right, contract left while valid. O(n) time, O(|set|) space.
use std::collections::{HashMap, HashSet};

fn shortest_substring(s: &str, need: &HashSet<char>) -> Option<String> {
    let bytes: Vec<char> = s.chars().collect();
    let mut want: HashMap<char, i32> = HashMap::new();
    for &c in need {
        *want.entry(c).or_insert(0) += 1;
    }
    let required = want.len();
    let mut win: HashMap<char, i32> = HashMap::new();
    let mut formed = 0usize;
    let mut best_len = usize::MAX;
    let mut best_l = 0usize;
    let mut l = 0usize;
    for r in 0..bytes.len() {
        let c = bytes[r];
        if want.contains_key(&c) {
            let e = win.entry(c).or_insert(0);
            *e += 1;
            if *e == want[&c] {
                formed += 1;
            }
        }
        while formed == required {
            if r - l + 1 < best_len {
                best_len = r - l + 1;
                best_l = l;
            }
            let lc = bytes[l];
            l += 1;
            if want.contains_key(&lc) {
                let e = win.get_mut(&lc).unwrap();
                *e -= 1;
                if *e < want[&lc] {
                    formed -= 1;
                }
            }
        }
    }
    if best_len == usize::MAX {
        None
    } else {
        Some(bytes[best_l..best_l + best_len].iter().collect())
    }
}

fn main() {
    let need: HashSet<char> = ['a', 'e', 'i'].into_iter().collect();
    match shortest_substring("figehaeci", &need) {
        Some(r) => println!("{}", r), // aeci
        None => println!("null"),
    }
}
