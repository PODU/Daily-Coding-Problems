// Max sum of non-adjacent numbers: track best-including vs best-excluding current.
// Time: O(n), Space: O(1). (Empty pick allowed, so negatives can be skipped.)
fn max_non_adjacent(nums: &[i64]) -> i64 {
    let (mut incl, mut excl) = (0i64, 0i64);
    for &n in nums {
        let ni = excl + n;
        let ne = incl.max(excl);
        incl = ni;
        excl = ne;
    }
    incl.max(excl)
}

fn main() {
    println!("{}", max_non_adjacent(&[2, 4, 6, 2, 5]));
}
