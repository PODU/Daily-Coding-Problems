// Largest sum of non-adjacent numbers via include/exclude DP; empty selection (0) allowed.
// Time: O(N), Space: O(1).
fn max_non_adjacent(a: &[i64]) -> i64 {
    let mut incl: i64 = 0;
    let mut excl: i64 = 0;
    for &n in a {
        let ni = excl + n;
        let ne = incl.max(excl);
        incl = ni;
        excl = ne;
    }
    incl.max(excl)
}

fn main() {
    println!("{}", max_non_adjacent(&[2, 4, 6, 2, 5]));
    println!("{}", max_non_adjacent(&[5, 1, 1, 5]));
}
