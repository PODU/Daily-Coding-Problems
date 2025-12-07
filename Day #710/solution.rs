// Day 710: Find start indices where s contains a concatenation of all equal-length
// words exactly once. Sliding window over wordLen offsets. Time O(n*wordLen).
use std::collections::HashMap;

fn find_substring(s: &str, words: &[&str]) -> Vec<usize> {
    let mut res = Vec::new();
    if words.is_empty() {
        return res;
    }
    let b = s.as_bytes();
    let wl = words[0].len();
    let k = words.len();
    let n = b.len();
    if wl * k > n {
        return res;
    }
    let mut need: HashMap<&[u8], i32> = HashMap::new();
    for w in words {
        *need.entry(w.as_bytes()).or_insert(0) += 1;
    }
    for off in 0..wl {
        let mut left = off;
        let mut count = 0;
        let mut window: HashMap<&[u8], i32> = HashMap::new();
        let mut j = off;
        while j + wl <= n {
            let w = &b[j..j + wl];
            if need.contains_key(w) {
                *window.entry(w).or_insert(0) += 1;
                count += 1;
                while window[w] > need[w] {
                    let lw = &b[left..left + wl];
                    *window.get_mut(lw).unwrap() -= 1;
                    left += wl;
                    count -= 1;
                }
                if count == k {
                    res.push(left);
                    let lw = &b[left..left + wl];
                    *window.get_mut(lw).unwrap() -= 1;
                    left += wl;
                    count -= 1;
                }
            } else {
                window.clear();
                count = 0;
                left = j + wl;
            }
            j += wl;
        }
    }
    res.sort();
    res
}

fn main() {
    println!("{:?}", find_substring("dogcatcatcodecatdog", &["cat", "dog"]));
    println!("{:?}", find_substring("barfoobazbitbyte", &["dog", "cat"]));
}
