// Day 415: Max stock profit, unlimited transactions with a per-transaction fee.
// DP two states: cash (no stock) and hold (holding). Time O(N), Space O(1).
fn max_profit(prices: &[i64], fee: i64) -> i64 {
    if prices.is_empty() {
        return 0;
    }
    let mut cash = 0i64;
    let mut hold = -prices[0];
    for &p in &prices[1..] {
        cash = cash.max(hold + p - fee);
        hold = hold.max(cash - p);
    }
    cash
}

fn main() {
    println!("{}", max_profit(&[1, 3, 2, 8, 4, 10], 2)); // 9
}
