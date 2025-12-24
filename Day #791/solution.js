// throw_dice: DP over dice, dp[s] = ways to reach sum s. Rolling array.
// Time O(N*total*faces), Space O(total).
function throwDice(N, faces, total) {
  let dp = new Array(total + 1).fill(0);
  dp[0] = 1;
  for (let i = 0; i < N; i++) {
    const ndp = new Array(total + 1).fill(0);
    for (let s = 0; s <= total; s++) {
      if (dp[s] === 0) continue;
      for (let f = 1; f <= faces && s + f <= total; f++)
        ndp[s + f] += dp[s];
    }
    dp = ndp;
  }
  return dp[total];
}

console.log(throwDice(3, 6, 7));
