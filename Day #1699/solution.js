// Count ways to roll N dice (faces each) summing to total via rolling 1D DP.
// Time O(N*total*faces), Space O(total).
function throwDice(n, faces, total) {
  let dp = new Array(total + 1).fill(0);
  dp[0] = 1;
  for (let k = 0; k < n; k++) {
    const ndp = new Array(total + 1).fill(0);
    for (let s = 0; s <= total; s++) {
      if (!dp[s]) continue;
      for (let f = 1; f <= faces && s + f <= total; f++) ndp[s + f] += dp[s];
    }
    dp = ndp;
  }
  return dp[total];
}

console.log(throwDice(3, 6, 7));
