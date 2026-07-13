// Day 1804: Find a duplicate in array of n+1 elements from {1..n} using a seen vector.
// O(n) time, O(n) space.
fn find_duplicate(nums: &[usize]) -> i64 {
    let mut seen = vec![false; nums.len() + 1];
    for &x in nums {
        if seen[x] {
            return x as i64;
        }
        seen[x] = true;
    }
    -1 // no duplicate (won't happen per problem constraints)
}

fn main() {
    let nums = [1usize, 3, 4, 2, 2];
    println!("{}", find_duplicate(&nums)); // expected 2
}
