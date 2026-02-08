// Shortest palindrome by inserting chars; lexicographically earliest on ties.
// DP dp[i][j]=min insertions; memoized build(i,j) reconstructs string. Time O(n^2), Space O(n^2).
use std::cmp::min;

fn build(i: i32, j: i32, s: &[u8], dp: &Vec<Vec<i32>>, memo: &mut Vec<Vec<Option<String>>>) -> String {
    if i > j {
        return String::new();
    }
    let (ui, uj) = (i as usize, j as usize);
    if i == j {
        return (s[ui] as char).to_string();
    }
    if let Some(v) = &memo[ui][uj] {
        return v.clone();
    }
    let res;
    if s[ui] == s[uj] {
        res = format!("{}{}{}", s[ui] as char, build(i + 1, j - 1, s, dp, memo), s[ui] as char);
    } else {
        let a = format!("{}{}{}", s[ui] as char, build(i + 1, j, s, dp, memo), s[ui] as char);
        let b = format!("{}{}{}", s[uj] as char, build(i, j - 1, s, dp, memo), s[uj] as char);
        if dp[ui + 1][uj] < dp[ui][uj - 1] {
            res = a;
        } else if dp[ui][uj - 1] < dp[ui + 1][uj] {
            res = b;
        } else if a <= b {
            res = a;
        } else {
            res = b;
        }
    }
    memo[ui][uj] = Some(res.clone());
    res
}

fn solve(input: &str) -> String {
    let s = input.as_bytes();
    let n = s.len();
    if n == 0 {
        return String::new();
    }
    let mut dp = vec![vec![0i32; n]; n];
    for len in 2..=n {
        for i in 0..=(n - len) {
            let j = i + len - 1;
            if s[i] == s[j] {
                dp[i][j] = if i + 1 <= j.wrapping_sub(1) && j >= 1 && i + 1 <= j - 1 { dp[i + 1][j - 1] } else { 0 };
            } else {
                dp[i][j] = 1 + min(dp[i + 1][j], dp[i][j - 1]);
            }
        }
    }
    let mut memo: Vec<Vec<Option<String>>> = vec![vec![None; n]; n];
    build(0, (n - 1) as i32, s, &dp, &mut memo)
}

fn main() {
    println!("race -> {}", solve("race"));
    println!("google -> {}", solve("google"));
}
