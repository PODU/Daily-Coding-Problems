// Day 1636: Can a string be made a palindrome by deleting at most k chars.
// min deletions = n - LongestPalindromicSubsequence; DP O(n^2) time/space.
fn can_make_palindrome(s: &str, k: usize) -> bool {
    let b = s.as_bytes();
    let n = b.len();
    if n == 0 {
        return true;
    }
    let mut dp = vec![vec![0usize; n]; n];
    for i in 0..n {
        dp[i][i] = 1;
    }
    for len in 2..=n {
        for i in 0..=(n - len) {
            let j = i + len - 1;
            if b[i] == b[j] {
                dp[i][j] = 2 + if len > 2 { dp[i + 1][j - 1] } else { 0 };
            } else {
                dp[i][j] = dp[i + 1][j].max(dp[i][j - 1]);
            }
        }
    }
    let lps = dp[0][n - 1];
    (n - lps) <= k
}

fn main() {
    println!("{}", if can_make_palindrome("waterrfetawx", 2) { "True" } else { "False" });
}
