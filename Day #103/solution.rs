// Day 103: Shortest window containing all chars of a set via sliding window with
// a required-count and a "have all" tracker. O(n) time, O(set) space.
use std::collections::{HashMap, HashSet};

fn min_window(s: &str, chars: &[char]) -> Option<String> {
    let need: HashSet<char> = chars.iter().copied().collect();
    if need.is_empty() {
        return Some(String::new());
    }
    let bytes: Vec<char> = s.chars().collect();
    let mut count: HashMap<char, i32> = HashMap::new();
    let (mut have, mut left) = (0usize, 0usize);
    let mut best: Option<(usize, usize)> = None;
    for right in 0..bytes.len() {
        let ch = bytes[right];
        if need.contains(&ch) {
            let e = count.entry(ch).or_insert(0);
            *e += 1;
            if *e == 1 {
                have += 1;
            }
        }
        while have == need.len() {
            if best.map_or(true, |(a, b)| right - left + 1 < b - a + 1) {
                best = Some((left, right));
            }
            let lc = bytes[left];
            if need.contains(&lc) {
                let e = count.get_mut(&lc).unwrap();
                *e -= 1;
                if *e == 0 {
                    have -= 1;
                }
            }
            left += 1;
        }
    }
    best.map(|(a, b)| bytes[a..=b].iter().collect())
}

fn main() {
    println!("{:?}", min_window("figehaeci", &['a', 'e', 'i'])); // Some("aeci")
}
