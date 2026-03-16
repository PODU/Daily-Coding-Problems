// Day 1215: Min steps to reduce N to 1 (decrement, or replace by larger factor).
// DP: dp[i] = 1 + min(dp[i-1], dp[i/d] for divisors d). Time O(N sqrt N), Space O(N).
fn min_steps(n: usize) -> usize {
    let mut dp = vec![0usize; n + 1];
    for i in 2..=n {
        dp[i] = dp[i - 1] + 1;
        let mut d = 2;
        while d * d <= i {
            if i % d == 0 {
                dp[i] = dp[i].min(dp[i / d] + 1);
            }
            d += 1;
        }
    }
    dp[n]
}

fn main() {
    println!("{}", min_steps(100)); // 5
}
