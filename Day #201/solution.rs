// Day 201: Maximum weight path in a triangle.
// Bottom-up DP: each cell becomes its value + max of the two children below.
// Time: O(n^2), Space: O(n).
fn max_path(t: &[Vec<i32>]) -> i32 {
    let mut dp = t[t.len() - 1].clone();
    for r in (0..t.len() - 1).rev() {
        for c in 0..=r {
            dp[c] = t[r][c] + dp[c].max(dp[c + 1]);
        }
    }
    dp[0]
}

fn main() {
    let t = vec![vec![1], vec![2, 3], vec![1, 5, 1]];
    println!("{}", max_path(&t)); // 9
}
