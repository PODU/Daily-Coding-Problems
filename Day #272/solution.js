// Day 272: throw_dice(N, faces, total) -> number of ways to reach total.
// 1-D DP over rolling sums. Time O(N*total*faces), Space O(total).
function throwDice(N, faces, total) {
  let dp = new Array(total + 1).fill(0);
  dp[0] = 1;
  for (let d = 0; d < N; d++) {
    const ndp = new Array(total + 1).fill(0);
    for (let t = 0; t <= total; t++) {
      if (!dp[t]) continue;
      for (let f = 1; f <= faces && t + f <= total; f++) ndp[t + f] += dp[t];
    }
    dp = ndp;
  }
  return dp[total];
}

console.log(throwDice(3, 6, 7)); // 15
