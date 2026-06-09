// Min subset-sum difference: 0/1 subset-sum DP over reachable sums, pick best<=total/2; backtrack subset. O(n*total) time/space.
'use strict';

function main() {
  const a = [5, 10, 15, 20, 25];
  const n = a.length;
  const total = a.reduce((x, y) => x + y, 0);

  // dp[i][s] = sum s reachable using first i items
  const dp = Array.from({ length: n + 1 }, () => new Array(total + 1).fill(false));
  dp[0][0] = true;
  for (let i = 1; i <= n; i++)
    for (let s = 0; s <= total; s++) {
      dp[i][s] = dp[i - 1][s];
      if (s >= a[i - 1] && dp[i - 1][s - a[i - 1]]) dp[i][s] = true;
    }

  let best = 0;
  for (let s = Math.floor(total / 2); s >= 0; s--)
    if (dp[n][s]) { best = s; break; }

  // Backtrack from last item to first to recover subset A
  const A = [];
  const used = new Array(n).fill(false);
  let s = best;
  for (let i = n; i >= 1; i--) {
    if (s >= a[i - 1] && dp[i - 1][s - a[i - 1]]) {
      A.push(a[i - 1]);
      used[i - 1] = true;
      s -= a[i - 1];
    }
  }
  A.sort((x, y) => x - y);

  const B = [];
  for (let i = 0; i < n; i++) if (!used[i]) B.push(a[i]);

  console.log('Minimum difference: ' + (total - 2 * best));
  console.log('Subset A: [' + A.join(', ') + ']');
  console.log('Subset B: [' + B.join(', ') + ']');
}

main();
