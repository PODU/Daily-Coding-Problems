// Bipartite check via BFS 2-coloring over all components.
// Time: O(V + E), Space: O(V).
"use strict";

function isBipartite(adj) {
  const n = adj.length;
  const color = new Array(n).fill(-1);
  for (let s = 0; s < n; s++) {
    if (color[s] !== -1) continue;
    color[s] = 0;
    const q = [s];
    while (q.length) {
      const u = q.shift();
      for (const v of adj[u]) {
        if (color[v] === -1) {
          color[v] = color[u] ^ 1;
          q.push(v);
        } else if (color[v] === color[u]) {
          return false;
        }
      }
    }
  }
  return true;
}

function build(n, edges) {
  const adj = Array.from({ length: n }, () => []);
  for (const [a, b] of edges) {
    adj[a].push(b);
    adj[b].push(a);
  }
  return adj;
}

const g1 = build(4, [[0, 1], [1, 2], [2, 3], [3, 0]]); // 4-cycle -> true
const g2 = build(3, [[0, 1], [1, 2], [2, 0]]);         // triangle -> false
console.log(isBipartite(g1));
console.log(isBipartite(g2));
