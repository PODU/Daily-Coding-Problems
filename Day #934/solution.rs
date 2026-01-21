// Day 934: First recurring character = first char that has been seen before while scanning.
// Hash set of seen chars; return on first repeat. Time O(n), Space O(min(n, alphabet)).
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
    // "b"
    // null
}
