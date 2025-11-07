// Day 564: Largest sum of non-adjacent numbers.
// DP tracking incl/excl running maxes. Time O(n), Space O(1).
fn largest_non_adjacent(nums: &[i64]) -> i64 {
    let (mut incl, mut excl) = (0i64, 0i64); // including / excluding previous element
    for &x in nums {
        let new_incl = excl + x;
        let new_excl = incl.max(excl);
        incl = new_incl;
        excl = new_excl;
    }
    incl.max(excl)
}

fn main() {
    println!("{}", largest_non_adjacent(&[2, 4, 6, 2, 5]));
    println!("{}", largest_non_adjacent(&[5, 1, 1, 5]));
}
