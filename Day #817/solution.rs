// Word break via DP with backpointers: dp[i] reachable, prev[i] start of last word. O(n^2) time, O(n) space.
use std::collections::HashSet;

fn word_break(s: &str, dict: &HashSet<&str>) -> Option<Vec<String>> {
    let n = s.len();
    let mut prev = vec![-2i32; n + 1]; // -2 = unreachable
    prev[0] = -1;
    for i in 1..=n {
        for j in (0..i).rev() { // prefer shortest last word
            if prev[j] != -2 && dict.contains(&s[j..i]) {
                prev[i] = j as i32;
                break;
            }
        }
    }
    if prev[n] == -2 {
        return None;
    }
    let mut res: Vec<String> = Vec::new();
    let mut i = n as i32;
    while i > 0 {
        let p = prev[i as usize];
        res.insert(0, s[p as usize..i as usize].to_string());
        i = p;
    }
    Some(res)
}

fn fmt(r: Option<Vec<String>>) -> String {
    match r {
        None => "null".to_string(),
        Some(v) => format!("[{}]", v.join(", ")),
    }
}

fn main() {
    let d1: HashSet<&str> = ["quick", "brown", "the", "fox"].iter().cloned().collect();
    let d2: HashSet<&str> = ["bed", "bath", "bedbath", "and", "beyond"].iter().cloned().collect();
    println!("{}", fmt(word_break("thequickbrownfox", &d1)));
    println!("{}", fmt(word_break("bedbathandbeyond", &d2)));
}
