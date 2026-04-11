// Longest consecutive run via hash set: start only at run heads (x-1 absent), walk up. O(n) time, O(n) space.
use std::collections::HashSet;

fn longest_consecutive(nums: &[i32]) -> i32 {
    let set: HashSet<i32> = nums.iter().copied().collect();
    let mut best = 0;
    for &x in &set {
        if set.contains(&(x - 1)) {
            continue; // not a run head
        }
        let mut cur = x;
        let mut len = 1;
        while set.contains(&(cur + 1)) {
            cur += 1;
            len += 1;
        }
        best = best.max(len);
    }
    best
}

fn main() {
    let nums = [100, 4, 200, 1, 3, 2];
    println!("{}", longest_consecutive(&nums));
}
