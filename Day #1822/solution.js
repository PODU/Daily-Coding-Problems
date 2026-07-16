// Bipartite check via BFS 2-coloring. O(V+E) time, O(V) space.
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

const n = 4;
const adj = Array.from({ length: n }, () => []);
for (const [a, b] of [[0, 1], [1, 2], [2, 3], [3, 0]]) { // even cycle -> bipartite
  adj[a].push(b);
  adj[b].push(a);
}
console.log(isBipartite(n, adj) ? "true" : "false");
