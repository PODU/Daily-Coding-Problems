// Deduce coin denominations from ways[] via incremental unbounded-knapsack DP.
// Time O(N^2), Space O(N).
function findDenominations(ways) {
  const n = ways.length;
  const dp = new Array(n).fill(0);
  dp[0] = 1;
  const coins = [];
  for (let i = 1; i < n; i++) {
    if (ways[i] !== dp[i]) {
      coins.push(i);
      for (let j = i; j < n; j++) dp[j] += dp[j - i];
    }
  }
  return coins;
}

const ways = [1, 0, 1, 1, 2];
console.log("[" + findDenominations(ways).join(", ") + "]");
