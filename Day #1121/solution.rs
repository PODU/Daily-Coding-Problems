// Day 1121 - Max stock profit with a transaction fee, unlimited transactions
// State machine DP: best cash (not holding) and best hold. Time: O(n), Space: O(1).

fn max_profit(prices: &[i32], fee: i32) -> i32 {
    let mut cash = 0;
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
