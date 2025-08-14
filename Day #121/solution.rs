// Day 121: Can form palindrome by deleting at most k chars.
// Min deletions = n - LongestPalindromicSubsequence. DP O(n^2) time, O(n^2) space.
fn lps(s: &str) -> usize {
    let b = s.as_bytes();
    let n = b.len();
    if n == 0 {
        return 0;
    }
    let mut dp = vec![vec![0usize; n]; n];
    for i in 0..n {
        dp[i][i] = 1;
    }
    for len in 2..=n {
        for i in 0..=n - len {
            let j = i + len - 1;
            if b[i] == b[j] {
                dp[i][j] = 2 + if len > 2 { dp[i + 1][j - 1] } else { 0 };
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
    if can_make_palindrome(s, k) {
        println!("You could delete f and x to get 'waterretaw'.");
    } else {
        println!("Not possible");
    }
}
