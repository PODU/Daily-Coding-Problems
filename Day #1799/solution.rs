// Longest contiguous subarray with at most 2 distinct values via sliding window + hashmap. O(n) time, O(1) space.
use std::collections::HashMap;

fn longest_two_distinct(a: &[i32]) -> usize {
    let mut cnt: HashMap<i32, i32> = HashMap::new();
    let mut left = 0;
    let mut best = 0;
    for right in 0..a.len() {
        *cnt.entry(a[right]).or_insert(0) += 1;
        while cnt.len() > 2 {
            let e = cnt.get_mut(&a[left]).unwrap();
            *e -= 1;
            if *e == 0 {
                cnt.remove(&a[left]);
            }
            left += 1;
        }
        best = best.max(right - left + 1);
    }
    best
}

fn main() {
    let a = [2, 1, 2, 3, 3, 1, 3, 5];
    println!("{}", longest_two_distinct(&a)); // 4
}
