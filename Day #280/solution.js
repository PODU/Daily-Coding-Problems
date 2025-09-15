// Day 280: Detect cycle in undirected graph via Union-Find.
// An edge joining two already-connected vertices implies a cycle.
// Time O(V + E * alpha(V)), Space O(V).
function hasCycle(n, edges) {
  const parent = Array.from({ length: n }, (_, i) => i);
  function find(x) {
    while (parent[x] !== x) {
      parent[x] = parent[parent[x]];
      x = parent[x];
    }
    return x;
  }
  for (const [u, v] of edges) {
    const ru = find(u), rv = find(v);
    if (ru === rv) return true;
    parent[ru] = rv;
  }
  return false;
}

console.log(hasCycle(4, [[0, 1], [1, 2], [2, 0], [2, 3]])); // true
console.log(hasCycle(4, [[0, 1], [1, 2], [2, 3]]));          // false
