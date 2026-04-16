// Can form palindrome by deleting <= k chars: min deletions = n - LPS(s).
// LPS via DP. Time O(n^2), Space O(n^2).
fn lps(s: &[u8]) -> usize {
    let n = s.len();
    let mut dp = vec![vec![0usize; n]; n];
    for i in 0..n {
        dp[i][i] = 1;
    }
    for len in 2..=n {
        for i in 0..=n - len {
            let j = i + len - 1;
            if s[i] == s[j] {
                dp[i][j] = if len == 2 { 2 } else { dp[i + 1][j - 1] + 2 };
            } else {
                dp[i][j] = dp[i + 1][j].max(dp[i][j - 1]);
            }
        }
    }
    dp[0][n - 1]
}

fn can_make_palindrome(s: &str, k: usize) -> bool {
    s.len() - lps(s.as_bytes()) <= k
}

fn main() {
    println!("{}", if can_make_palindrome("waterrfetawx", 2) { "True" } else { "False" });
}
