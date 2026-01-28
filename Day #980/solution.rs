// Count attacking bishop pairs by grouping on diagonals (row-col) and anti-diagonals (row+col).
// For each group of size k, add k*(k-1)/2. Time O(N), Space O(N).
use std::collections::HashMap;

fn count_attacking_pairs(bishops: &[(i32, i32)]) -> i64 {
    let mut diag: HashMap<i32, i64> = HashMap::new();
    let mut anti: HashMap<i32, i64> = HashMap::new();
    for &(r, c) in bishops {
        *diag.entry(r - c).or_insert(0) += 1;
        *anti.entry(r + c).or_insert(0) += 1;
    }
    let mut pairs = 0i64;
    for &k in diag.values() {
        pairs += k * (k - 1) / 2;
    }
    for &k in anti.values() {
        pairs += k * (k - 1) / 2;
    }
    pairs
}

fn main() {
    let bishops = [(0, 0), (1, 2), (2, 2), (4, 0)];
    println!("{}", count_attacking_pairs(&bishops)); // 2
}
