// Subset Sum: boolean DP dp[i][j] = can make j with first i items, then backtrack.
// Found subset is sorted descending for a deterministic [12, 9, 2, 1] output.
// Time O(n*k), Space O(n*k).

fn subset_sum(s: &[i32], k: usize) -> Option<Vec<i32>> {
    let n = s.len();
    let mut dp = vec![vec![false; k + 1]; n + 1];
    for i in 0..=n {
        dp[i][0] = true;
    }
    for i in 1..=n {
        let v = s[i - 1] as usize;
        for j in 1..=k {
            dp[i][j] = dp[i - 1][j];
            if j >= v && dp[i - 1][j - v] {
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
        if dp[i - 1][j] {
            continue; // item i-1 not needed
        }
        res.push(s[i - 1]); // item i-1 must be included
        j -= s[i - 1] as usize;
    }
    res.sort_by(|a, b| b.cmp(a));
    Some(res)
}

fn fmt(v: &Option<Vec<i32>>) -> String {
    match v {
        None => "null".to_string(),
        Some(list) => {
            let parts: Vec<String> = list.iter().map(|x| x.to_string()).collect();
            format!("[{}]", parts.join(", "))
        }
    }
}

fn main() {
    let s = [12, 1, 61, 5, 9, 2];
    println!("{}", fmt(&subset_sum(&s, 24)));
}
