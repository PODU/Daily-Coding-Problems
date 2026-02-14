// Sliding window with count map: longest subarray with at most 2 distinct values. O(n) time, O(1) space.
use std::collections::HashMap;

fn longest_at_most_2_distinct(nums: &[i32]) -> usize {
    let mut cnt: HashMap<i32, usize> = HashMap::new();
    let mut left = 0usize;
    let mut best = 0usize;
    for right in 0..nums.len() {
        *cnt.entry(nums[right]).or_insert(0) += 1;
        while cnt.len() > 2 {
            let l = nums[left];
            let e = cnt.get_mut(&l).unwrap();
            *e -= 1;
            if *e == 0 {
                cnt.remove(&l);
            }
            left += 1;
        }
        best = best.max(right - left + 1);
    }
    best
}

fn main() {
    let nums = vec![2, 1, 2, 3, 3, 1, 3, 5];
    println!("{}", longest_at_most_2_distinct(&nums));
}
