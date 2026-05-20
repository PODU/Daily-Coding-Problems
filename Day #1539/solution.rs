// Remove all consecutive nodes summing to zero using prefix sums + hash map.
// A prefix sum seen before means the span between is zero-sum and is excised.
// Time O(n), Space O(n). (Implemented over a Vec for Rust ownership simplicity.)
use std::collections::HashMap;

fn remove_zero_sum(vals: &[i64]) -> Vec<i64> {
    // prefix[i] = sum of vals[0..i]; if prefix value repeats, drop the in-between span.
    let n = vals.len();
    let mut prefix = vec![0i64; n + 1];
    for i in 0..n {
        prefix[i + 1] = prefix[i] + vals[i];
    }
    // last index (in prefix array) where each sum occurs
    let mut last: HashMap<i64, usize> = HashMap::new();
    for i in 0..=n {
        last.insert(prefix[i], i);
    }
    let mut res = Vec::new();
    let mut i = 0;
    while i < n {
        // jump to the last occurrence of prefix[i]
        let j = last[&prefix[i]];
        if j > i {
            i = j; // skip zero-sum span vals[i..j]
            continue;
        }
        res.push(vals[i]);
        i += 1;
    }
    res
}

fn main() {
    let out = remove_zero_sum(&[3, 4, -7, 5, -6, 6]);
    let s: Vec<String> = out.iter().map(|v| v.to_string()).collect();
    println!("{}", s.join(" "));
}
