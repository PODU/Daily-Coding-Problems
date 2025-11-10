// throw_dice: DP over dice count, dp[s] = ways to reach sum s. Time O(N*total*faces), Space O(total).
function throwDice(N, faces, total) {
  let dp = new Array(total + 1).fill(0);
  dp[0] = 1;
  for (let d = 1; d <= N; d++) {
    const ndp = new Array(total + 1).fill(0);
    for (let s = 0; s <= total; s++) {
      if (dp[s] === 0) continue;
      for (let f = 1; f <= faces && s + f <= total; f++) ndp[s + f] += dp[s];
    }
    dp = ndp;
  }
  return dp[total];
}

console.log(throwDice(3, 6, 7));
