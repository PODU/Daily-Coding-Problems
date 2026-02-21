// Day 1108: Detect a cycle in an undirected graph using Union-Find.
// If an edge connects two already-connected vertices, there's a cycle.
// Time: O(E * alpha(V)). Space: O(V).
function hasCycle(n, edges) {
  const parent = [...Array(n).keys()];
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
  for (const [a, b] of edges) if (!unite(a, b)) return true;
  return false;
}

console.log(hasCycle(3, [[0, 1], [1, 2], [2, 0]])); // true
console.log(hasCycle(4, [[0, 1], [1, 2], [2, 3]])); // false
