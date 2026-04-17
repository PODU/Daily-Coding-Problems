// Subset sum returning an actual subset via DP + backtracking reconstruction.
// Time O(n*k), Space O(n*k). Returns None if no subset sums to k.
fn subset_sum(s: &[usize], k: usize) -> Option<Vec<usize>> {
    let n = s.len();
    let mut dp = vec![vec![false; k + 1]; n + 1];
    for i in 0..=n {
        dp[i][0] = true;
    }
    for i in 1..=n {
        for j in 0..=k {
            dp[i][j] = dp[i - 1][j];
            if j >= s[i - 1] && dp[i - 1][j - s[i - 1]] {
                dp[i][j] = true;
            }
        }
    }
    if !dp[n][k] {
        return None;
    }
    let mut res = Vec::new();
    let mut j = k;
    for i in (1..=n).rev() {
        if !dp[i - 1][j] {
            res.push(s[i - 1]);
            j -= s[i - 1];
        }
    }
    res.reverse();
    Some(res)
}

fn main() {
    match subset_sum(&[12, 1, 61, 5, 9, 2], 24) {
        None => println!("null"),
        Some(res) => {
            let parts: Vec<String> = res.iter().map(|v| v.to_string()).collect();
            println!("[{}]", parts.join(", "));
        }
    }
}
