// Two-sum existence: one-pass hash set, check (k-num) seen before insert.
// Time O(n), Space O(n).
use std::collections::HashSet;

fn has_pair(a: &[i64], k: i64) -> bool {
    let mut seen = HashSet::new();
    for &x in a {
        if seen.contains(&(k - x)) {
            return true;
        }
        seen.insert(x);
    }
    false
}

fn main() {
    let v = [10i64, 15, 3, 7];
    println!("{}", has_pair(&v, 17));
    println!("{}", has_pair(&v, 50));
}
