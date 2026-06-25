// DP states cash/hold; fee charged once per buy-sell on sell. Time O(n), Space O(1).
fn max_profit(prices: &[i64], fee: i64) -> i64 {
    if prices.is_empty() {
        return 0;
    }
    let mut cash: i64 = 0;
    let mut hold: i64 = -prices[0];
    for &p in &prices[1..] {
        cash = cash.max(hold + p - fee);
        hold = hold.max(cash - p);
    }
    cash
}

fn main() {
    let prices = [1, 3, 2, 8, 4, 10];
    let fee = 2;
    println!("{}", max_profit(&prices, fee));
}
