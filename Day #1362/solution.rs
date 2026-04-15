// Longest contiguous subarray with at most two distinct values.
// Sliding window + hashmap of counts, shrink when distinct > 2. Time O(n), Space O(1).
use std::collections::HashMap;

fn longest_two_distinct(a: &[i32]) -> usize {
    let mut cnt: HashMap<i32, i32> = HashMap::new();
    let mut left = 0usize;
    let mut best = 0usize;
    for right in 0..a.len() {
        *cnt.entry(a[right]).or_insert(0) += 1;
        while cnt.len() > 2 {
            let c = cnt.get_mut(&a[left]).unwrap();
            *c -= 1;
            if *c == 0 {
                cnt.remove(&a[left]);
            }
            left += 1;
        }
        if right - left + 1 > best {
            best = right - left + 1;
        }
    }
    best
}

fn main() {
    let a = [2, 1, 2, 3, 3, 1, 3, 5];
    println!("{}", longest_two_distinct(&a));
}
