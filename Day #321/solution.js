// Min steps to reduce N to 1 (N-1 or replace with max factor): DP.
// Time O(N*sqrt(N)), Space O(N).
'use strict';

function minSteps(N) {
  const dp = new Array(N + 1).fill(0);
  for (let n = 2; n <= N; n++) {
    dp[n] = dp[n - 1] + 1;
    for (let a = 2; a * a <= n; a++) {
      if (n % a === 0) {
        const b = n / a;        // b >= a, max(a,b)=b
        dp[n] = Math.min(dp[n], dp[b] + 1);
      }
    }
  }
  return dp[N];
}

console.log(minSteps(100));
