// Topological-order DP over DAG: dp[node][c]=max count of letter c on path from node.
// Kahn's algorithm; cycle => null. Time O((n+m)*26), Space O(n*26).
'use strict';

function largestPathValue(s, edges) {
  const n = s.length;
  const adj = Array.from({ length: n }, () => []);
  const indeg = new Array(n).fill(0);
  for (const [a, b] of edges) { adj[a].push(b); indeg[b]++; }
  const dp = Array.from({ length: n }, () => new Array(26).fill(0));
  const q = [];
  for (let i = 0; i < n; i++) if (indeg[i] === 0) q.push(i);
  let seen = 0, ans = 0, head = 0;
  while (head < q.length) {
    const u = q[head++]; seen++;
    const cu = s.charCodeAt(u) - 65;
    dp[u][cu]++;
    ans = Math.max(ans, dp[u][cu]);
    for (const v of adj[u]) {
      for (let c = 0; c < 26; c++) if (dp[u][c] > dp[v][c]) dp[v][c] = dp[u][c];
      if (--indeg[v] === 0) q.push(v);
    }
  }
  return seen < n ? null : ans;
}

console.log(String(largestPathValue("ABACA", [[0, 1], [0, 2], [2, 3], [3, 4]])));
console.log(String(largestPathValue("A", [[0, 0]])));
