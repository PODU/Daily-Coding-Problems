// Two numbers summing to k via a single-pass hash set.
// Time O(n), Space O(n).
use std::collections::HashSet;

fn has_pair_sum(nums: &[i64], k: i64) -> bool {
    let mut seen = HashSet::new();
    for &x in nums {
        if seen.contains(&(k - x)) {
            return true;
        }
        seen.insert(x);
    }
    false
}

fn main() {
    println!("{}", has_pair_sum(&[10, 15, 3, 7], 17));
}
