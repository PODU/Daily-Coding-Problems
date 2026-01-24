// Day 945: Longest path (diameter) in a weighted tree.
// DFS: at each node combine its two deepest downward branches. Time O(V+E), Space O(V).
const adj = new Map();
let best = 0;

function addEdge(u, v, w) {
  if (!adj.has(u)) adj.set(u, []);
  if (!adj.has(v)) adj.set(v, []);
  adj.get(u).push([v, w]);
  adj.get(v).push([u, w]);
}

// Returns the longest downward path length from node (excluding edge above it).
function dfs(node, parent) {
  let max1 = 0, max2 = 0;
  for (const [nb, w] of adj.get(node) || []) {
    if (nb === parent) continue;
    const d = dfs(nb, node) + w;
    if (d > max1) { max2 = max1; max1 = d; }
    else if (d > max2) { max2 = d; }
  }
  best = Math.max(best, max1 + max2);
  return max1;
}

addEdge("a", "b", 3); addEdge("a", "c", 5); addEdge("a", "d", 8);
addEdge("d", "e", 2); addEdge("d", "f", 4);
addEdge("e", "g", 1); addEdge("e", "h", 1);
dfs("a", null);
console.log(best); // 17 (path c -> a -> d -> f)
