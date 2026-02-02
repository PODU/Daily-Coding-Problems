// Day 1003: Transitive closure of a graph (adjacency list -> reachability matrix).
// DFS from each vertex marking everything reachable. O(V*(V+E)) time, O(V^2) space.

function transitiveClosure(graph) {
  const n = graph.length;
  const M = Array.from({ length: n }, () => new Array(n).fill(0));
  function dfs(start, u) {
    for (const v of graph[u]) {
      if (M[start][v] === 0) {
        M[start][v] = 1;
        dfs(start, v);
      }
    }
  }
  for (let s = 0; s < n; s++) {
    M[s][s] = 1;
    dfs(s, s);
  }
  return M;
}

const graph = [[0, 1, 3], [1, 2], [2], [3]];
for (const row of transitiveClosure(graph)) console.log("[" + row.join(", ") + "]");
