// Day 159: First recurring character. Scan left to right tracking seen chars in
// a set; return the first already seen. Time O(n), Space O(alphabet).
use std::collections::HashSet;

fn first_recurring(s: &str) -> Option<char> {
    let mut seen = HashSet::new();
    for c in s.chars() {
        if !seen.insert(c) {
            return Some(c);
        }
    }
    None
}

fn main() {
    let show = |s: &str| match first_recurring(s) {
        Some(c) => c.to_string(),
        None => "null".to_string(),
    };
    println!("{}", show("acbbac")); // b
    println!("{}", show("abcdef")); // null
}
