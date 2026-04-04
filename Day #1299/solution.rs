// Day 1299: Count ordered ways to climb N stairs using step sizes from set X.
// DP: dp[i] = sum(dp[i-x]) for x in X. O(N*|X|) time, O(N) space.
fn climb_ways(n: usize, steps: &[usize]) -> u64 {
    let mut dp = vec![0u64; n + 1];
    dp[0] = 1;
    for i in 1..=n {
        for &x in steps {
            if i >= x {
                dp[i] += dp[i - x];
            }
        }
    }
    dp[n]
}

fn main() {
    println!("{}", climb_ways(4, &[1, 2]));     // 5
    println!("{}", climb_ways(10, &[1, 3, 5])); // generalized
}
