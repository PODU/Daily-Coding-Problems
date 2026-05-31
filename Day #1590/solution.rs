// Smallest window containing every distinct char: sliding window with counts.
// Time O(n), Space O(k).
use std::collections::HashMap;
use std::collections::HashSet;

fn smallest_window(s: &str) -> usize {
    let chars: Vec<char> = s.chars().collect();
    let need = chars.iter().cloned().collect::<HashSet<char>>().len();
    let mut cnt: HashMap<char, i32> = HashMap::new();
    let (mut formed, mut left, mut best) = (0usize, 0usize, chars.len());
    for right in 0..chars.len() {
        let e = cnt.entry(chars[right]).or_insert(0);
        *e += 1;
        if *e == 1 {
            formed += 1;
        }
        while formed == need {
            if right - left + 1 < best {
                best = right - left + 1;
            }
            let le = cnt.get_mut(&chars[left]).unwrap();
            *le -= 1;
            if *le == 0 {
                formed -= 1;
            }
            left += 1;
        }
    }
    best
}

fn main() {
    println!("{}", smallest_window("jiujitsu"));
}
