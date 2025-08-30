// Day 193: Max profit, unlimited transactions, fee charged per completed transaction.
// State DP: cash (no stock) / hold (holding). Time O(n), Space O(1).
function maxProfit(prices, fee) {
  if (prices.length === 0) return 0;
  let cash = 0, hold = -prices[0];
  for (let i = 1; i < prices.length; i++) {
    cash = Math.max(cash, hold + prices[i] - fee);
    hold = Math.max(hold, cash - prices[i]);
  }
  return cash;
}

console.log(maxProfit([1, 3, 2, 8, 4, 10], 2));
