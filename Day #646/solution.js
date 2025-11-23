// Day 646: Count friend groups = connected components in an undirected graph.
// Approach: Union-Find (DSU) with path compression + union by rank.
// Time: O(V + E * alpha(V)), Space: O(V).

function countGroups(adj) {
  const parent = new Map(), rank = new Map();
  for (const u of adj.keys()) { parent.set(u, u); rank.set(u, 0); }
  const find = (x) => {
    while (parent.get(x) !== x) {
      parent.set(x, parent.get(parent.get(x)));
      x = parent.get(x);
    }
    return x;
  };
  const unite = (a, b) => {
    a = find(a); b = find(b);
    if (a === b) return;
    if (rank.get(a) < rank.get(b)) [a, b] = [b, a];
    parent.set(b, a);
    if (rank.get(a) === rank.get(b)) rank.set(a, rank.get(a) + 1);
  };
  for (const [u, nbrs] of adj) for (const v of nbrs) unite(u, v);
  const roots = new Set();
  for (const u of adj.keys()) roots.add(find(u));
  return roots.size;
}

const adj = new Map([
  [0, [1, 2]], [1, [0, 5]], [2, [0]], [3, [6]], [4, []], [5, [1]], [6, [3]],
]);
console.log(countGroups(adj)); // 3
