// First recurring character: single pass with a hash set, return first char already seen.
// Time O(n), Space O(k).
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

fn run(s: &str) {
    match first_recurring(s) {
        Some(c) => println!("{}", c),
        None => println!("null"),
    }
}

fn main() {
    run("acbbac");
    run("abcdef");
}
