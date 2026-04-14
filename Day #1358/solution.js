// Max profit with at most k transactions. If k>=n/2 -> unlimited (sum positive diffs).
// Else DP with buy/sell rolling arrays. Time O(n*k), Space O(k).

function maxProfit(k, prices) {
  const n = prices.length;
  if (n === 0 || k === 0) return 0;
  if (k >= Math.floor(n / 2)) {
    let profit = 0;
    for (let i = 1; i < n; i++)
      if (prices[i] > prices[i-1]) profit += prices[i] - prices[i-1];
    return profit;
  }
  const buy = new Array(k + 1).fill(-Infinity);
  const sell = new Array(k + 1).fill(0);
  for (const p of prices)
    for (let j = 1; j <= k; j++) {
      buy[j]  = Math.max(buy[j], sell[j-1] - p);
      sell[j] = Math.max(sell[j], buy[j] + p);
    }
  return sell[k];
}

console.log(maxProfit(2, [5, 2, 4, 0, 1]));
