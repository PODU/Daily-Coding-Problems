// Find all bridges with Tarjan's DFS: edge (u,v) is a bridge if low[v] > disc[u].
// Time: O(V + E), Space: O(V + E).

function findBridges(n, edges) {
  const adj = Array.from({ length: n }, () => []);
  for (const [u, v] of edges) { adj[u].push(v); adj[v].push(u); }
  const disc = new Array(n).fill(0);
  const low = new Array(n).fill(0);
  const bridges = [];
  let timer = 0;

  function dfs(u, parent) {
    disc[u] = low[u] = ++timer;
    let skippedParent = false;
    for (const v of adj[u]) {
      if (v === parent && !skippedParent) { skippedParent = true; continue; }
      if (disc[v] === 0) {
        dfs(v, u);
        low[u] = Math.min(low[u], low[v]);
        if (low[v] > disc[u]) bridges.push([Math.min(u, v), Math.max(u, v)]);
      } else {
        low[u] = Math.min(low[u], disc[v]);
      }
    }
  }

  for (let i = 0; i < n; i++) if (disc[i] === 0) dfs(i, -1);
  bridges.sort((a, b) => a[0] - b[0] || a[1] - b[1]);
  return bridges;
}

const edges = [[0, 1], [1, 2], [2, 0], [1, 3], [3, 4]];
for (const [u, v] of findBridges(5, edges)) console.log(`(${u}, ${v})`);
// (1, 3)
// (3, 4)
