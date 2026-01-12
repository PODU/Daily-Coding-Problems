// Min steps to 1: dp[i] = 1 + min(dp[i-1], dp[i/a] over factors a). Time O(N*sqrt N), Space O(N).

fn min_steps(n: usize) -> usize {
    let mut dp = vec![0usize; n + 1];
    for i in 2..=n {
        dp[i] = dp[i - 1] + 1;
        let mut a = 2;
        while a * a <= i {
            if i % a == 0 {
                let larger = i / a;
                dp[i] = dp[i].min(1 + dp[larger]);
            }
            a += 1;
        }
    }
    dp[n]
}

fn main() {
    println!("{}", min_steps(100));
}
