// Min-insertion palindrome, lexicographically earliest. DP over substrings; O(n^2) states, O(n^3) overall.
fn solve(s: &str) -> String {
    let s: Vec<char> = s.chars().collect();
    let n = s.len();
    if n == 0 {
        return String::new();
    }
    let mut dp = vec![vec![String::new(); n]; n];
    for i in 0..n {
        dp[i][i] = s[i].to_string();
    }
    for len in 2..=n {
        for i in 0..=n - len {
            let j = i + len - 1;
            if s[i] == s[j] {
                let inner = if i + 1 <= j - 1 {
                    dp[i + 1][j - 1].clone()
                } else {
                    String::new()
                };
                dp[i][j] = format!("{}{}{}", s[i], inner, s[j]);
            } else {
                let c1 = format!("{}{}{}", s[i], dp[i + 1][j], s[i]);
                let c2 = format!("{}{}{}", s[j], dp[i][j - 1], s[j]);
                dp[i][j] = if c1.len() < c2.len() {
                    c1
                } else if c2.len() < c1.len() {
                    c2
                } else if c1 <= c2 {
                    c1
                } else {
                    c2
                };
            }
        }
    }
    dp[0][n - 1].clone()
}

fn main() {
    println!("\"{}\"", solve("race"));
}
