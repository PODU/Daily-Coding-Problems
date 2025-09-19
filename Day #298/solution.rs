// Longest contiguous subarray with at most 2 distinct values via sliding window + count map.
// Time: O(n), Space: O(1) (at most 3 keys in map).
use std::collections::HashMap;

fn longest_at_most_2(a: &[i32]) -> usize {
    let mut cnt: HashMap<i32, i32> = HashMap::new();
    let mut left = 0usize;
    let mut best = 0usize;
    for right in 0..a.len() {
        *cnt.entry(a[right]).or_insert(0) += 1;
        while cnt.len() > 2 {
            let v = a[left];
            let e = cnt.get_mut(&v).unwrap();
            *e -= 1;
            if *e == 0 {
                cnt.remove(&v);
            }
            left += 1;
        }
        best = best.max(right - left + 1);
    }
    best
}

fn main() {
    let a = [2, 1, 2, 3, 3, 1, 3, 5];
    println!("{}", longest_at_most_2(&a));
}
