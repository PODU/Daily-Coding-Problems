// Palindrome permutation: toggle chars in a set; a permutation is a palindrome
// iff at most one char has an odd count (set size <= 1). Time O(n), Space O(alphabet).
use std::collections::HashSet;

fn can_form_palindrome(s: &str) -> bool {
    let mut odd: HashSet<char> = HashSet::new();
    for c in s.chars() {
        if !odd.insert(c) {
            odd.remove(&c);
        }
    }
    odd.len() <= 1
}

fn main() {
    let s = "carrace";
    println!("{}", if can_form_palindrome(s) { "true" } else { "false" });
}
