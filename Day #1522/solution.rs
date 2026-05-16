// Concatenation of all equal-length words: sliding window per offset (0..L-1).
// Time O(|s| * L), Space O(words). Each word used exactly once.
use std::collections::HashMap;

fn find_substring(s: &str, words: &[&str]) -> Vec<usize> {
    let mut res = Vec::new();
    if words.is_empty() {
        return res;
    }
    let sb = s.as_bytes();
    let l = words[0].len();
    let k = words.len();
    let n = sb.len();
    if l * k > n {
        return res;
    }
    let mut need: HashMap<&[u8], i32> = HashMap::new();
    for w in words {
        *need.entry(w.as_bytes()).or_insert(0) += 1;
    }
    for off in 0..l {
        let mut left = off;
        let mut count = 0;
        let mut win: HashMap<&[u8], i32> = HashMap::new();
        let mut j = off;
        while j + l <= n {
            let w = &sb[j..j + l];
            if need.contains_key(w) {
                *win.entry(w).or_insert(0) += 1;
                count += 1;
                while win[w] > need[w] {
                    let lw = &sb[left..left + l];
                    *win.get_mut(lw).unwrap() -= 1;
                    left += l;
                    count -= 1;
                }
                if count == k {
                    res.push(left);
                    let lw = &sb[left..left + l];
                    *win.get_mut(lw).unwrap() -= 1;
                    left += l;
                    count -= 1;
                }
            } else {
                win.clear();
                count = 0;
                left = j + l;
            }
            j += l;
        }
    }
    res.sort();
    res
}

fn main() {
    println!("{:?}", find_substring("dogcatcatcodecatdog", &["cat", "dog"]));
    println!("{:?}", find_substring("barfoobazbitbyte", &["dog", "cat"]));
}
