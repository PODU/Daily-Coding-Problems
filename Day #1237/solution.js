// Count connected components (friend groups) via Union-Find.
// Time O(V+E a(V)), Space O(V).
function countGroups(adj) {
  const keys = Object.keys(adj).map(Number);
  const p = {};
  keys.forEach((k) => (p[k] = k));
  const find = (x) => { while (p[x] !== x) { p[x] = p[p[x]]; x = p[x]; } return x; };
  const unite = (a, b) => { p[find(a)] = find(b); };
  for (const u of keys) for (const v of adj[u]) unite(u, v);
  return new Set(keys.map(find)).size;
}

const adj = { 0: [1, 2], 1: [0, 5], 2: [0], 3: [6], 4: [], 5: [1], 6: [3] };
console.log(countGroups(adj));
