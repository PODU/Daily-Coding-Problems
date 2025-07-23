// Word Break reconstruction: DP over positions with memoization using a word set.
// Time: O(n^2) substring checks (n = string length), Space: O(n) for memo + recursion.
use std::collections::HashSet;

fn solve(s: &[u8], dict: &HashSet<String>, i: usize, memo: &mut Vec<i32>) -> bool {
    let n = s.len();
    if i == n {
        return true;
    }
    if memo[i] != -2 {
        return memo[i] != -1;
    }
    for j in (i + 1)..=n {
        let sub = String::from_utf8_lossy(&s[i..j]).to_string();
        if dict.contains(&sub) && solve(s, dict, j, memo) {
            memo[i] = (j - i) as i32;
            return true;
        }
    }
    memo[i] = -1;
    false
}

fn word_break(s: &str, words: &[&str]) -> Option<Vec<String>> {
    let dict: HashSet<String> = words.iter().map(|w| w.to_string()).collect();
    let bytes = s.as_bytes();
    let n = bytes.len();
    let mut memo = vec![-2i32; n + 1];
    if !solve(bytes, &dict, 0, &mut memo) {
        return None;
    }
    let mut res = Vec::new();
    let mut i = 0usize;
    while i < n {
        let len = memo[i] as usize;
        res.push(String::from_utf8_lossy(&bytes[i..i + len]).to_string());
        i += len;
    }
    Some(res)
}

fn main() {
    let words = ["quick", "brown", "the", "fox"];
    let s = "thequickbrownfox";
    match word_break(s, &words) {
        None => println!("None"),
        Some(res) => {
            let parts: Vec<String> = res.iter().map(|w| format!("'{}'", w)).collect();
            println!("[{}]", parts.join(", "));
        }
    }
}
