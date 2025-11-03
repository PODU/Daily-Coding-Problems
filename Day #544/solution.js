// Subset-sum DP with reconstruction. O(n*k) time, O(n*k) space. Result sorted desc.
function subsetSum(s, k) {
  const n = s.length;
  const dp = Array.from({ length: n + 1 }, () => new Array(k + 1).fill(false));
  for (let i = 0; i <= n; i++) dp[i][0] = true;
  for (let i = 1; i <= n; i++)
    for (let j = 0; j <= k; j++) {
      dp[i][j] = dp[i - 1][j];
      if (j >= s[i - 1] && dp[i - 1][j - s[i - 1]]) dp[i][j] = true;
    }
  if (!dp[n][k]) return null;
  const res = [];
  let j = k;
  for (let i = n; i >= 1; i--) {
    if (!dp[i - 1][j]) { res.push(s[i - 1]); j -= s[i - 1]; }
  }
  res.sort((a, b) => b - a);
  return res;
}

function printRes(r) {
  if (r === null) { console.log("null"); return; }
  console.log("[" + r.join(", ") + "]");
}

const s = [12, 1, 61, 5, 9, 2];
printRes(subsetSum(s, 24));
printRes(subsetSum(s, 1000));
