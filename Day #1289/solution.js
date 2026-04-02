// Day 1289: PageRank via power iteration with damping factor d.
// Iterate score vector until convergence; dangling nodes spread mass uniformly.
// Time O(iters * (V + E)), Space O(V + E).
function pagerank(nodes, links, d = 0.85, iters = 100) {
  const n = nodes.length;
  const outCount = {};
  for (const node of nodes) outCount[node] = (links[node] || []).length;
  let score = {};
  for (const node of nodes) score[node] = 1 / n;

  for (let it = 0; it < iters; it++) {
    let danglingSum = 0;
    for (const node of nodes) if (outCount[node] === 0) danglingSum += score[node];
    const nw = {};
    for (const node of nodes) nw[node] = (1 - d) / n + (d * danglingSum) / n;
    for (const src of nodes) {
      if (outCount[src] === 0) continue;
      const share = (d * score[src]) / outCount[src];
      for (const dst of links[src]) nw[dst] += share;
    }
    score = nw;
  }
  return score;
}

const nodes = ["A", "B", "C", "D"];
const links = { A: ["B", "C"], B: ["C"], C: ["A"], D: ["C"] };
const score = pagerank(nodes, links);
for (const node of nodes) console.log(`${node}: ${score[node].toFixed(4)}`);
