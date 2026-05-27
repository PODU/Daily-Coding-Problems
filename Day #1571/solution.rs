// Word break reconstruction via DP with backpointers. O(n^2) time, O(n) space.
use std::collections::HashSet;

fn word_break(s: &str, words: &[&str]) -> Option<Vec<String>> {
    let dict: HashSet<&str> = words.iter().cloned().collect();
    let n = s.len();
    let mut back = vec![-2i32; n + 1]; // -2 unreachable
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
    let mut i = n as i32;
    while i > 0 {
        let p = back[i as usize];
        res.push(s[p as usize..i as usize].to_string());
        i = p;
    }
    res.reverse();
    Some(res)
}

fn main() {
    let words = ["quick", "brown", "the", "fox"];
    let s = "thequickbrownfox";
    let res = word_break(s, &words).unwrap();
    let parts: Vec<String> = res.iter().map(|w| format!("'{}'", w)).collect();
    println!("[{}]", parts.join(", "));
}
