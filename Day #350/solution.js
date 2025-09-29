// Min perfect squares summing to N via DP, then greedy-largest reconstruction.
// Time O(N*sqrt N), Space O(N).
function solve(n) {
  const dp = new Array(n + 1).fill(Infinity);
  dp[0] = 0;
  for (let i = 1; i <= n; i++)
    for (let s = 1; s * s <= i; s++)
      dp[i] = Math.min(dp[i], dp[i - s * s] + 1);

  // Reconstruct: greedily pick largest s with dp[i - s*s] == dp[i] - 1.
  const squares = [];
  let i = n;
  while (i > 0) {
    for (let s = Math.floor(Math.sqrt(i)); s >= 1; s--) {
      if (dp[i - s * s] === dp[i] - 1) {
        squares.push(s * s);
        i -= s * s;
        break;
      }
    }
  }

  return `${dp[n]} (${squares.join(" + ")})`;
}

for (const n of [4, 17, 18]) console.log(solve(n));
