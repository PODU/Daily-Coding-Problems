// Maximum spanning tree: Kruskal with edges sorted DESC by weight + Union-Find (path compression, union by rank). O(E log E).
class DSU {
  constructor(n) {
    this.parent = Array.from({ length: n }, (_, i) => i);
    this.rank = new Array(n).fill(0);
  }
  find(x) {
    while (this.parent[x] !== x) {
      this.parent[x] = this.parent[this.parent[x]];
      x = this.parent[x];
    }
    return x;
  }
  unite(a, b) {
    a = this.find(a);
    b = this.find(b);
    if (a === b) return false;
    if (this.rank[a] < this.rank[b]) [a, b] = [b, a];
    this.parent[b] = a;
    if (this.rank[a] === this.rank[b]) this.rank[a]++;
    return true;
  }
}

function maxSpanningTree(n, edges) {
  const dsu = new DSU(n);
  let total = 0;
  for (const [u, v, w] of [...edges].sort((x, y) => y[2] - x[2])) {
    if (dsu.unite(u, v)) total += w;
  }
  return total;
}

const edges = [[0, 1, 1], [0, 2, 2], [0, 3, 3], [1, 2, 4], [2, 3, 5]];
console.log("Maximum spanning tree weight: " + maxSpanningTree(4, edges));
