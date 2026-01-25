// Day 953: largest sum of non-adjacent numbers (may pick none -> >= 0).
// incl/excl DP. Time O(n), Space O(1).

fn max_non_adjacent(a: &[i64]) -> i64 {
    let (mut incl, mut excl) = (0i64, 0i64);
    for &x in a {
        let ni = excl + x;
        let ne = incl.max(excl);
        incl = ni;
        excl = ne;
    }
    incl.max(excl)
}

fn main() {
    println!("{}", max_non_adjacent(&[2, 4, 6, 2, 5])); // 13
    println!("{}", max_non_adjacent(&[5, 1, 1, 5]));    // 10
}
