// Knight's tour counting via plain DFS backtracking from every start cell.
// Time: exponential (bounded by tour search); Space: O(N^2) visited grid + recursion.
'use strict';

const DX = [1, 1, -1, -1, 2, 2, -2, -2];
const DY = [2, -2, 2, -2, 1, -1, 1, -1];

function countTours(n) {
  const visited = Array.from({ length: n }, () => new Array(n).fill(false));
  const target = n * n;
  let total = 0;

  function dfs(x, y, count) {
    if (count === target) { total++; return; }
    for (let d = 0; d < 8; d++) {
      const nx = x + DX[d], ny = y + DY[d];
      if (nx >= 0 && nx < n && ny >= 0 && ny < n && !visited[nx][ny]) {
        visited[nx][ny] = true;
        dfs(nx, ny, count + 1);
        visited[nx][ny] = false;
      }
    }
  }

  for (let i = 0; i < n; i++)
    for (let j = 0; j < n; j++) {
      visited[i][j] = true;
      dfs(i, j, 1);
      visited[i][j] = false;
    }
  return total;
}

console.log(countTours(1));
console.log(countTours(5));
