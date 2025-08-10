// Day 99: Longest consecutive sequence. Hash all values; begin counting only at
// sequence starts (n-1 absent) and walk up. O(n) time, O(n) space.
use std::collections::HashSet;

fn longest_consecutive(nums: &[i32]) -> i32 {
    let s: HashSet<i32> = nums.iter().copied().collect();
    let mut best = 0;
    for &n in &s {
        if !s.contains(&(n - 1)) {
            let mut length = 1;
            while s.contains(&(n + length)) {
                length += 1;
            }
            best = best.max(length);
        }
    }
    best
}

fn main() {
    println!("{}", longest_consecutive(&[100, 4, 200, 1, 3, 2])); // 4
}
