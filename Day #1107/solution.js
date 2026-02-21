// Day 1107: Max edges removable so every component has an even node count.
// DFS subtree sizes; every non-root node with an even-sized subtree => one cuttable edge.
// Time: O(V+E). Space: O(V).
function maxEdgesRemoved(n, edges, root = 1) {
  const adj = Array.from({ length: n + 1 }, () => []);
  for (const [a, b] of edges) { adj[a].push(b); adj[b].push(a); }
  let answer = 0;
  const dfs = (u, parent) => {
    let size = 1;
    for (const v of adj[u]) if (v !== parent) size += dfs(v, u);
    if (parent !== -1 && size % 2 === 0) answer++;
    return size;
  };
  dfs(root, -1);
  return answer;
}

const edges = [[1, 2], [1, 3], [3, 4], [3, 5], [4, 6], [4, 7], [4, 8]];
console.log(maxEdgesRemoved(8, edges)); // 2
