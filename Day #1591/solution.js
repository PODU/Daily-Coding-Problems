// Subset Sum: boolean DP over reachable sums; reconstruct one subset by backtracking.
// Time O(n*k), Space O(n*k).
'use strict';

function subsetSum(S, k) {
  const n = S.length;
  const reach = Array.from({ length: n + 1 }, () => new Array(k + 1).fill(false));
  reach[0][0] = true;
  for (let i = 1; i <= n; i++) {
    for (let s = 0; s <= k; s++) {
      if (reach[i - 1][s]) reach[i][s] = true;
      if (s >= S[i - 1] && reach[i - 1][s - S[i - 1]]) reach[i][s] = true;
    }
  }
  if (!reach[n][k]) return null;
  const chosen = [];
  let s = k;
  for (let i = n; i >= 1; i--) {
    if (s >= S[i - 1] && reach[i - 1][s - S[i - 1]]) {
      chosen.push(S[i - 1]);
      s -= S[i - 1];
    }
  }
  return chosen;
}

const S = [12, 1, 61, 5, 9, 2];
const k = 24;
const sub = subsetSum(S, k);
console.log('[' + (sub ? sub.join(', ') : '') + ']');
console.log('Sum = ' + (sub ? sub.reduce((a, b) => a + b, 0) : 0));
