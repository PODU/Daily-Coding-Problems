// Day 831: All start indices of substrings that are a concatenation of every word once.
// Sliding window per offset 0..L-1 with hashmap word counts. O(n * L) ~ O(n) total.
use std::collections::HashMap;

fn find_substring(s: &str, words: &[&str]) -> Vec<usize> {
    let mut res = Vec::new();
    if words.is_empty() || s.is_empty() {
        return res;
    }
    let bytes = s.as_bytes();
    let l = words[0].len();
    let m = words.len();
    let n = bytes.len();
    if l * m > n {
        return res;
    }
    let mut need: HashMap<&[u8], i32> = HashMap::new();
    for w in words {
        *need.entry(w.as_bytes()).or_insert(0) += 1;
    }

    for offset in 0..l {
        let mut left = offset;
        let mut count = 0;
        let mut have: HashMap<&[u8], i32> = HashMap::new();
        let mut right = offset;
        while right + l <= n {
            let w = &bytes[right..right + l];
            right += l;
            if need.contains_key(w) {
                *have.entry(w).or_insert(0) += 1;
                count += 1;
                while have[w] > need[w] {
                    let lw = &bytes[left..left + l];
                    *have.get_mut(lw).unwrap() -= 1;
                    left += l;
                    count -= 1;
                }
                if count == m {
                    res.push(left);
                    let lw = &bytes[left..left + l];
                    *have.get_mut(lw).unwrap() -= 1;
                    left += l;
                    count -= 1;
                }
            } else {
                have.clear();
                count = 0;
                left = right;
            }
        }
    }
    res.sort();
    res
}

fn fmt(v: &[usize]) -> String {
    let parts: Vec<String> = v.iter().map(|x| x.to_string()).collect();
    format!("[{}]", parts.join(", "))
}

fn main() {
    println!("{}", fmt(&find_substring("dogcatcatcodecatdog", &["cat", "dog"]))); // [0, 13]
    println!("{}", fmt(&find_substring("barfoobazbitbyte", &["dog", "cat"])));    // []
}
