// Tree diameter with edge weights: DFS returning max downward path; global best
// = sum of two largest child paths. Time O(N), Space O(N).
function longestPath(n, edges) {
  const adj = Array.from({ length: n }, () => []);
  for (const [u, v, w] of edges) {
    adj[u].push([v, w]);
    adj[v].push([u, w]);
  }
  let best = 0;

  function dfs(u, parent) {
    let max1 = 0, max2 = 0;
    for (const [v, w] of adj[u]) {
      if (v === parent) continue;
      const path = dfs(v, u) + w;
      if (path > max1) { max2 = max1; max1 = path; }
      else if (path > max2) { max2 = path; }
    }
    best = Math.max(best, max1 + max2);
    return max1;
  }

  dfs(0, -1);
  return best;
}

// a0 b1 c2 d3 e4 f5 g6 h7
const edges = [[0, 1, 3], [0, 2, 5], [0, 3, 8],
               [3, 4, 2], [3, 5, 4], [4, 6, 1], [4, 7, 1]];
console.log(longestPath(8, edges));
