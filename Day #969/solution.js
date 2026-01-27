// Day 969: Count islands (4-connected groups of 1s) in a binary matrix.
// Approach: DFS flood fill from each unvisited land cell. Time O(R*C), Space O(R*C).

function numIslands(grid) {
  const g = grid.map((r) => r.slice());
  const rows = g.length, cols = g[0].length;
  function dfs(r, c) {
    if (r < 0 || r >= rows || c < 0 || c >= cols || g[r][c] === 0) return;
    g[r][c] = 0;
    dfs(r + 1, c); dfs(r - 1, c); dfs(r, c + 1); dfs(r, c - 1);
  }
  let count = 0;
  for (let i = 0; i < rows; i++)
    for (let j = 0; j < cols; j++)
      if (g[i][j] === 1) { count++; dfs(i, j); }
  return count;
}

const grid = [
  [1, 0, 0, 0, 0],
  [0, 0, 1, 1, 0],
  [0, 1, 1, 0, 0],
  [0, 0, 0, 0, 0],
  [1, 1, 0, 0, 1],
  [1, 1, 0, 0, 1],
];
console.log(numIslands(grid)); // 4
