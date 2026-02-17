// Count connected components via Union-Find with path compression. Time ~O(N+E*alpha), Space O(N).
function countGroups(adj) {
  const parent = new Map();
  for (const u of adj.keys()) parent.set(u, u);
  function find(x) {
    while (parent.get(x) !== x) {
      parent.set(x, parent.get(parent.get(x)));
      x = parent.get(x);
    }
    return x;
  }
  function unite(a, b) { parent.set(find(a), find(b)); }
  for (const [u, nbrs] of adj) for (const v of nbrs) unite(u, v);
  const roots = new Set();
  for (const u of adj.keys()) roots.add(find(u));
  return roots.size;
}

const adj = new Map([
  [0, [1, 2]], [1, [0, 5]], [2, [0]], [3, [6]], [4, []], [5, [1]], [6, [3]]]);
console.log(countGroups(adj));
