// Day 1025: Remove all consecutive linked-list nodes that sum to zero.
// Approach: prefix-sum over a Vec, tracking last index per prefix sum to skip zero-sum runs. O(N).
use std::collections::HashMap;

fn remove_zero_sum(vals: &[i64]) -> Vec<i64> {
    // prefix[i] = sum of vals[0..i]; prefix[0] = 0 (dummy)
    let n = vals.len();
    let mut prefix = vec![0i64; n + 1];
    for i in 0..n {
        prefix[i + 1] = prefix[i] + vals[i];
    }
    // last[s] = largest index j in 0..=n with prefix[j] == s
    let mut last: HashMap<i64, usize> = HashMap::new();
    for (j, &s) in prefix.iter().enumerate() {
        last.insert(s, j);
    }
    let mut result = Vec::new();
    let mut i = 0usize;
    while i < n {
        let j = last[&prefix[i]]; // skip everything up to last occurrence of this prefix sum
        if j > i {
            i = j; // nodes (i..j) summed to zero, drop them
        } else {
            result.push(vals[i]);
            i += 1;
        }
    }
    result
}

fn main() {
    let out = remove_zero_sum(&[3, 4, -7, 5, -6, 6]);
    let parts: Vec<String> = out.iter().map(|v| v.to_string()).collect();
    println!("{}", parts.join(" -> "));
}
