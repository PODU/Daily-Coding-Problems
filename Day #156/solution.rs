// Day 156: Min perfect squares summing to n via DP. dp[i] = min over squares
// j*j<=i of dp[i-j*j]+1. Time O(n*sqrt(n)), Space O(n).

fn num_squares(n: usize) -> usize {
    let mut dp = vec![usize::MAX; n + 1];
    dp[0] = 0;
    for i in 1..=n {
        let mut j = 1;
        while j * j <= i {
            dp[i] = dp[i].min(dp[i - j * j] + 1);
            j += 1;
        }
    }
    dp[n]
}

fn main() {
    println!("{}", num_squares(13)); // 2
    println!("{}", num_squares(27)); // 3
}
