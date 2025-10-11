// Day 408: Max profit with at most k stock transactions.
// Approach: DP tracking best buy/sell state per transaction in one pass.
// Time: O(n*k), Space: O(k). Example k=2, [5,2,4,0,1] -> 3.

fn max_profit(k: usize, prices: &[i32]) -> i32 {
    let n = prices.len();
    if n == 0 || k == 0 {
        return 0;
    }
    // If k >= n/2, unlimited transactions are possible.
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
    let mut sell = vec![0; k + 1];
    for &price in prices {
        for t in 1..=k {
            buy[t] = buy[t].max(sell[t - 1] - price);
            sell[t] = sell[t].max(buy[t] + price);
        }
    }
    sell[k]
}

fn main() {
    println!("{}", max_profit(2, &[5, 2, 4, 0, 1])); // 3
}
