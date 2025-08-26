// Substring concatenation of all words: sliding window over wordLen offsets with hash-map counts.
// Time O(n * wordLen), Space O(words * wordLen).
use std::collections::HashMap;

fn find_substring(s: &str, words: &[&str]) -> Vec<usize> {
    let mut res: Vec<usize> = Vec::new();
    if words.is_empty() || s.is_empty() {
        return res;
    }
    let b = s.as_bytes();
    let wl = words[0].len();
    let nw = words.len();
    let total = wl * nw;
    let n = b.len();
    if total > n {
        return res;
    }
    let mut need: HashMap<&[u8], i32> = HashMap::new();
    for w in words {
        *need.entry(w.as_bytes()).or_insert(0) += 1;
    }
    for i in 0..wl {
        let mut left = i;
        let mut count = 0;
        let mut window: HashMap<&[u8], i32> = HashMap::new();
        let mut j = i;
        while j + wl <= n {
            let w = &b[j..j + wl];
            if need.contains_key(w) {
                *window.entry(w).or_insert(0) += 1;
                count += 1;
                while window[w] > need[w] {
                    let lw = &b[left..left + wl];
                    *window.get_mut(lw).unwrap() -= 1;
                    count -= 1;
                    left += wl;
                }
                if count == nw {
                    res.push(left);
                    let lw = &b[left..left + wl];
                    *window.get_mut(lw).unwrap() -= 1;
                    count -= 1;
                    left += wl;
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
    let a = find_substring("dogcatcatcodecatdog", &["cat", "dog"]);
    let c = find_substring("barfoobazbitbyte", &["dog", "cat"]);
    let fmt = |v: &[usize]| {
        format!("[{}]", v.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(", "))
    };
    println!("{}", fmt(&a));
    println!("{}", fmt(&c));
}
