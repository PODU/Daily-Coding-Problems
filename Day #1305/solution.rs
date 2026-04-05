// Day 1305: Longest common subsequence of three strings.
// 3D DP over prefixes. O(|a||b||c|) time, O(|a||b||c|) space.
fn lcs3(a: &str, b: &str, c: &str) -> usize {
    let (a, b, c) = (a.as_bytes(), b.as_bytes(), c.as_bytes());
    let (n, m, p) = (a.len(), b.len(), c.len());
    let mut dp = vec![vec![vec![0usize; p + 1]; m + 1]; n + 1];
    for i in 1..=n {
        for j in 1..=m {
            for k in 1..=p {
                if a[i - 1] == b[j - 1] && b[j - 1] == c[k - 1] {
                    dp[i][j][k] = dp[i - 1][j - 1][k - 1] + 1;
                } else {
                    dp[i][j][k] = dp[i - 1][j][k]
                        .max(dp[i][j - 1][k])
                        .max(dp[i][j][k - 1]);
                }
            }
        }
    }
    dp[n][m][p]
}

fn main() {
    println!(
        "{}",
        lcs3("epidemiologist", "refrigeration", "supercalifragilisticexpialodocious")
    ); // 5
}
