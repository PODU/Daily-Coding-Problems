// Count ways to roll N dice (faces each) summing to total via rolling 1D DP.
// Time O(N*total*faces), Space O(total).
fn throw_dice(n: usize, faces: usize, total: usize) -> i64 {
    let mut dp = vec![0i64; total + 1];
    dp[0] = 1;
    for _ in 0..n {
        let mut ndp = vec![0i64; total + 1];
        for s in 0..=total {
            if dp[s] == 0 {
                continue;
            }
            for f in 1..=faces {
                if s + f <= total {
                    ndp[s + f] += dp[s];
                }
            }
        }
        dp = ndp;
    }
    dp[total]
}

fn main() {
    println!("{}", throw_dice(3, 6, 7));
}
