// Max profit, unlimited transactions with a fixed fee per completed sale.
// DP states cash/hold tracked greedily. Time O(n), Space O(1).
fn max_profit(prices: &[i64], fee: i64) -> i64 {
    let mut cash: i64 = 0;
    let mut hold: i64 = i64::MIN / 4;
    for &p in prices {
        hold = hold.max(cash - p);
        cash = cash.max(hold + p - fee);
    }
    cash
}

fn main() {
    let prices = [1, 3, 2, 8, 4, 10];
    let fee = 2;
    println!("{}", max_profit(&prices, fee));
}
