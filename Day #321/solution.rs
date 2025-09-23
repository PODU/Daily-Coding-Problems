// Min steps to reduce N to 1 (N-1 or replace with max factor): DP.
// Time O(N*sqrt(N)), Space O(N).
fn min_steps(n: usize) -> usize {
    let mut dp = vec![0usize; n + 1];
    for m in 2..=n {
        dp[m] = dp[m - 1] + 1;
        let mut a = 2;
        while a * a <= m {
            if m % a == 0 {
                let b = m / a; // b >= a, max(a,b)=b
                if dp[b] + 1 < dp[m] {
                    dp[m] = dp[b] + 1;
                }
            }
            a += 1;
        }
    }
    dp[n]
}

fn main() {
    println!("{}", min_steps(100));
}
