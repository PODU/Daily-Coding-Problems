// Largest path value in a directed graph: DFS topological memo + color cycle detection.
// dp[u][c] = max count of letter c on a path ending at u. Cycle -> null. O((n+m)*26) time, O(n*26) space.
function largestPathValue(colors, edges) {
  const n = colors.length;
  const adj = Array.from({ length: n }, () => []);
  for (const [u, v] of edges) adj[u].push(v);
  const dp = Array.from({ length: n }, () => new Array(26).fill(0));
  const state = new Array(n).fill(0); // 0 unvisited, 1 in-stack, 2 done

  function dfs(u) {
    state[u] = 1;
    for (const v of adj[u]) {
      if (state[v] === 1) return false;       // back edge -> cycle
      if (state[v] === 0 && !dfs(v)) return false;
    }
    for (const v of adj[u])
      for (let c = 0; c < 26; c++) if (dp[v][c] > dp[u][c]) dp[u][c] = dp[v][c];
    dp[u][colors.charCodeAt(u) - 65]++;
    state[u] = 2;
    return true;
  }

  for (let i = 0; i < n; i++)
    if (state[i] === 0 && !dfs(i)) return null;
  let ans = 0;
  for (let i = 0; i < n; i++) for (let c = 0; c < 26; c++) ans = Math.max(ans, dp[i][c]);
  return ans;
}

const r1 = largestPathValue("ABACA", [[0, 1], [0, 2], [2, 3], [3, 4]]);
console.log(r1 === null ? "null" : r1);
const r2 = largestPathValue("A", [[0, 0]]);
console.log(r2 === null ? "null" : r2);
