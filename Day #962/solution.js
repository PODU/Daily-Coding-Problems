// Day 962: Count knight's tours on an N x N board (visit every square once).
// Approach: DFS backtracking from every start square. Time O(8^(N^2)) worst, Space O(N^2).

const MOVES = [[1,2],[1,-2],[-1,2],[-1,-2],[2,1],[2,-1],[-2,1],[-2,-1]];

function countTours(n) {
  let total = 0;
  function dfs(vis, x, y, count) {
    if (count === n * n) return 1;
    let t = 0;
    for (const [ddx, ddy] of MOVES) {
      const nx = x + ddx, ny = y + ddy;
      if (nx >= 0 && nx < n && ny >= 0 && ny < n && !vis[nx][ny]) {
        vis[nx][ny] = true;
        t += dfs(vis, nx, ny, count + 1);
        vis[nx][ny] = false;
      }
    }
    return t;
  }
  for (let i = 0; i < n; i++)
    for (let j = 0; j < n; j++) {
      const vis = Array.from({ length: n }, () => new Array(n).fill(false));
      vis[i][j] = true;
      total += dfs(vis, i, j, 1);
    }
  return total;
}

console.log(countTours(5)); // 1728
