// Count ways to roll N dice (faces each) summing to total via DP.
// Time O(N*faces*total), Space O(total).
function throwDice(N, faces, total) {
  let dp = new Array(total + 1).fill(0);
  dp[0] = 1;
  for (let d = 0; d < N; d++) {
    const nxt = new Array(total + 1).fill(0);
    for (let s = 0; s <= total; s++)
      if (dp[s])
        for (let f = 1; f <= faces && s + f <= total; f++)
          nxt[s + f] += dp[s];
    dp = nxt;
  }
  return dp[total];
}

console.log(throwDice(3, 6, 7));
