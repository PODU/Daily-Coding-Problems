// Day 993: Majority element (the value occurring more than floor(n/2) times).
// Count occurrences in a hash map and return the most frequent value. This also
// yields the expected answer for the README's (non-strict) example. O(n) time/space.
use std::collections::HashMap;

fn majority(nums: &[i32]) -> i32 {
    let mut freq: HashMap<i32, i32> = HashMap::new();
    let mut best = nums[0];
    let mut best_count = 0;
    for &x in nums {
        let c = freq.entry(x).or_insert(0);
        *c += 1;
        if *c > best_count {
            best_count = *c;
            best = x;
        }
    }
    best
}

fn main() {
    println!("{}", majority(&[1, 2, 1, 1, 3, 4, 0])); // 1
}
