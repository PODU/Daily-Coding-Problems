// Day 272: throw_dice(N, faces, total) -> number of ways to reach total.
// 1-D DP over rolling sums. Time O(N*total*faces), Space O(total).
fn throw_dice(n: usize, faces: usize, total: usize) -> u64 {
    let mut dp = vec![0u64; total + 1];
    dp[0] = 1;
    for _ in 0..n {
        let mut ndp = vec![0u64; total + 1];
        for t in 0..=total {
            if dp[t] == 0 {
                continue;
            }
            let mut f = 1;
            while f <= faces && t + f <= total {
                ndp[t + f] += dp[t];
                f += 1;
            }
        }
        dp = ndp;
    }
    dp[total]
}

fn main() {
    println!("{}", throw_dice(3, 6, 7)); // 15
}
