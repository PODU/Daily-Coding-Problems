// Transitive closure: each input row is [node, neighbors...]. DFS from every node
// marks all reachable vertices (incl. itself). Time O(V*(V+E)), Space O(V^2).

function transitiveClosure(graph) {
  const n = graph.length;
  const adj = Array.from({ length: n }, () => []);
  for (const r of graph) {
    const node = r[0];
    for (let i = 1; i < r.length; i++) adj[node].push(r[i]);
  }
  const M = Array.from({ length: n }, () => new Array(n).fill(0));
  const dfs = (u, row) => {
    row[u] = 1;
    for (const v of adj[u]) if (!row[v]) dfs(v, row);
  };
  for (let i = 0; i < n; i++) dfs(i, M[i]);
  return M;
}

const graph = [[0, 1, 3], [1, 2], [2], [3]];
for (const row of transitiveClosure(graph)) console.log("[" + row.join(", ") + "]");
