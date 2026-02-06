// Day 1027: Longest consecutive elements sequence.
// Hash set; only count runs starting at numbers with no predecessor. Time O(n), Space O(n).
use std::collections::HashSet;

fn longest_consecutive(nums: &[i32]) -> i32 {
    let s: HashSet<i32> = nums.iter().copied().collect();
    let mut best = 0;
    for &x in &s {
        if !s.contains(&(x - 1)) {
            let mut len = 1;
            let mut cur = x;
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
