// Largest consecutive range via hash set: from each start (n-1 absent) extend up. O(n) time, O(n) space.
use std::collections::HashSet;

fn largest_range(nums: &[i32]) -> (i32, i32) {
    let s: HashSet<i32> = nums.iter().cloned().collect();
    let (mut best_lo, mut best_hi, mut best_len) = (nums[0], nums[0], 0);
    for &n in &s {
        if s.contains(&(n - 1)) {
            continue;
        }
        let mut hi = n;
        while s.contains(&(hi + 1)) {
            hi += 1;
        }
        if hi - n + 1 > best_len {
            best_len = hi - n + 1;
            best_lo = n;
            best_hi = hi;
        }
    }
    (best_lo, best_hi)
}

fn main() {
    let nums = [9, 6, 1, 3, 8, 10, 12, 11];
    let (lo, hi) = largest_range(&nums);
    println!("({}, {})", lo, hi);
}
