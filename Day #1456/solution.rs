// Day 1456: First recurring character in a string.
// Approach: scan left-to-right, track seen chars in a HashSet; first char
// already seen is the answer. Time O(n), Space O(1) (fixed alphabet).
use std::collections::HashSet;

fn first_recurring(s: &str) -> Option<char> {
    let mut seen = HashSet::new();
    for c in s.chars() {
        if seen.contains(&c) {
            return Some(c);
        }
        seen.insert(c);
    }
    None
}

fn main() {
    for s in ["acbbac", "abcdef"] {
        match first_recurring(s) {
            Some(c) => println!("\"{}\"", c),
            None => println!("null"),
        }
    }
}
