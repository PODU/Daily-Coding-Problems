// Day 1805: Find all bridges via Tarjan's algorithm (disc/low, edge is bridge if low[v] > disc[u]).
// Parallel edges handled by skipping the parent edge once via edge id. O(V+E).
function findBridges(n, edges) {
  const adj = Array.from({ length: n }, () => []);
  edges.forEach(([u, v], i) => {
    adj[u].push([v, i]);
    adj[v].push([u, i]);
  });

  const disc = new Array(n).fill(-1);
  const low = new Array(n).fill(-1);
  let timer = 0;
  const result = [];

  function dfs(u, parentEdge) {
    disc[u] = low[u] = timer++;
    for (const [v, id] of adj[u]) {
      if (id === parentEdge) continue;       // skip the single parent edge once
      if (disc[v] === -1) {
        dfs(v, id);
        low[u] = Math.min(low[u], low[v]);
        if (low[v] > disc[u]) result.push([Math.min(u, v), Math.max(u, v)]);
      } else {
        low[u] = Math.min(low[u], disc[v]);
      }
    }
  }

  for (let i = 0; i < n; i++) if (disc[i] === -1) dfs(i, -1);
  result.sort((a, b) => (a[0] !== b[0] ? a[0] - b[0] : a[1] - b[1]));
  return result;
}

function main() {
  const n = 5;
  const edges = [[0, 1], [1, 2], [2, 0], [1, 3], [3, 4]];
  for (const [u, v] of findBridges(n, edges)) console.log(`${u} - ${v}`);
  // expected: 1 - 3 and 3 - 4
}

main();
