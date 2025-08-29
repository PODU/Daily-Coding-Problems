// Day 186: Minimum subset-sum difference (partition problem).
// Subset-sum DP over total, pick best <= total/2, reconstruct. Time O(n*S), Space O(n*S).
function solve(a) {
  const n = a.length;
  const tot = a.reduce((x, y) => x + y, 0);
  const dp = Array.from({ length: n + 1 }, () => new Array(tot + 1).fill(false));
  dp[0][0] = true;
  for (let i = 1; i <= n; i++)
    for (let s = 0; s <= tot; s++)
      dp[i][s] = dp[i - 1][s] || (s >= a[i - 1] && dp[i - 1][s - a[i - 1]]);
  let best = 0;
  for (let s = Math.floor(tot / 2); s >= 0; s--) if (dp[n][s]) { best = s; break; }
  const sub = [], other = [];
  let s = best;
  for (let i = n; i >= 1; i--) {
    if (s >= a[i - 1] && dp[i - 1][s - a[i - 1]]) { sub.push(a[i - 1]); s -= a[i - 1]; }
    else other.push(a[i - 1]);
  }
  sub.reverse(); other.reverse();
  const fmt = (v) => "{" + v.join(", ") + "}";
  console.log(`${fmt(sub)} and ${fmt(other)}`);
}

solve([5, 10, 15, 20, 25]);
