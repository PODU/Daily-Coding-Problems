// Staircase ways with step set X: dp[i] = sum dp[i-x] for x in X.
// Time: O(N*|X|), Space: O(N).
fn staircase(n: usize, x: &[usize]) -> u64 {
    let mut dp = vec![0u64; n + 1];
    dp[0] = 1;
    for i in 1..=n {
        for &s in x {
            if i >= s {
                dp[i] += dp[i - s];
            }
        }
    }
    dp[n]
}

fn main() {
    println!("{}", staircase(4, &[1, 2])); // 5
}
