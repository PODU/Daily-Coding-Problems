// throw_dice: DP over dice, dp[s] = ways to reach sum s. Rolling array.
// Time O(N*total*faces), Space O(total).
fn throw_dice(n: usize, faces: usize, total: usize) -> u64 {
    let mut dp = vec![0u64; total + 1];
    dp[0] = 1;
    for _ in 0..n {
        let mut ndp = vec![0u64; total + 1];
        for s in 0..=total {
            if dp[s] == 0 {
                continue;
            }
            let mut f = 1;
            while f <= faces && s + f <= total {
                ndp[s + f] += dp[s];
                f += 1;
            }
        }
        dp = ndp;
    }
    dp[total]
}

fn main() {
    println!("{}", throw_dice(3, 6, 7));
}
