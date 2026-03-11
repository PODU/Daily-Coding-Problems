// Smallest window containing all distinct chars of the string. Sliding window:
// expand right, shrink left while all distinct kinds present. O(N) time, O(K) space.
use std::collections::{HashMap, HashSet};

fn smallest_window(s: &str) -> usize {
    let chars: Vec<char> = s.chars().collect();
    let distinct = chars.iter().cloned().collect::<HashSet<char>>().len();

    let mut cnt: HashMap<char, i32> = HashMap::new();
    let mut have = 0usize;
    let mut left = 0usize;
    let mut best = usize::MAX;
    for right in 0..chars.len() {
        let c = chars[right];
        let e = cnt.entry(c).or_insert(0);
        *e += 1;
        if *e == 1 {
            have += 1;
        }
        while have == distinct {
            if right - left + 1 < best {
                best = right - left + 1;
            }
            let lc = chars[left];
            let le = cnt.entry(lc).or_insert(0);
            *le -= 1;
            if *le == 0 {
                have -= 1;
            }
            left += 1;
        }
    }
    if best == usize::MAX {
        0
    } else {
        best
    }
}

fn main() {
    println!("{}", smallest_window("jiujitsu"));
}
