// Transitive closure via DFS from each vertex. Time O(V*(V+E)), Space O(V^2).
function transitiveClosure(graph) {
  const n = graph.length;
  const M = Array.from({ length: n }, () => new Array(n).fill(0));
  for (let s = 0; s < n; s++) {
    const vis = new Array(n).fill(false);
    const stack = [s];
    while (stack.length) {
      const u = stack.pop();
      if (vis[u]) continue;
      vis[u] = true;
      M[s][u] = 1;
      for (const v of graph[u]) if (!vis[v]) stack.push(v);
    }
  }
  return M;
}

const graph = [[0, 1, 3], [1, 2], [2], [3]];
for (const row of transitiveClosure(graph)) {
  console.log("[" + row.join(", ") + "]");
}
