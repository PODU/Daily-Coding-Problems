// Day 771: One-to-one (bijective) character mapping between s1 and s2.
// Track forward and reverse maps; reject conflicts. O(n) time, O(1) space.
use std::collections::HashMap;

fn is_one_to_one(s1: &str, s2: &str) -> bool {
    if s1.len() != s2.len() {
        return false;
    }
    let mut fwd: HashMap<char, char> = HashMap::new();
    let mut rev: HashMap<char, char> = HashMap::new();
    for (a, b) in s1.chars().zip(s2.chars()) {
        if let Some(&v) = fwd.get(&a) {
            if v != b {
                return false;
            }
        }
        if let Some(&v) = rev.get(&b) {
            if v != a {
                return false;
            }
        }
        fwd.insert(a, b);
        rev.insert(b, a);
    }
    true
}

fn main() {
    println!("{}", is_one_to_one("abc", "bcd")); // true
    println!("{}", is_one_to_one("foo", "bar")); // false
}
