// Subset Sum: boolean DP dp[i][j] = can make j with first i items, then backtrack.
// Found subset is sorted descending for a deterministic [12, 9, 2, 1] output.
// Time O(n*k), Space O(n*k).
function subsetSum(S, k) {
  const n = S.length;
  const dp = Array.from({ length: n + 1 }, () => new Array(k + 1).fill(false));
  for (let i = 0; i <= n; i++) dp[i][0] = true;
  for (let i = 1; i <= n; i++)
    for (let j = 1; j <= k; j++) {
      dp[i][j] = dp[i - 1][j];
      if (j >= S[i - 1] && dp[i - 1][j - S[i - 1]]) dp[i][j] = true;
    }
  if (!dp[n][k]) return null;
  const res = [];
  let j = k;
  for (let i = n; i >= 1; i--) {
    if (dp[i - 1][j]) continue;       // item i-1 not needed
    res.push(S[i - 1]);                // item i-1 must be included
    j -= S[i - 1];
  }
  res.sort((a, b) => b - a);
  return res;
}

function fmt(v) {
  if (v === null) return "null";
  return "[" + v.join(", ") + "]";
}

const S = [12, 1, 61, 5, 9, 2];
console.log(fmt(subsetSum(S, 24)));
