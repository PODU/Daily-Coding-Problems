// Day 1774: Fewest steps to reduce N to 1 (decrement by 1, or jump to the larger
// factor of any factorization). DP over 1..N, trying every divisor. O(N*sqrt N) time, O(N) space.
fn min_steps(n: usize) -> usize {
    let mut dp = vec![0usize; n + 1];
    for i in 2..=n {
        dp[i] = dp[i - 1] + 1; // decrement step
        let mut a = 2;
        while a * a <= i {
            if i % a == 0 {
                dp[i] = dp[i].min(dp[i / a] + 1); // jump to larger factor i/a
            }
            a += 1;
        }
    }
    dp[n]
}

fn main() {
    println!("{}", min_steps(100)); // 100->10->9->3->2->1 = 5
}
