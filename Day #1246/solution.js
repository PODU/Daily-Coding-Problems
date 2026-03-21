// Day 1246: Maximum weight spanning tree.
// Approach: Kruskal's algorithm with edges sorted in DESCENDING weight + union-find.
// Time: O(E log E), Space: O(V + E).

class DSU {
  constructor(n) {
    this.p = Array.from({ length: n }, (_, i) => i);
    this.r = new Array(n).fill(0);
  }
  find(x) {
    while (this.p[x] !== x) {
      this.p[x] = this.p[this.p[x]];
      x = this.p[x];
    }
    return x;
  }
  unite(a, b) {
    a = this.find(a);
    b = this.find(b);
    if (a === b) return false;
    if (this.r[a] < this.r[b]) [a, b] = [b, a];
    this.p[b] = a;
    if (this.r[a] === this.r[b]) this.r[a]++;
    return true;
  }
}

// edges: [weight, u, v]
function maxSpanningTree(n, edges) {
  const dsu = new DSU(n);
  let total = 0;
  for (const [w, u, v] of [...edges].sort((x, y) => y[0] - x[0])) {
    if (dsu.unite(u, v)) total += w;
  }
  return total;
}

const edges = [[1, 0, 1], [2, 1, 2], [3, 2, 3], [4, 0, 3], [5, 0, 2]];
console.log(maxSpanningTree(4, edges)); // 11
