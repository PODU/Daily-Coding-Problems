// Day 868: Can any permutation of the string form a palindrome?
// Approach: count chars; palindrome possible iff at most one char has an odd count.
// Time: O(n), Space: O(alphabet).
use std::collections::HashMap;

fn can_form_palindrome(s: &str) -> bool {
    let mut cnt: HashMap<char, i32> = HashMap::new();
    for c in s.chars() {
        *cnt.entry(c).or_insert(0) += 1;
    }
    let odd = cnt.values().filter(|&&v| v % 2 == 1).count();
    odd <= 1
}

fn main() {
    println!("{}", can_form_palindrome("carrace")); // true
    println!("{}", can_form_palindrome("daily"));   // false
}
