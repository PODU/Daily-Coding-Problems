// Day 305: Remove consecutive nodes summing to zero. Prefix-sum + hashmap. O(N).
// Implemented on a Vec model (equivalent to a singly linked list) for clarity.
use std::collections::HashMap;

fn remove_zero_sum(vals: &[i64]) -> Vec<i64> {
    // prefix[i] = sum of vals[0..i]; if prefix repeats, the span between cancels to 0.
    let n = vals.len();
    let mut prefix = vec![0i64; n + 1];
    for i in 0..n { prefix[i + 1] = prefix[i] + vals[i]; }
    // last index where each prefix value occurs
    let mut last: HashMap<i64, usize> = HashMap::new();
    for i in 0..=n { last.insert(prefix[i], i); }
    let mut res = Vec::new();
    let mut i = 0;
    while i < n {
        let j = last[&prefix[i]]; // skip span [i, j) which sums to zero
        if j > i { i = j; } else { res.push(vals[i]); i += 1; }
    }
    res
}

fn main() {
    let out = remove_zero_sum(&[3, 4, -7, 5, -6, 6]);
    println!("{}", out.iter().map(|v| v.to_string()).collect::<Vec<_>>().join(" ")); // 5
}
