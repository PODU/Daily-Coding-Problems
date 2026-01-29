// Longest Increasing Subsequence via patience sorting: maintain a "tails" vector and
// binary-search the insertion point for each element. Time O(n log n), Space O(n).
fn length_of_lis(nums: &[i32]) -> usize {
    let mut tails: Vec<i32> = Vec::new(); // tails[i] = smallest tail of length i+1
    for &x in nums {
        match tails.binary_search(&x) {
            Ok(_) => {} // x already present (strictly increasing: skip duplicate)
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
    let nums = [10, 9, 2, 5, 3, 7, 101, 18];
    println!("Input: {:?}", nums);
    println!("LIS length: {}", length_of_lis(&nums)); // 4
}
