// Recover coin denominations from change-ways array A (unbounded coin change).
// dp starts {1,0,...}; if A[i] != dp[i], i is a coin -> unbounded-knapsack update. O(N^2) time, O(N) space.

fn recover_coins(a: &[i64]) -> Vec<usize> {
    let n = a.len();
    let mut dp = vec![0i64; n];
    dp[0] = 1;
    let mut coins = Vec::new();
    for i in 1..n {
        if a[i] != dp[i] {
            coins.push(i);
            for v in i..n {
                dp[v] += dp[v - i];
            }
        }
    }
    coins
}

fn format_list(xs: &[usize]) -> String {
    let n = xs.len();
    xs.iter()
        .enumerate()
        .map(|(i, x)| {
            if i == n - 1 && n > 1 {
                format!("and {}", x)
            } else {
                x.to_string()
            }
        })
        .collect::<Vec<_>>()
        .join(", ")
}

fn main() {
    let a = [1, 0, 1, 1, 2];
    println!("{}", format_list(&recover_coins(&a))); // 2, 3, and 4
}
