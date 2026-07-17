// Count knight's tours on NxN via DFS backtracking from every start cell.
// Worst-case exponential; fine for small N (N=5 -> 1728).
const DR = [-2, -2, -1, -1, 1, 1, 2, 2];
const DC = [-1, 1, -2, 2, -2, 2, -1, 1];

function countTours(n) {
  const target = n * n;
  let total = 0;

  function dfs(r, c, count, vis) {
    if (count === target) return 1;
    let t = 0;
    for (let k = 0; k < 8; k++) {
      const nr = r + DR[k], nc = c + DC[k];
      if (nr >= 0 && nr < n && nc >= 0 && nc < n && !vis[nr][nc]) {
        vis[nr][nc] = true;
        t += dfs(nr, nc, count + 1, vis);
        vis[nr][nc] = false;
      }
    }
    return t;
  }

  for (let r = 0; r < n; r++) {
    for (let c = 0; c < n; c++) {
      const vis = Array.from({ length: n }, () => new Array(n).fill(false));
      vis[r][c] = true;
      total += dfs(r, c, 1, vis);
    }
  }
  return total;
}

console.log(countTours(5)); // 1728
