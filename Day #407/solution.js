// Day 407: Minimum Spanning Tree of water pipes (Prim's algorithm).
// Approach: Prim with a binary min-heap over an undirected weighted graph.
// Time: O(E log V), Space: O(V + E). Example MST total cost = 16.
"use strict";

class MinHeap {
  constructor() { this.h = []; }
  push(x) {
    const h = this.h; h.push(x); let i = h.length - 1;
    while (i > 0) { const p = (i - 1) >> 1; if (h[p][0] <= h[i][0]) break; [h[p], h[i]] = [h[i], h[p]]; i = p; }
  }
  pop() {
    const h = this.h; const top = h[0], last = h.pop();
    if (h.length) { h[0] = last; let i = 0; const n = h.length;
      while (true) { let l = 2*i+1, r = 2*i+2, s = i;
        if (l < n && h[l][0] < h[s][0]) s = l;
        if (r < n && h[r][0] < h[s][0]) s = r;
        if (s === i) break; [h[s], h[i]] = [h[i], h[s]]; i = s; } }
    return top;
  }
  get size() { return this.h.length; }
}

function minimumCost(pipes) {
  const adj = {};
  for (const node of Object.keys(pipes)) adj[node] = adj[node] || [];
  for (const [u, nbrs] of Object.entries(pipes)) {
    for (const [v, w] of Object.entries(nbrs)) {
      (adj[u] = adj[u] || []).push([w, v]);
      (adj[v] = adj[v] || []).push([w, u]);
    }
  }
  const nodes = Object.keys(adj);
  if (nodes.length === 0) return 0;
  const visited = new Set();
  const pq = new MinHeap();
  pq.push([0, nodes[0]]);
  let total = 0;
  while (pq.size) {
    const [cost, node] = pq.pop();
    if (visited.has(node)) continue;
    visited.add(node);
    total += cost;
    for (const [w, nbr] of adj[node]) if (!visited.has(nbr)) pq.push([w, nbr]);
  }
  return total;
}

const pipes = {
  plant: { A: 1, B: 5, C: 20 },
  A: { C: 15 },
  B: { C: 10 },
  C: {},
};
console.log(minimumCost(pipes)); // 16
