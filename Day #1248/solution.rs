// Concatenation substring indices via sliding window over wordLen offsets with hashmap counts. O(n*wordLen) time, O(m) space.
use std::collections::HashMap;

fn find_substring(s: &str, words: &[&str]) -> Vec<usize> {
    let mut res: Vec<usize> = Vec::new();
    if words.is_empty() || s.is_empty() {
        return res;
    }
    let bytes = s.as_bytes();
    let n = bytes.len();
    let word_len = words[0].len();
    let num_words = words.len();
    let window_len = word_len * num_words;
    if n < window_len {
        return res;
    }

    let mut need: HashMap<&str, i32> = HashMap::new();
    for &w in words {
        *need.entry(w).or_insert(0) += 1;
    }

    for offset in 0..word_len {
        let mut window: HashMap<&str, i32> = HashMap::new();
        let mut count = 0i32;
        let mut left = offset;
        let mut right = offset;
        while right + word_len <= n {
            let word = &s[right..right + word_len];
            if need.contains_key(word) {
                *window.entry(word).or_insert(0) += 1;
                count += 1;
                while window[word] > need[word] {
                    let lw = &s[left..left + word_len];
                    *window.get_mut(lw).unwrap() -= 1;
                    count -= 1;
                    left += word_len;
                }
                if count == num_words as i32 {
                    res.push(left);
                    let lw = &s[left..left + word_len];
                    *window.get_mut(lw).unwrap() -= 1;
                    count -= 1;
                    left += word_len;
                }
            } else {
                window.clear();
                count = 0;
                left = right + word_len;
            }
            right += word_len;
        }
    }
    res.sort();
    res
}

fn main() {
    let s = "dogcatcatcodecatdog";
    let words = ["cat", "dog"];
    let res = find_substring(s, &words);
    let parts: Vec<String> = res.iter().map(|x| x.to_string()).collect();
    println!("[{}]", parts.join(", "));
}
