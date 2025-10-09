// Longest palindromic subsequence via interval DP dp[i][j] over s[i..j].
// Time O(n^2), Space O(n^2).
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
        for i in 0..=(n - len) {
            let j = i + len - 1;
            if b[i] == b[j] {
                dp[i][j] = 2 + if len == 2 { 0 } else { dp[i + 1][j - 1] };
            } else {
                dp[i][j] = dp[i + 1][j].max(dp[i][j - 1]);
            }
        }
    }
    dp[0][n - 1]
}

fn main() {
    println!("{}", lps("MAPTPTMTPA"));
}
