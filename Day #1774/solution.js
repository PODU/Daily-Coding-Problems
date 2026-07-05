// Day 1774: Fewest steps to reduce N to 1 (decrement by 1, or jump to the larger
// factor of any factorization). DP over 1..N, trying every divisor. O(N*sqrt N) time, O(N) space.
function minSteps(N) {
  const dp = new Array(N + 1).fill(0);
  for (let i = 2; i <= N; i++) {
    dp[i] = dp[i - 1] + 1;                 // decrement step
    for (let a = 2; a * a <= i; a++) {
      if (i % a === 0) dp[i] = Math.min(dp[i], dp[i / a] + 1); // jump to larger factor
    }
  }
  return dp[N];
}

console.log(minSteps(100)); // 100->10->9->3->2->1 = 5
