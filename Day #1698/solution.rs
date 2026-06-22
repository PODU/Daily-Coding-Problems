// Deduce coin denominations from ways[] via incremental unbounded-knapsack DP.
// Time O(N^2), Space O(N).
fn find_denominations(ways: &[i64]) -> Vec<usize> {
    let n = ways.len();
    let mut dp = vec![0i64; n];
    dp[0] = 1;
    let mut coins = Vec::new();
    for i in 1..n {
        if ways[i] != dp[i] {
            coins.push(i);
            for j in i..n {
                dp[j] += dp[j - i];
            }
        }
    }
    coins
}

fn main() {
    let ways = vec![1, 0, 1, 1, 2];
    let coins = find_denominations(&ways);
    let parts: Vec<String> = coins.iter().map(|c| c.to_string()).collect();
    println!("[{}]", parts.join(", "));
}
