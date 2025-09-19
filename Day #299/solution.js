// Minimum spanning tree cost via Kruskal + union-find over undirected weighted graph.
// Time: O(E log E), Space: O(V + E).
function mstCost(pipes) {
  const parent = {};
  for (const node of Object.keys(pipes)) parent[node] = node;
  const find = (x) => {
    while (parent[x] !== x) { parent[x] = parent[parent[x]]; x = parent[x]; }
    return x;
  };
  const unite = (a, b) => {
    const ra = find(a), rb = find(b);
    if (ra === rb) return false;
    parent[ra] = rb;
    return true;
  };

  const seen = new Set();
  const edges = [];
  for (const u of Object.keys(pipes)) {
    for (const [v, w] of Object.entries(pipes[u])) {
      const [a, b] = u < v ? [u, v] : [v, u];
      const key = `${a}|${b}|${w}`;
      if (!seen.has(key)) { seen.add(key); edges.push([w, a, b]); }
    }
  }
  edges.sort((x, y) => x[0] - y[0]);

  let total = 0;
  for (const [w, a, b] of edges) if (unite(a, b)) total += w;
  return total;
}

const pipes = {
  plant: { A: 1, B: 5, C: 20 },
  A: { C: 15 },
  B: { C: 10 },
  C: {},
};
console.log(mstCost(pipes));
