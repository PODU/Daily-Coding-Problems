// Day 1422: largest sum of non-adjacent numbers (values may be 0/negative).
// Approach: rolling include/exclude DP; empty subset allowed (floor 0). O(n) time, O(1) space.

fn max_non_adjacent(nums: &[i64]) -> i64 {
    let mut incl = 0i64;
    let mut excl = 0i64;
    for &n in nums {
        let new_incl = excl + n;
        let new_excl = incl.max(excl);
        incl = new_incl;
        excl = new_excl;
    }
    incl.max(excl)
}

fn main() {
    println!("{}", max_non_adjacent(&[2, 4, 6, 2, 5])); // 13
    println!("{}", max_non_adjacent(&[5, 1, 1, 5]));    // 10
}
