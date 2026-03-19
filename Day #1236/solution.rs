// Count ways to roll N dice (faces each) summing to total via DP.
// Time O(N*faces*total), Space O(total).
fn throw_dice(n: usize, faces: usize, total: usize) -> u64 {
    let mut dp = vec![0u64; total + 1];
    dp[0] = 1;
    for _ in 0..n {
        let mut nxt = vec![0u64; total + 1];
        for s in 0..=total {
            if dp[s] != 0 {
                let mut f = 1;
                while f <= faces && s + f <= total {
                    nxt[s + f] += dp[s];
                    f += 1;
                }
            }
        }
        dp = nxt;
    }
    dp[total]
}

fn main() {
    println!("{}", throw_dice(3, 6, 7));
}
