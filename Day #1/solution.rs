// Two-sum existence: one pass with a hash set of complements.
// Time: O(n), Space: O(n).
use std::collections::HashSet;

fn two_sum(nums: &[i32], k: i32) -> bool {
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
    println!("{}", two_sum(&[10, 15, 3, 7], 17));
}
