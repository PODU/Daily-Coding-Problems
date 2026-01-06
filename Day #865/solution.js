// Day 865: Minimum cost to connect all houses to the plant = MST cost.
// Approach: Kruskal's algorithm with union-find over all pipe edges.
// Time: O(E log E), Space: O(V + E).
'use strict';

function minCost(pipes) {
  const nodes = new Set(Object.keys(pipes));
  const edges = [];
  for (const a of Object.keys(pipes))
    for (const [b, c] of Object.entries(pipes[a])) {
      nodes.add(b);
      edges.push([c, a, b]);
    }
  const idx = new Map([...nodes].map((n, i) => [n, i]));
  const parent = [...idx.values()].map((_, i) => i);
  const find = (x) => {
    while (parent[x] !== x) { parent[x] = parent[parent[x]]; x = parent[x]; }
    return x;
  };

  edges.sort((p, q) => p[0] - q[0]);
  let total = 0;
  for (const [c, a, b] of edges) {
    const ra = find(idx.get(a)), rb = find(idx.get(b));
    if (ra !== rb) { parent[ra] = rb; total += c; }
  }
  return total;
}

const pipes = {
  plant: { A: 1, B: 5, C: 20 },
  A: { C: 15 },
  B: { C: 10 },
  C: {}
};
console.log(minCost(pipes)); // 16
