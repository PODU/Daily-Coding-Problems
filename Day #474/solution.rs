// Min coins via DP over amounts (optimal for arbitrary denominations).
// Greedy also works for canonical US coins {1,5,10,25}. Time: O(n*k), Space: O(n).

fn min_coins(n: usize, coins: &[usize]) -> i64 {
    let inf = i64::MAX;
    let mut dp = vec![inf; n + 1];
    dp[0] = 0;
    for a in 1..=n {
        for &c in coins {
            if c <= a && dp[a - c] != inf {
                dp[a] = dp[a].min(dp[a - c] + 1);
            }
        }
    }
    dp[n]
}

fn main() {
    let coins = [1usize, 5, 10, 25];
    let n = 16;
    println!("{}", min_coins(n, &coins));
}
