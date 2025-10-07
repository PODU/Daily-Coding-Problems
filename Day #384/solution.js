// Min coins via bottom-up DP. Returns null if unreachable.
// Time: O(amount * |coins|), Space: O(amount).
function minCoins(coins, amount) {
  const INF = Infinity;
  const dp = new Array(amount + 1).fill(INF);
  dp[0] = 0;
  for (let a = 1; a <= amount; a++) {
    for (const c of coins) {
      if (c <= a && dp[a - c] + 1 < dp[a]) dp[a] = dp[a - c] + 1;
    }
  }
  return dp[amount] === INF ? null : dp[amount];
}

console.log(minCoins([1, 5, 10], 56));
console.log(minCoins([5, 8], 15));
