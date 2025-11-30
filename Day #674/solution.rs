// Day 674: Longest contiguous subarray with at most 2 distinct values. Sliding window.
// Time O(n), Space O(1) (at most 3 keys in the map).
use std::collections::HashMap;

fn longest_two_types(a: &[i32]) -> usize {
    let mut cnt: HashMap<i32, i32> = HashMap::new();
    let (mut best, mut l) = (0usize, 0usize);
    for r in 0..a.len() {
        *cnt.entry(a[r]).or_insert(0) += 1;
        while cnt.len() > 2 {
            let e = cnt.get_mut(&a[l]).unwrap();
            *e -= 1;
            if *e == 0 {
                cnt.remove(&a[l]);
            }
            l += 1;
        }
        best = best.max(r - l + 1);
    }
    best
}

fn main() {
    println!("{}", longest_two_types(&[2, 1, 2, 3, 3, 1, 3, 5])); // 4
}
