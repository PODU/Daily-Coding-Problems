// Day 996: Maximum weight spanning tree.
// Kruskal's algorithm with edges sorted in DESCENDING weight + union-find.
// O(E log E) time, O(V) space.

function maxSpanningTree(n, edges) {
  const parent = Array.from({ length: n }, (_, i) => i);
  const find = (x) => {
    while (parent[x] !== x) {
      parent[x] = parent[parent[x]];
      x = parent[x];
    }
    return x;
  };
  const union = (a, b) => {
    const ra = find(a),
      rb = find(b);
    if (ra === rb) return false;
    parent[ra] = rb;
    return true;
  };
  let total = 0;
  for (const [w, u, v] of [...edges].sort((a, b) => b[0] - a[0])) {
    if (union(u, v)) total += w;
  }
  return total;
}

// [weight, u, v]
const edges = [[1, 0, 1], [3, 0, 2], [2, 1, 2], [4, 1, 3], [5, 2, 3]];
console.log("Maximum spanning tree weight:", maxSpanningTree(4, edges)); // 12
