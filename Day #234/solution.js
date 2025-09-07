// Maximum spanning tree: Kruskal with edges sorted by descending weight + union-find.
// Time: O(E log E), Space: O(V).
function maxSpanningTree(n, edges) {
  const parent = Array.from({length: n}, (_, i) => i);
  const find = x => {
    while (parent[x] !== x) { parent[x] = parent[parent[x]]; x = parent[x]; }
    return x;
  };
  const unite = (a, b) => {
    const ra = find(a), rb = find(b);
    if (ra === rb) return false;
    parent[ra] = rb;
    return true;
  };
  let total = 0;
  const chosen = [];
  for (const [u, v, w] of [...edges].sort((a, b) => b[2] - a[2])) {
    if (unite(u, v)) { total += w; chosen.push([u, v]); }
  }
  return {total, chosen};
}

const n = 4;
const edges = [[0, 1, 1], [1, 2, 2], [2, 3, 3], [0, 3, 4], [0, 2, 5]];
const {total, chosen} = maxSpanningTree(n, edges);
console.log("Max spanning tree weight: " + total); // 11
console.log("Edges: " + chosen.map(([u, v]) => `(${u}-${v})`).join(" "));
