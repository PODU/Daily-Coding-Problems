// Transitive closure: DFS from each vertex marking reachable nodes (incl self).
// Time O(V*(V+E)), Space O(V^2) for the matrix.
function dfs(u, g, row) {
  row[u] = 1;
  for (const v of g[u]) if (!row[v]) dfs(v, g, row);
}

const graph = [[0, 1, 3], [1, 2], [2], [3]];
const n = graph.length;
const M = Array.from({ length: n }, () => new Array(n).fill(0));
for (let i = 0; i < n; i++) dfs(i, graph, M[i]);

for (const row of M) {
  console.log("[" + row.join(", ") + "]");
}
