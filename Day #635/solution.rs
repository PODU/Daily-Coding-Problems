// Day 635: Regular expression matching with '.' and '*' (full-string match).
// Approach: bottom-up DP; dp[i][j] = does s[i:] match p[j:].
// Time: O(m*n), Space: O(m*n).
fn is_match(s: &str, p: &str) -> bool {
    let s: Vec<char> = s.chars().collect();
    let p: Vec<char> = p.chars().collect();
    let m = s.len();
    let n = p.len();
    let mut dp = vec![vec![false; n + 1]; m + 1];
    dp[m][n] = true;
    for i in (0..=m).rev() {
        for j in (0..n).rev() {
            let first = i < m && (p[j] == s[i] || p[j] == '.');
            if j + 1 < n && p[j + 1] == '*' {
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
