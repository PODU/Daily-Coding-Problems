// Remove consecutive nodes summing to zero: prefix-sum -> last index map over a vector;
// repeated prefix means a zero-sum span to skip. Time O(n), Space O(n).
use std::collections::HashMap;

fn remove_zero_sum(vals: &[i32]) -> Vec<i32> {
    // prefix[i] = sum of vals[0..i]; prefix has len n+1 (index 0 = dummy).
    let n = vals.len();
    let mut prefix = vec![0i32; n + 1];
    for i in 0..n {
        prefix[i + 1] = prefix[i] + vals[i];
    }
    // For each prefix value, record the last index achieving it.
    let mut last: HashMap<i32, usize> = HashMap::new();
    for i in 0..=n {
        last.insert(prefix[i], i);
    }
    // Walk via "next" links built from last map, like splicing the list.
    let mut res = Vec::new();
    let mut i = 0usize;
    while i < n {
        let j = last[&prefix[i]]; // skip zero-sum span up to j
        // node at position i corresponds to value vals[i]; after dummy, the
        // surviving node is the one right after index j.
        if j > i {
            i = j; // span [i..j] sums to zero, drop it
        } else {
            res.push(vals[i]);
            i += 1;
        }
    }
    res
}

fn main() {
    let vals = [3, 4, -7, 5, -6, 6];
    let res = remove_zero_sum(&vals);
    let parts: Vec<String> = res.iter().map(|v| v.to_string()).collect();
    println!("{}", parts.join(" -> "));
}
