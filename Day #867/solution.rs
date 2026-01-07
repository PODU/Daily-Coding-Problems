// Day 867: Maximum weight path from top to bottom of a triangle.
// Approach: bottom-up DP, fold each row into the one above using max of adjacent.
// Time: O(n^2), Space: O(n).
fn max_path(t: &[Vec<i32>]) -> i32 {
    let mut dp = t[t.len() - 1].clone();
    for i in (0..t.len() - 1).rev() {
        for j in 0..=i {
            dp[j] = t[i][j] + dp[j].max(dp[j + 1]);
        }
    }
    dp[0]
}

fn main() {
    let t = vec![vec![1], vec![2, 3], vec![1, 5, 1]];
    println!("{}", max_path(&t)); // 9
}
