// Max profit with at most k transactions. If k>=n/2 -> unlimited (sum positive diffs).
// Else DP with buy/sell rolling arrays. Time O(n*k), Space O(k).

fn max_profit(k: usize, prices: &[i32]) -> i32 {
    let n = prices.len();
    if n == 0 || k == 0 {
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
    for &p in prices {
        for j in 1..=k {
            buy[j] = buy[j].max(sell[j - 1] - p);
            sell[j] = sell[j].max(buy[j] + p);
        }
    }
    sell[k]
}

fn main() {
    let prices = [5, 2, 4, 0, 1];
    println!("{}", max_profit(2, &prices));
}
