// Longest Increasing Subsequence (length), patience sorting.
// Maintain tails[]; binary-search insertion point for each value. O(n log n) time, O(n) space.

fn length_of_lis(nums: &[i32]) -> usize {
    let mut tails: Vec<i32> = Vec::new();
    for &x in nums {
        match tails.binary_search(&x) {
            Ok(_) => {} // equal element exists; strictly increasing, skip duplicate
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
    let nums = vec![0, 8, 4, 12, 2, 10, 6, 14, 1, 9, 5, 13, 3, 11, 7, 15];
    println!("{}", length_of_lis(&nums));
}
