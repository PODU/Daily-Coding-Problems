// Day 1749: Max profit with at most k transactions.
// DP with buy/sell states; if k>=n/2 it's unlimited (sum positive diffs).
// Time O(n*k) (or O(n) when unlimited), Space O(k).

fn max_profit(k: usize, prices: &[i32]) -> i32 {
    let n = prices.len();
    if n < 2 || k == 0 {
        return 0;
    }
    if k >= n / 2 {
        let mut profit = 0;
        for i in 1..n {
            if prices[i] > prices[i - 1] {
                profit += prices[i] - prices[i - 1];
            }
        }
        return profit;
    }
    let mut buy = vec![i32::MIN; k + 1];
    let mut sell = vec![0i32; k + 1];
    for &price in prices {
        for j in 1..=k {
            buy[j] = buy[j].max(sell[j - 1] - price);
            sell[j] = sell[j].max(buy[j] + price);
        }
    }
    sell[k]
}

fn main() {
    println!("{}", max_profit(2, &[5, 2, 4, 0, 1])); // 3
}
