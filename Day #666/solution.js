// Day 666: Minimize difference of two subset sums. Subset-sum DP over reachable sums up to total/2,
// pick reachable sum closest to total/2, reconstruct one subset. Time O(n*sum), Space O(n*sum).
function minDiffPartition(a) {
  const n = a.length, total = a.reduce((x, y) => x + y, 0), half = (total / 2) | 0;
  const dp = Array.from({ length: n + 1 }, () => new Array(half + 1).fill(false));
  for (let i = 0; i <= n; i++) dp[i][0] = true;
  for (let i = 1; i <= n; i++)
    for (let s = 0; s <= half; s++)
      dp[i][s] = dp[i - 1][s] || (s >= a[i - 1] && dp[i - 1][s - a[i - 1]]);
  let best = 0;
  for (let s = half; s >= 0; s--) if (dp[n][s]) { best = s; break; }
  const A = [], B = []; let s = best;
  for (let i = n; i >= 1; i--) {
    if (s >= a[i - 1] && dp[i - 1][s - a[i - 1]]) { A.push(a[i - 1]); s -= a[i - 1]; }
    else B.push(a[i - 1]);
  }
  return [A, B, total - 2 * best];
}

const [A, B, diff] = minDiffPartition([5, 10, 15, 20, 25]);
console.log(`{${A.join(", ")}} and {${B.join(", ")}}, difference of ${diff}`);
