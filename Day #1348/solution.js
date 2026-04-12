// Weighted tree diameter: DFS, at each node track two largest downward child path sums;
// diameter = max over nodes of (sum of two largest). Time O(V+E), Space O(V+E).
function treeDiameter(n, edges) {
  const adj = Array.from({ length: n }, () => []);
  for (const [a, b, w] of edges) {
    adj[a].push([b, w]);
    adj[b].push([a, w]);
  }
  let best = 0;

  function dfs(u, parent) {
    let max1 = 0, max2 = 0; // two largest downward path sums
    for (const [v, w] of adj[u]) {
      if (v === parent) continue;
      const down = dfs(v, u) + w;
      if (down > max1) { max2 = max1; max1 = down; }
      else if (down > max2) { max2 = down; }
    }
    best = Math.max(best, max1 + max2);
    return max1;
  }

  dfs(0, -1);
  return best;
}

// a..h -> 0..7
const edges = [
  [0, 1, 3], // a-b
  [0, 2, 5], // a-c
  [0, 3, 8], // a-d
  [3, 4, 2], // d-e
  [3, 5, 4], // d-f
  [4, 6, 1], // e-g
  [4, 7, 1], // e-h
];
console.log(treeDiameter(8, edges));
