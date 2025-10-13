// Day 423: Transitive closure via DFS from each vertex. O(V*(V+E)) time, O(V^2) space.
// M[i][j] = 1 iff j is reachable from i (each vertex reaches itself).
function transitiveClosure(g) {
  const n = g.length;
  const M = Array.from({ length: n }, () => new Array(n).fill(0));

  function dfs(src, u) {
    M[src][u] = 1;
    for (const v of g[u]) if (!M[src][v]) dfs(src, v);
  }

  for (let i = 0; i < n; i++) dfs(i, i);
  return M;
}

const g = [[0, 1, 3], [1, 2], [2], [3]];
for (const row of transitiveClosure(g)) console.log("[" + row.join(", ") + "]");
