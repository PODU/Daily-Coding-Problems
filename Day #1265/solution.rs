// Day 1265: Reconstruct a sentence from a space-free string and a dictionary.
// DP over prefixes storing one valid breakpoint. O(n^2) time (avg), O(n) space.
use std::collections::HashSet;

fn word_break(s: &str, words: &[&str]) -> Option<Vec<String>> {
    let dict: HashSet<&str> = words.iter().copied().collect();
    let n = s.len();
    let mut back = vec![-2i32; n + 1];
    back[0] = -1;
    for i in 1..=n {
        for j in 0..i {
            if back[j] != -2 && dict.contains(&s[j..i]) {
                back[i] = j as i32;
                break;
            }
        }
    }
    if back[n] == -2 {
        return None;
    }
    let mut res = Vec::new();
    let mut i = n;
    while i > 0 {
        let j = back[i] as usize;
        res.push(s[j..i].to_string());
        i = j;
    }
    res.reverse();
    Some(res)
}

fn main() {
    match word_break("thequickbrownfox", &["quick", "brown", "the", "fox"]) {
        Some(v) => println!("{:?}", v),
        None => println!("null"),
    }
}
