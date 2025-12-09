// Day 722: Minimum swaps so each couple sits side by side.
// Approach: Greedy - fix each even seat; partner = x^1. Each swap places one couple.
// Answer equals N - (#cycles). Time: O(N), Space: O(N).
use std::collections::HashMap;

fn min_swaps(row: &[i32]) -> i32 {
    let mut r = row.to_vec();
    let mut pos: HashMap<i32, usize> = HashMap::new();
    for (i, &v) in r.iter().enumerate() { pos.insert(v, i); }
    let mut swaps = 0;
    let mut i = 0;
    while i < r.len() {
        let partner = r[i] ^ 1;
        if r[i + 1] != partner {
            let j = pos[&partner];
            pos.insert(r[i + 1], j);
            pos.insert(partner, i + 1);
            r.swap(i + 1, j);
            swaps += 1;
        }
        i += 2;
    }
    swaps
}

fn main() {
    println!("Minimum swaps: {}", min_swaps(&[0, 2, 1, 3])); // 1
}
