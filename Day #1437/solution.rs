// Day 1437: Length of longest strictly increasing subsequence.
// Approach: Patience sorting; maintain tails array, binary search lower_bound.
// Time: O(n log n), Space: O(n).

fn length_of_lis(nums: &[i32]) -> usize {
    let mut tails: Vec<i32> = Vec::new();
    for &x in nums {
        match tails.binary_search(&x) {
            Ok(_) => {} // x already present (strict) -> replace position is same value, skip
            Err(i) => {
                if i == tails.len() {
                    tails.push(x);
                } else {
                    tails[i] = x;
                }
            }
        }
    }
    tails.len()
}

fn main() {
    let nums = [0, 8, 4, 12, 2, 10, 6, 14, 1, 9, 5, 13, 3, 11, 7, 15];
    println!("{}", length_of_lis(&nums)); // 6
}
