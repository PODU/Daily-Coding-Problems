// Regex matching with '.' and '*' via DP. Time O(m*n), Space O(m*n).
fn is_match(s: &str, p: &str) -> bool {
    let s: Vec<char> = s.chars().collect();
    let p: Vec<char> = p.chars().collect();
    let (m, n) = (s.len(), p.len());
    let mut dp = vec![vec![false; n + 1]; m + 1];
    dp[0][0] = true;
    for j in 1..=n {
        if p[j - 1] == '*' {
            dp[0][j] = dp[0][j - 2];
        }
    }
    for i in 1..=m {
        for j in 1..=n {
            if p[j - 1] == '*' {
                dp[i][j] = dp[i][j - 2];
                if p[j - 2] == '.' || p[j - 2] == s[i - 1] {
                    dp[i][j] = dp[i][j] || dp[i - 1][j];
                }
            } else if p[j - 1] == '.' || p[j - 1] == s[i - 1] {
                dp[i][j] = dp[i - 1][j - 1];
            }
        }
    }
    dp[m][n]
}

fn main() {
    assert!(is_match("aa", "a*"));
    assert!(is_match("ab", ".*"));
    assert!(!is_match("mississippi", "mis*is*p*."));
    println!("{}", if is_match("ray", "ra.") { "true" } else { "false" });
}
