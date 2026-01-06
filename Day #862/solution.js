// Day 862: Find all bridges in a connected undirected graph.
// Approach: Tarjan's DFS using discovery times and low-link values.
// Time: O(V + E), Space: O(V + E).
'use strict';

function findBridges(n, edges) {
  const adj = Array.from({ length: n }, () => []);
  for (const [a, b] of edges) { adj[a].push(b); adj[b].push(a); }
  const disc = new Array(n).fill(0);
  const low = new Array(n).fill(0);
  let timer = 0;
  const bridges = [];

  function dfs(u, parent) {
    disc[u] = low[u] = ++timer;
    for (const v of adj[u]) {
      if (v === parent) continue;
      if (disc[v]) low[u] = Math.min(low[u], disc[v]);
      else {
        dfs(v, u);
        low[u] = Math.min(low[u], low[v]);
        if (low[v] > disc[u]) bridges.push([Math.min(u, v), Math.max(u, v)]);
      }
    }
  }

  for (let i = 0; i < n; i++) if (!disc[i]) dfs(i, -1);
  bridges.sort((a, b) => a[0] - b[0] || a[1] - b[1]);
  return bridges;
}

const res = findBridges(5, [[0, 1], [1, 2], [2, 0], [1, 3], [3, 4]]);
console.log('Bridges: ' + res.map(([a, b]) => `(${a}, ${b})`).join(' ')); // (1, 3) (3, 4)
