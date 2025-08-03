// Count open knight's tours on N x N board via backtracking DFS from every start.
// Time exponential, Space O(N^2).
const MOVES = [[-2,-1],[-2,1],[-1,-2],[-1,2],[1,-2],[1,2],[2,-1],[2,1]];

function countTours(n) {
  let total = 0;
  const vis = Array.from({ length: n }, () => new Array(n).fill(false));

  function dfs(r, c, count) {
    if (count === n * n) return 1;
    let res = 0;
    for (const [dr, dc] of MOVES) {
      const nr = r + dr, nc = c + dc;
      if (nr >= 0 && nr < n && nc >= 0 && nc < n && !vis[nr][nc]) {
        vis[nr][nc] = true;
        res += dfs(nr, nc, count + 1);
        vis[nr][nc] = false;
      }
    }
    return res;
  }

  for (let r = 0; r < n; ++r) {
    for (let c = 0; c < n; ++c) {
      vis[r][c] = true;
      total += dfs(r, c, 1);
      vis[r][c] = false;
    }
  }
  return total;
}

console.log(countTours(5));
