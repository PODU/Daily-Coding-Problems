// Min coins via bottom-up DP. Returns None if unreachable.
// Time: O(amount * |coins|), Space: O(amount).

fn min_coins(coins: &[u32], amount: usize) -> Option<u32> {
    let inf = u32::MAX;
    let mut dp = vec![inf; amount + 1];
    dp[0] = 0;
    for a in 1..=amount {
        for &c in coins {
            let c = c as usize;
            if c <= a && dp[a - c] != inf {
                dp[a] = dp[a].min(dp[a - c] + 1);
            }
        }
    }
    if dp[amount] == inf { None } else { Some(dp[amount]) }
}

fn show(coins: &[u32], amount: usize) {
    match min_coins(coins, amount) {
        Some(r) => println!("{}", r),
        None => println!("null"),
    }
}

fn main() {
    show(&[1, 5, 10], 56);
    show(&[5, 8], 15);
}
