// Day 373: Longest run of consecutive integers formable from the list.
// Hash set; start counting only at run-starts (x with no x-1 present). O(n) time.
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
    println!("{}", longest_consecutive(&[5, 2, 99, 3, 4, 1, 100])); // 5
}
