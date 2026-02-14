// Subset sum DP: dp[i][s] = reachable using first i items; reconstruct path. O(n*k) time/space.

fn subset_sum(items: &[i64], k: usize) -> Option<Vec<i64>> {
    let n = items.len();
    let mut dp = vec![vec![false; k + 1]; n + 1];
    dp[0][0] = true;
    for i in 1..=n {
        for s in 0..=k {
            dp[i][s] = dp[i - 1][s];
            if !dp[i][s] && s >= items[i - 1] as usize {
                dp[i][s] = dp[i - 1][s - items[i - 1] as usize];
            }
        }
    }
    if !dp[n][k] {
        return None;
    }
    let mut result = Vec::new();
    let mut s = k;
    for i in (1..=n).rev() {
        if !dp[i - 1][s] {
            result.push(items[i - 1]);
            s -= items[i - 1] as usize;
        }
    }
    result.reverse();
    Some(result)
}

fn main() {
    let items: Vec<i64> = vec![12, 1, 61, 5, 9, 2];
    match subset_sum(&items, 24) {
        Some(res) => {
            let formatted: Vec<String> = res.iter().map(|x| x.to_string()).collect();
            println!("Subset: [{}]", formatted.join(", "));
            let sum: i64 = res.iter().sum();
            println!("Sum: {}", sum);
        }
        None => println!("null"),
    }
    match subset_sum(&items, 1000) {
        Some(_) => println!("found"),
        None => println!("null"),
    }
}
