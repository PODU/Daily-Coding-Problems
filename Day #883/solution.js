// Min steps to 1: dp[i] = 1 + min(dp[i-1], dp[i/a] over factors a). Time O(N*sqrt N), Space O(N).

function minSteps(n) {
  const dp = new Array(n + 1).fill(0);
  for (let i = 2; i <= n; i++) {
    dp[i] = dp[i - 1] + 1;
    for (let a = 2; a * a <= i; a++) {
      if (i % a === 0) {
        const larger = i / a;
        dp[i] = Math.min(dp[i], 1 + dp[larger]);
      }
    }
  }
  return dp[n];
}

console.log(minSteps(100));
