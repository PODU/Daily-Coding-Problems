// Day 1455: Maximum top-to-bottom path sum in a triangle. Bottom-up DP folding
// each row into the one above. Time O(n^2), Space O(n).
fn max_path_sum(triangle: &[Vec<i32>]) -> i32 {
    if triangle.is_empty() {
        return 0;
    }
    let n = triangle.len();
    let mut dp = triangle[n - 1].clone();
    for r in (0..n - 1).rev() {
        for i in 0..=r {
            dp[i] = triangle[r][i] + dp[i].max(dp[i + 1]);
        }
    }
    dp[0]
}

fn main() {
    let triangle = vec![vec![1], vec![2, 3], vec![1, 5, 1]];
    println!("{}", max_path_sum(&triangle)); // 9  (1 -> 3 -> 5)
}
