// Minimum cost to connect all houses to plant = Minimum Spanning Tree weight.
// Prim's algorithm with a min-heap. Time O(E log V), Space O(V + E).
'use strict';

// Tiny binary min-heap keyed by weight.
class MinHeap {
  constructor() { this.a = []; }
  push(x) { const a = this.a; a.push(x); let i = a.length - 1;
    while (i > 0) { const p = (i - 1) >> 1; if (a[p][0] <= a[i][0]) break;
      [a[p], a[i]] = [a[i], a[p]]; i = p; } }
  pop() { const a = this.a; const top = a[0], last = a.pop();
    if (a.length) { a[0] = last; let i = 0;
      for (;;) { let l = 2*i+1, r = 2*i+2, s = i;
        if (l < a.length && a[l][0] < a[s][0]) s = l;
        if (r < a.length && a[r][0] < a[s][0]) s = r;
        if (s === i) break; [a[s], a[i]] = [a[i], a[s]]; i = s; } }
    return top; }
  get size() { return this.a.length; }
}

function minCost(g) {
  const nodes = Object.keys(g);
  if (nodes.length === 0) return 0;
  const visited = new Set();
  const h = new MinHeap();
  h.push([0, nodes[0]]);
  let total = 0;
  while (h.size) {
    const [w, u] = h.pop();
    if (visited.has(u)) continue;
    visited.add(u);
    total += w;
    for (const [v, cost] of Object.entries(g[u]))
      if (!visited.has(v)) h.push([cost, v]);
  }
  return total;
}

function toUndirected(directed) {
  const g = {};
  for (const n of Object.keys(directed)) g[n] = {};
  for (const [a, nbrs] of Object.entries(directed))
    for (const [b, c] of Object.entries(nbrs)) {
      (g[a] = g[a] || {})[b] = c;
      (g[b] = g[b] || {})[a] = c;
    }
  return g;
}

const pipes = {
  plant: { A: 1, B: 5, C: 20 },
  A: { C: 15 },
  B: { C: 10 },
  C: {},
};
console.log(minCost(toUndirected(pipes))); // 16
