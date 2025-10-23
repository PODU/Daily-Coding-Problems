// Word break reconstruction via memoized DP: for each suffix, try each prefix
// word and recurse. Time: O(n^2 * L) with memo, Space: O(n^2).
use std::collections::HashMap;
use std::collections::HashSet;

fn solve(
    s: &str,
    start: usize,
    dict: &HashSet<String>,
    memo: &mut HashMap<usize, Option<Vec<String>>>,
) -> Option<Vec<String>> {
    if start == s.len() {
        return Some(Vec::new());
    }
    if let Some(v) = memo.get(&start) {
        return v.clone();
    }
    for end in (start + 1)..=s.len() {
        let word = &s[start..end];
        if dict.contains(word) {
            if let Some(rest) = solve(s, end, dict, memo) {
                let mut res = vec![word.to_string()];
                res.extend(rest);
                memo.insert(start, Some(res.clone()));
                return Some(res);
            }
        }
    }
    memo.insert(start, None);
    None
}

fn reconstruct(words: &[&str], s: &str) -> Option<Vec<String>> {
    let dict: HashSet<String> = words.iter().map(|w| w.to_string()).collect();
    let mut memo = HashMap::new();
    solve(s, 0, &dict, &mut memo)
}

fn fmt(res: Option<Vec<String>>) -> String {
    match res {
        None => "null".to_string(),
        Some(v) => {
            let parts: Vec<String> = v.iter().map(|w| format!("'{}'", w)).collect();
            format!("[{}]", parts.join(", "))
        }
    }
}

fn main() {
    println!("{}", fmt(reconstruct(&["quick", "brown", "the", "fox"], "thequickbrownfox")));
    println!("{}", fmt(reconstruct(&["bed", "bath", "bedbath", "and", "beyond"], "bedbathandbeyond")));
}
