// PageRank: iterative power method with damping d=0.85, dangling-node mass redistributed evenly.
// Time O(iters*(N+E)), Space O(N+E).
function pagerank(nodes, edges, d = 0.85, iters = 100, tol = 1e-9) {
  const n = nodes.length;
  let score = {};
  for (const nd of nodes) score[nd] = 1.0 / n;
  for (let it = 0; it < iters; it++) {
    let dangling = 0;
    for (const nd of nodes) if (!(edges[nd] || []).length) dangling += score[nd];
    const next = {};
    for (const nd of nodes) next[nd] = (1.0 - d) / n + (d * dangling) / n;
    for (const nd of nodes) {
      const outs = edges[nd] || [];
      if (!outs.length) continue;
      const share = (d * score[nd]) / outs.length;
      for (const t of outs) next[t] += share;
    }
    let diff = 0;
    for (const nd of nodes) diff += Math.abs(next[nd] - score[nd]);
    score = next;
    if (diff < tol) break;
  }
  return score;
}

const nodes = ["A", "B", "C", "D"];
const edges = { A: ["B", "C"], B: ["C"], C: ["A"], D: ["C"] };
const score = pagerank(nodes, edges);
for (const nd of [...nodes].sort()) console.log(`${nd} ${score[nd].toFixed(6)}`);
