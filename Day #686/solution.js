// Remove max edges so each resulting subtree has an even node count.
// DFS subtree sizes; count non-root nodes whose subtree size is even (each = one removable edge above it). O(n) time/space.
function solve(n, edges, root = 1) {
  const adj = Array.from({ length: n + 1 }, () => []);
  for (const [a, b] of edges) { adj[a].push(b); adj[b].push(a); }

  let removable = 0;
  function dfs(u, parent) {
    let size = 1;
    for (const v of adj[u]) if (v !== parent) size += dfs(v, u);
    if (parent !== -1 && size % 2 === 0) removable++;
    return size;
  }
  dfs(root, -1);
  return removable;
}

const edges = [[1,2],[1,3],[3,4],[3,5],[4,6],[4,7],[4,8]];
console.log(solve(8, edges));
