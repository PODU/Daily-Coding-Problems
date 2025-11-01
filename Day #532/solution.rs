// Count attacking bishop pairs: group by diagonals r+c and r-c, sum C(size,2).
// Time O(N), Space O(N).
use std::collections::HashMap;

fn count_attacking_pairs(bishops: &[(i32, i32)]) -> i64 {
    let mut diag1: HashMap<i32, i64> = HashMap::new();
    let mut diag2: HashMap<i32, i64> = HashMap::new();
    for &(r, c) in bishops {
        *diag1.entry(r + c).or_insert(0) += 1;
        *diag2.entry(r - c).or_insert(0) += 1;
    }
    let mut pairs = 0i64;
    for &c in diag1.values() {
        pairs += c * (c - 1) / 2;
    }
    for &c in diag2.values() {
        pairs += c * (c - 1) / 2;
    }
    pairs
}

fn main() {
    let bishops = [(0, 0), (1, 2), (2, 2), (4, 0)];
    println!("{}", count_attacking_pairs(&bishops));
}
