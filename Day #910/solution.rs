// Minimum jumps to reach end: greedy BFS-by-levels tracking current reach and farthest reach.
// Bump jumps when current window ends. Time O(n), Space O(1).

fn min_jumps(nums: &[i32]) -> i32 {
    let n = nums.len();
    if n <= 1 {
        return 0;
    }
    let (mut jumps, mut cur_end, mut farthest) = (0, 0usize, 0usize);
    for i in 0..n - 1 {
        farthest = farthest.max(i + nums[i] as usize);
        if i == cur_end {
            jumps += 1;
            cur_end = farthest;
        }
    }
    jumps
}

fn main() {
    let nums = [6, 2, 4, 0, 5, 1, 1, 4, 2, 9];
    println!("{}", min_jumps(&nums));
}
