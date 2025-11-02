// Permutation-palindrome check: a permutation can be a palindrome iff at most one
// character has an odd frequency. Toggle membership in a set as we scan.
// Time: O(n); Space: O(alphabet).
use std::collections::HashSet;

fn can_permute_palindrome(s: &str) -> bool {
    let mut odd: HashSet<char> = HashSet::new();
    for ch in s.chars() {
        if !odd.insert(ch) {
            odd.remove(&ch);
        }
    }
    odd.len() <= 1
}

fn main() {
    println!("{}", can_permute_palindrome("carrace"));
    println!("{}", can_permute_palindrome("daily"));
}
