// DFS subtree sizes; count non-root subtrees with even size = max edges removable.
// Time O(n), Space O(n).
function main() {
  const n = 8;
  const g = Array.from({ length: n + 1 }, () => []);
  const edges = [[1,2],[1,3],[3,4],[3,5],[4,6],[4,7],[4,8]];
  for (const [a, b] of edges) { g[a].push(b); g[b].push(a); }
  let ans = 0;
  function dfs(u, p) {
    let sz = 1;
    for (const v of g[u]) if (v !== p) sz += dfs(v, u);
    if (p !== -1 && sz % 2 === 0) ans++;
    return sz;
  }
  dfs(1, -1);
  console.log(ans);
}
main();
