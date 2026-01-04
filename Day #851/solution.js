// Day 851: a graph is minimally-connected iff it is a tree: connected AND edges == nodes-1.
// Union-Find detects cycles and connectivity in O(E alpha(V)).
function isMinimallyConnected(n, edges) {
  if (edges.length !== n - 1) return false;
  const parent = Array.from({ length: n }, (_, i) => i);
  const find = (x) => {
    while (parent[x] !== x) { parent[x] = parent[parent[x]]; x = parent[x]; }
    return x;
  };
  for (const [a, b] of edges) {
    const ra = find(a), rb = find(b);
    if (ra === rb) return false; // cycle
    parent[ra] = rb;
  }
  const roots = new Set();
  for (let i = 0; i < n; i++) roots.add(find(i));
  return roots.size === 1;
}

console.log(isMinimallyConnected(5, [[0,1],[0,2],[1,3],[1,4]])); // true
console.log(isMinimallyConnected(4, [[0,1],[1,2],[2,0],[2,3]])); // false
