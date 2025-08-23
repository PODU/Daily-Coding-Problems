// Day 157: A permutation is a palindrome iff at most one char has odd count.
// Track parity via a set of odd-count chars. Time O(n), Space O(alphabet).
use std::collections::HashSet;

fn can_form_palindrome(s: &str) -> bool {
    let mut odd: HashSet<char> = HashSet::new();
    for c in s.chars() {
        if !odd.remove(&c) {
            odd.insert(c);
        }
    }
    odd.len() <= 1
}

fn main() {
    println!("{}", can_form_palindrome("carrace")); // true
    println!("{}", can_form_palindrome("daily"));   // false
}
