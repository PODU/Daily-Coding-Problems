// Max edges removed so every component has even node count. DFS subtree sizes;
// answer = count of non-root nodes with even subtree size. Time O(n), Space O(n).
// Note: README narrates one valid single cut (3,4), but the MAXIMUM is 2:
// cut (1,3) and (3,4) -> {1,2},{3,5},{4,6,7,8}, all even.
function solve(n, edges, root = 1) {
  const adj = Array.from({ length: n + 1 }, () => []);
  for (const [a, b] of edges) {
    adj[a].push(b);
    adj[b].push(a);
  }
  let answer = 0;

  function dfs(u, parent) {
    let s = 1;
    for (const v of adj[u]) {
      if (v !== parent) s += dfs(v, u);
    }
    if (u !== root && s % 2 === 0) answer++;
    return s;
  }

  dfs(root, 0);
  return answer;
}

function main() {
  const edges = [[1, 2], [1, 3], [3, 4], [3, 5], [4, 6], [4, 7], [4, 8]];
  console.log(solve(8, edges));
}

main();
