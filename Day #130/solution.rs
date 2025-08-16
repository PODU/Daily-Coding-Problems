// Day 130: Max profit with at most k buy/sell transactions.
// DP with hold/cash per transaction. O(n*k) time, O(k) space (greedy when k large).
fn max_profit(k: usize, p: &[i32]) -> i32 {
    let n = p.len();
    if n == 0 || k == 0 {
        return 0;
    }
    if k >= n / 2 {
        let mut prof = 0;
        for i in 1..n {
            if p[i] > p[i - 1] {
                prof += p[i] - p[i - 1];
            }
        }
        return prof;
    }
    let mut buy = vec![i32::MIN; k + 1];
    let mut sell = vec![0i32; k + 1];
    for &price in p {
        for j in 1..=k {
            buy[j] = buy[j].max(sell[j - 1].saturating_sub(price));
            sell[j] = sell[j].max(buy[j] + price);
        }
    }
    sell[k]
}

fn main() {
    let prices = [5, 2, 4, 0, 1];
    println!("{}", max_profit(2, &prices)); // 3
}
