// Day 411: Can we make s a palindrome by deleting at most k chars?
// Answer: n - LPS(s) <= k, where LPS = longest palindromic subsequence via DP. O(n^2) time, O(n^2) space.
fn lps(s: &str) -> usize {
    let b = s.as_bytes();
    let n = b.len();
    let mut dp = vec![vec![0usize; n]; n];
    for i in (0..n).rev() {
        dp[i][i] = 1;
        for j in (i + 1)..n {
            if b[i] == b[j] {
                dp[i][j] = dp[i + 1][j - 1] + 2;
            } else {
                dp[i][j] = dp[i + 1][j].max(dp[i][j - 1]);
            }
        }
    }
    dp[0][n - 1]
}

fn can_make_palindrome(s: &str, k: usize) -> bool {
    s.len() - lps(s) <= k
}

fn main() {
    let s = "waterrfetawx";
    let k = 2;
    println!("{}", if can_make_palindrome(s, k) { "True" } else { "False" });
}
