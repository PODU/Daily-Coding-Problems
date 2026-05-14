// First recurring character: scan L->R, track seen set; first char already seen wins.
// O(n) time, O(alphabet) space.
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
    match first_recurring("acbbac") {
        Some(c) => println!("{}", c),
        None => println!("null"),
    }
}
