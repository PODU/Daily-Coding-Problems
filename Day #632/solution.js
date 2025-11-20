// Day 632: Deduce coin denominations from a "ways to make change" array.
// Approach: reverse coin-change DP. If ways[i] exceeds count reachable with
// coins found so far, i is itself a denomination.
// Time: O(N * D), Space: O(N).
function findDenominations(ways) {
  const n = ways.length;
  const dp = new Array(n).fill(0);
  dp[0] = 1;
  const coins = [];
  for (let i = 1; i < n; i++) {
    if (dp[i] < ways[i]) {
      coins.push(i);
      for (let j = i; j < n; j++) dp[j] += dp[j - i];
    }
  }
  return coins;
}

const ways = [1, 0, 1, 1, 2];
const coins = findDenominations(ways);
if (coins.length > 1) {
  console.log(coins.slice(0, -1).join(", ") + ", and " + coins[coins.length - 1]);
} else {
  console.log(coins.join(", "));
}
