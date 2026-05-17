// Minimum spanning tree (Prim's algorithm) over an undirected pipe graph.
// Returns total cost to connect every house to the plant.
// Time O(E log V) with a simple priority approach, Space O(V + E).
function mstCost(pipes) {
  const adj = {};
  const ensure = (n) => { if (!adj[n]) adj[n] = []; };
  for (const u in pipes) {
    ensure(u);
    for (const v in pipes[u]) {
      const w = pipes[u][v];
      ensure(v);
      adj[u].push([w, v]);
      adj[v].push([w, u]);
    }
  }
  const start = Object.keys(pipes)[0];
  const visited = new Set();
  // simple array-based min selection (small graphs); use a heap for large ones
  const pq = [[0, start]];
  let total = 0;
  while (pq.length) {
    pq.sort((a, b) => a[0] - b[0]);
    const [w, u] = pq.shift();
    if (visited.has(u)) continue;
    visited.add(u);
    total += w;
    for (const [cw, v] of adj[u]) if (!visited.has(v)) pq.push([cw, v]);
  }
  return total;
}

const pipes = {
  plant: { A: 1, B: 5, C: 20 },
  A: { C: 15 },
  B: { C: 10 },
  C: {},
};
console.log(mstCost(pipes));
