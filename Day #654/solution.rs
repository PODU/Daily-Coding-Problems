// Smallest window containing every distinct char: sliding window with two pointers.
// Expand right until all distinct chars present, shrink left to minimize. Time O(n), space O(k).
use std::collections::{HashMap, HashSet};

fn smallest_window(s: &str) -> usize {
    let chars: Vec<char> = s.chars().collect();
    let need = chars.iter().collect::<HashSet<_>>().len();
    let mut cnt: HashMap<char, i32> = HashMap::new();
    let mut have = 0usize;
    let mut best = usize::MAX;
    let mut left = 0usize;
    for right in 0..chars.len() {
        let e = cnt.entry(chars[right]).or_insert(0);
        *e += 1;
        if *e == 1 {
            have += 1;
        }
        while have == need {
            if right - left + 1 < best {
                best = right - left + 1;
            }
            let le = cnt.entry(chars[left]).or_insert(0);
            *le -= 1;
            if *le == 0 {
                have -= 1;
            }
            left += 1;
        }
    }
    if best == usize::MAX { 0 } else { best }
}

fn main() {
    println!("{}", smallest_window("jiujitsu"));
}
