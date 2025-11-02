// Day 539: Detect a cycle in an undirected graph using union-find (disjoint set).
// For each edge, if endpoints already share a root -> cycle. Time O(E*alpha(V)), Space O(V).

function hasCycle(n, edges) {
  const parent = Array.from({ length: n }, (_, i) => i);
  const rank = new Array(n).fill(0);

  const find = (x) => {
    while (parent[x] !== x) {
      parent[x] = parent[parent[x]];
      x = parent[x];
    }
    return x;
  };

  const unite = (a, b) => {
    a = find(a); b = find(b);
    if (a === b) return false;           // already connected -> cycle
    if (rank[a] < rank[b]) [a, b] = [b, a];
    parent[b] = a;
    if (rank[a] === rank[b]) rank[a]++;
    return true;
  };

  for (const [a, b] of edges) {
    if (!unite(a, b)) return true;
  }
  return false;
}

const cyclic = [[0, 1], [1, 2], [2, 3], [3, 0]];
const tree = [[0, 1], [1, 2], [2, 3]];
console.log(hasCycle(4, cyclic));
console.log(hasCycle(4, tree));
