// Day 207: Check if an undirected graph is bipartite.
// BFS 2-coloring: color each component, fail if an edge joins same colors. Handles disconnected graphs.
// Time: O(V + E), Space: O(V).
function isBipartite(n, adj) {
  const color = new Array(n).fill(-1);
  for (let s = 0; s < n; s++) {
    if (color[s] !== -1) continue;
    color[s] = 0;
    const q = [s];
    while (q.length) {
      const u = q.shift();
      for (const v of adj[u]) {
        if (color[v] === -1) { color[v] = color[u] ^ 1; q.push(v); }
        else if (color[v] === color[u]) return false;
      }
    }
  }
  return true;
}

function graph(edges, n) {
  const adj = Array.from({ length: n }, () => []);
  for (const [a, b] of edges) { adj[a].push(b); adj[b].push(a); }
  return adj;
}

console.log(isBipartite(4, graph([[0, 1], [1, 2], [2, 3], [3, 0]], 4))); // true
console.log(isBipartite(3, graph([[0, 1], [1, 2], [2, 0]], 3)));         // false
