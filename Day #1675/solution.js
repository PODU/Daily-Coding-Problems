// Day 1675: Detect a cycle in an undirected graph via Union-Find.
// Union endpoints; a cycle exists if an edge joins already-connected nodes.
// Time O(E*alpha(V)), Space O(V).
function hasCycle(n, edges) {
  const parent = Array.from({ length: n }, (_, i) => i);
  const find = (x) => {
    while (parent[x] !== x) { parent[x] = parent[parent[x]]; x = parent[x]; }
    return x;
  };
  for (const [a, b] of edges) {
    const ra = find(a), rb = find(b);
    if (ra === rb) return true;
    parent[ra] = rb;
  }
  return false;
}

console.log(hasCycle(4, [[0,1],[1,2],[2,3],[3,0]])); // true
console.log(hasCycle(4, [[0,1],[1,2],[2,3]]));       // false
