// Bijective char mapping: maintain s1->s2 (consistent) and s2->s1 (injective) maps. Time O(n), Space O(n).
use std::collections::HashMap;

fn bijective(s1: &str, s2: &str) -> bool {
    if s1.len() != s2.len() {
        return false;
    }
    let mut fwd: HashMap<char, char> = HashMap::new();
    let mut bwd: HashMap<char, char> = HashMap::new();
    for (a, b) in s1.chars().zip(s2.chars()) {
        if let Some(&v) = fwd.get(&a) {
            if v != b {
                return false;
            }
        }
        if let Some(&v) = bwd.get(&b) {
            if v != a {
                return false;
            }
        }
        fwd.insert(a, b);
        bwd.insert(b, a);
    }
    true
}

fn main() {
    if bijective("abc", "bcd") {
        println!("true (map a to b, b to c, and c to d)");
    }
    if !bijective("foo", "bar") {
        println!("false (the o cannot map to two characters)");
    }
}
