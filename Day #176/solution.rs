// Bijective char mapping check: track s1->s2 map and set of used s2 chars; reject conflicts.
// Time O(n), Space O(unique chars).
use std::collections::{HashMap, HashSet};

fn is_bijective(s1: &str, s2: &str) -> bool {
    if s1.len() != s2.len() {
        return false;
    }
    let mut mapping: HashMap<char, char> = HashMap::new();
    let mut used: HashSet<char> = HashSet::new();
    for (a, b) in s1.chars().zip(s2.chars()) {
        if let Some(&v) = mapping.get(&a) {
            if v != b {
                return false;
            }
        } else {
            if used.contains(&b) {
                return false;
            }
            mapping.insert(a, b);
            used.insert(b);
        }
    }
    true
}

fn main() {
    println!("{}", is_bijective("abc", "bcd"));
    println!("{}", is_bijective("foo", "bar"));
}
