// Day 632: Deduce coin denominations from a "ways to make change" array.
// Approach: reverse coin-change DP. If ways[i] exceeds count reachable with
// coins found so far, i is itself a denomination.
// Time: O(N * D), Space: O(N).
fn find_denominations(ways: &[i64]) -> Vec<usize> {
    let n = ways.len();
    let mut dp = vec![0i64; n];
    dp[0] = 1;
    let mut coins = Vec::new();
    for i in 1..n {
        if dp[i] < ways[i] {
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
    let strs: Vec<String> = coins.iter().map(|c| c.to_string()).collect();
    if strs.len() > 1 {
        println!("{}, and {}", strs[..strs.len() - 1].join(", "), strs[strs.len() - 1]);
    } else {
        println!("{}", strs.join(", "));
    }
}
