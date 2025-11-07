// Two-sum existence: one pass with a hash set of seen numbers.
// Time: O(N), Space: O(N).
use std::collections::HashSet;

fn has_pair_with_sum(nums: &[i32], k: i32) -> bool {
    let mut seen: HashSet<i32> = HashSet::new();
    for &x in nums {
        if seen.contains(&(k - x)) {
            return true;
        }
        seen.insert(x);
    }
    false
}

fn main() {
    let nums = [10, 15, 3, 7];
    let k = 17;
    println!("{}", has_pair_with_sum(&nums, k));
}
