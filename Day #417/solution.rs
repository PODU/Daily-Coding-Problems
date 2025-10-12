// Day 417: Remove all consecutive nodes summing to zero via prefix-sum + hashmap.
// Operates on a Vec model (indices as node identity). Time O(n), Space O(n).
use std::collections::HashMap;

fn remove_zero_sum(vals: &[i32]) -> Vec<i32> {
    // prefix[i] = sum of vals[0..i]; prefix has len n+1.
    let n = vals.len();
    let mut prefix = vec![0i32; n + 1];
    for i in 0..n {
        prefix[i + 1] = prefix[i] + vals[i];
    }
    // For each prefix-sum value keep the last index where it occurs; then jump.
    let mut last: HashMap<i32, usize> = HashMap::new();
    for i in 0..=n {
        last.insert(prefix[i], i);
    }
    let mut result = Vec::new();
    let mut i = 0;
    while i < n {
        let next = *last.get(&prefix[i]).unwrap();
        if next > i {
            // values i..next sum to zero -> skip them
            i = next;
        } else {
            result.push(vals[i]);
            i += 1;
        }
    }
    result
}

fn main() {
    let res = remove_zero_sum(&[3, 4, -7, 5, -6, 6]);
    let parts: Vec<String> = res.iter().map(|v| v.to_string()).collect();
    println!("{}", parts.join(" -> "));
}
