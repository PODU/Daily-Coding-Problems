// Day 1022: Single number where all others appear 3 times.
// Approach: ones/twos bitmask accumulators. Time O(N), Space O(1).
fn single_number(nums: &[i32]) -> i32 {
    let mut ones = 0;
    let mut twos = 0;
    for &x in nums {
        ones = (ones ^ x) & !twos;
        twos = (twos ^ x) & !ones;
    }
    ones
}

fn main() {
    let tests: [&[i32]; 2] = [&[6, 1, 3, 3, 3, 6, 6], &[13, 19, 13, 13]];
    for t in tests {
        println!("{}", single_number(t));
    }
}
