// Day 1423: can you reach the end of the array (each value = max steps forward)?
// Approach: greedy, track farthest reachable index. O(n) time, O(1) space.

fn can_reach_end(nums: &[usize]) -> bool {
    let mut farthest = 0usize;
    for (i, &step) in nums.iter().enumerate() {
        if i > farthest {
            return false;
        }
        farthest = farthest.max(i + step);
    }
    true
}

fn main() {
    println!("{}", can_reach_end(&[1, 3, 1, 2, 0, 1])); // true
    println!("{}", can_reach_end(&[1, 2, 1, 0, 0]));    // false
}
