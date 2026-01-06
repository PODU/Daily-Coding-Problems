// Day 860: Regex matching with '.' and '*'.
// Approach: bottom-up DP, dp[i][j] = does s[i:] match p[j:].
// Time: O(n*m), Space: O(n*m).
fn is_match(s: &str, p: &str) -> bool {
    let s: Vec<char> = s.chars().collect();
    let p: Vec<char> = p.chars().collect();
    let (n, m) = (s.len(), p.len());
    let mut dp = vec![vec![false; m + 1]; n + 1];
    dp[n][m] = true;
    for i in (0..=n).rev() {
        for j in (0..m).rev() {
            let first = i < n && (p[j] == s[i] || p[j] == '.');
            if j + 1 < m && p[j + 1] == '*' {
                dp[i][j] = dp[i][j + 2] || (first && dp[i + 1][j]);
            } else {
                dp[i][j] = first && dp[i + 1][j + 1];
            }
        }
    }
    dp[0][0]
}

fn main() {
    println!("{}", is_match("ray", "ra."));     // true
    println!("{}", is_match("raymond", "ra.")); // false
    println!("{}", is_match("chat", ".*at"));   // true
    println!("{}", is_match("chats", ".*at"));  // false
}
