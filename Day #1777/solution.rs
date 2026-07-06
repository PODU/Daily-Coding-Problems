// Count attacking bishop pairs: group by diagonal (r-c) and anti-diagonal (r+c), sum c*(c-1)/2.
// Time: O(N), Space: O(N).
use std::collections::HashMap;

fn main() {
    let _m = 5;
    let bishops = [(0i32, 0i32), (1, 2), (2, 2), (4, 0)];
    let mut diag: HashMap<i32, i64> = HashMap::new();
    let mut anti: HashMap<i32, i64> = HashMap::new();
    for &(r, c) in bishops.iter() {
        *diag.entry(r - c).or_insert(0) += 1;
        *anti.entry(r + c).or_insert(0) += 1;
    }
    let mut pairs: i64 = 0;
    for &c in diag.values() {
        pairs += c * (c - 1) / 2;
    }
    for &c in anti.values() {
        pairs += c * (c - 1) / 2;
    }
    println!("{}", pairs);
}
