// Subset-sum DP with reconstruction. O(n*k) time, O(n*k) space. Result sorted desc.
fn subset_sum(s: &[i32], k: usize) -> Option<Vec<i32>> {
    let n = s.len();
    let mut dp = vec![vec![false; k + 1]; n + 1];
    for i in 0..=n {
        dp[i][0] = true;
    }
    for i in 1..=n {
        let item = s[i - 1] as usize;
        for j in 0..=k {
            dp[i][j] = dp[i - 1][j];
            if j >= item && dp[i - 1][j - item] {
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
            j -= s[i - 1] as usize;
        }
    }
    res.sort_unstable_by(|a, b| b.cmp(a));
    Some(res)
}

fn print_res(r: Option<Vec<i32>>) {
    match r {
        None => println!("null"),
        Some(v) => {
            let parts: Vec<String> = v.iter().map(|x| x.to_string()).collect();
            println!("[{}]", parts.join(", "));
        }
    }
}

fn main() {
    let s = [12, 1, 61, 5, 9, 2];
    print_res(subset_sum(&s, 24));
    print_res(subset_sum(&s, 1000));
}
