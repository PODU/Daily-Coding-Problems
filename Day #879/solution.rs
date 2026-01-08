// Two numbers summing to k via one-pass hash set. Time O(n), Space O(n).
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
    let nums = vec![10, 15, 3, 7];
    println!("{}", two_sum(&nums, 17));
}
