// Palindrome permutation: count char parities; valid iff <=1 char has odd count. O(n) time, O(1) space.
use std::collections::HashMap;

fn can_permute_palindrome(s: &str) -> bool {
    let mut cnt: HashMap<char, u32> = HashMap::new();
    for c in s.chars() {
        *cnt.entry(c).or_insert(0) += 1;
    }
    cnt.values().filter(|&&v| v & 1 == 1).count() <= 1
}

fn main() {
    println!("{}", can_permute_palindrome("carrace"));
    println!("{}", can_permute_palindrome("daily"));
}
