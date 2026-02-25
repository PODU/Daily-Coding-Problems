// Smallest window containing every distinct character of the string.
// Sliding window with frequency counts; expand then shrink. O(n) time, O(k) space.
use std::collections::{HashMap, HashSet};

fn smallest_window(s: &str) -> usize {
    let bytes = s.as_bytes();
    let need = bytes.iter().collect::<HashSet<_>>().len();
    let mut cnt: HashMap<u8, i32> = HashMap::new();
    let mut have = 0;
    let mut best = usize::MAX;
    let mut left = 0;
    for right in 0..bytes.len() {
        let e = cnt.entry(bytes[right]).or_insert(0);
        *e += 1;
        if *e == 1 {
            have += 1;
        }
        while have == need {
            if right - left + 1 < best {
                best = right - left + 1;
            }
            let le = cnt.entry(bytes[left]).or_insert(0);
            *le -= 1;
            if *le == 0 {
                have -= 1;
            }
            left += 1;
        }
    }
    best
}

fn main() {
    println!("{}", smallest_window("jiujitsu"));
}
