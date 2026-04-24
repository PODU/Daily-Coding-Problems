// Day 1414: does a one-to-one (bijective) char mapping s1 -> s2 exist?
// Approach: enforce a consistent forward map AND injective (no two srcs share a target). O(n).
use std::collections::HashMap;

fn can_map(s1: &str, s2: &str) -> bool {
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
    println!("{}", can_map("abc", "bcd")); // true
    println!("{}", can_map("foo", "bar")); // false
}
