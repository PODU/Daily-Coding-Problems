// Day 409: PageRank via power iteration with damping d=0.85.
// Approach: iterate score(j)=(1-d)/N + d*sum(score(i)/Ci); dangling rank
// is spread over all nodes. Time: O(iters*(N+E)), Space: O(N+E).
"use strict";

function pageRank(graph, d = 0.85, iters = 100, eps = 1e-12) {
  const nodes = Object.keys(graph);
  const N = nodes.length;
  let rank = {};
  for (const node of nodes) rank[node] = 1.0 / N;
  for (let it = 0; it < iters; it++) {
    // Dangling nodes (no out-links) leak rank; redistribute it evenly.
    let dangling = 0;
    for (const node of nodes) if (graph[node].length === 0) dangling += rank[node];
    const next = {};
    for (const node of nodes) next[node] = (1 - d) / N + (d * dangling) / N;
    for (const node of nodes) {
      const outs = graph[node];
      if (outs.length === 0) continue;
      const share = rank[node] / outs.length;
      for (const nbr of outs) next[nbr] += d * share;
    }
    let diff = 0;
    for (const node of nodes) diff += Math.abs(next[node] - rank[node]);
    rank = next;
    if (diff < eps) break;
  }
  return rank;
}

const graph = {
  A: ["B", "C"],
  B: ["C"],
  C: ["A"],
};
const rank = pageRank(graph);
for (const node of Object.keys(rank).sort())
  console.log(`${node}: ${rank[node].toFixed(4)}`);
