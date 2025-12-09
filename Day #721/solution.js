// Day 721: Maximum-weight spanning tree.
// Approach: Kruskal with edges sorted by DECREASING weight + union-find.
// Time: O(E log E), Space: O(V + E).

function maxSpanningTree(n, edges) {
  const parent = Array.from({ length: n }, (_, i) => i);
  const rank = new Array(n).fill(0);
  const find = (x) => {
    while (parent[x] !== x) { parent[x] = parent[parent[x]]; x = parent[x]; }
    return x;
  };
  const unite = (a, b) => {
    a = find(a); b = find(b);
    if (a === b) return false;
    if (rank[a] < rank[b]) [a, b] = [b, a];
    parent[b] = a;
    if (rank[a] === rank[b]) rank[a]++;
    return true;
  };
  let total = 0, used = 0;
  for (const [u, v, w] of [...edges].sort((x, y) => y[2] - x[2]))
    if (unite(u, v)) { total += w; used++; }
  return used === n - 1 ? total : -1;
}

const n = 4;
const edges = [[0,1,1],[0,2,2],[0,3,3],[1,2,4],[2,3,5]];
console.log("Maximum spanning tree weight:", maxSpanningTree(n, edges));
