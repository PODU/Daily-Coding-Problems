// Smallest window containing all distinct chars: O(n) sliding window.
// Time O(n), Space O(alphabet).
use std::collections::{HashMap, HashSet};

fn smallest_window(s: &str) -> usize {
    let chars: Vec<char> = s.chars().collect();
    let need = chars.iter().cloned().collect::<HashSet<char>>().len();
    let mut cnt: HashMap<char, i32> = HashMap::new();
    let (mut have, mut left, mut best) = (0usize, 0usize, usize::MAX);
    for right in 0..chars.len() {
        let e = cnt.entry(chars[right]).or_insert(0);
        *e += 1;
        if *e == 1 { have += 1; }
        while have == need {
            best = best.min(right - left + 1);
            let le = cnt.entry(chars[left]).or_insert(0);
            *le -= 1;
            if *le == 0 { have -= 1; }
            left += 1;
        }
    }
    best
}

fn main() {
    println!("{}", smallest_window("jiujitsu"));
}
