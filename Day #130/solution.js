// Day 130: Max profit with at most k buy/sell transactions.
// DP with hold/cash per transaction. O(n*k) time, O(k) space (greedy when k large).
function maxProfit(k, p) {
  const n = p.length;
  if (n === 0 || k === 0) return 0;
  if (k >= Math.floor(n / 2)) {
    let prof = 0;
    for (let i = 1; i < n; i++) if (p[i] > p[i - 1]) prof += p[i] - p[i - 1];
    return prof;
  }
  const buy = new Array(k + 1).fill(-Infinity);
  const sell = new Array(k + 1).fill(0);
  for (const price of p)
    for (let j = 1; j <= k; j++) {
      buy[j] = Math.max(buy[j], sell[j - 1] - price);
      sell[j] = Math.max(sell[j], buy[j] + price);
    }
  return sell[k];
}

const prices = [5, 2, 4, 0, 1];
console.log(maxProfit(2, prices)); // 3
