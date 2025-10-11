// Day 408: Max profit with at most k stock transactions.
// Approach: DP tracking best buy/sell state per transaction in one pass.
// Time: O(n*k), Space: O(k). Example k=2, [5,2,4,0,1] -> 3.
"use strict";

function maxProfit(k, prices) {
  const n = prices.length;
  if (n === 0 || k === 0) return 0;
  // If k >= n/2, unlimited transactions are possible.
  if (k >= Math.floor(n / 2)) {
    let profit = 0;
    for (let i = 1; i < n; i++)
      if (prices[i] > prices[i - 1]) profit += prices[i] - prices[i - 1];
    return profit;
  }
  const buy = new Array(k + 1).fill(-Infinity);
  const sell = new Array(k + 1).fill(0);
  for (const price of prices) {
    for (let t = 1; t <= k; t++) {
      buy[t] = Math.max(buy[t], sell[t - 1] - price);
      sell[t] = Math.max(sell[t], buy[t] + price);
    }
  }
  return sell[k];
}

console.log(maxProfit(2, [5, 2, 4, 0, 1])); // 3
