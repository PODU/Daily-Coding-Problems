// Longest consecutive sequence: hash set, count runs from each sequence start. O(n) time, O(n) space.
use std::collections::HashSet;

fn longest_consecutive(nums: &[i32]) -> i32 {
    let s: HashSet<i32> = nums.iter().cloned().collect();
    let mut best = 0;
    for &n in &s {
        if !s.contains(&(n - 1)) {
            let mut cur = n;
            let mut len = 1;
            while s.contains(&(cur + 1)) {
                cur += 1;
                len += 1;
            }
            best = best.max(len);
        }
    }
    best
}

fn main() {
    println!("{}", longest_consecutive(&[100, 4, 200, 1, 3, 2]));
}
