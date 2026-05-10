// Day 1500: Number of islands via iterative DFS flood fill (4-directional).
// Time O(R*C), Space O(R*C).
function numIslands(grid) {
  const rows = grid.length, cols = grid[0].length;
  const g = grid.map((row) => row.slice());
  let count = 0;
  const dirs = [[1, 0], [-1, 0], [0, 1], [0, -1]];
  for (let sr = 0; sr < rows; sr++) {
    for (let sc = 0; sc < cols; sc++) {
      if (g[sr][sc] === 1) {
        count++;
        const stack = [[sr, sc]];
        g[sr][sc] = 0;
        while (stack.length) {
          const [r, c] = stack.pop();
          for (const [dr, dc] of dirs) {
            const nr = r + dr, nc = c + dc;
            if (nr >= 0 && nr < rows && nc >= 0 && nc < cols && g[nr][nc] === 1) {
              g[nr][nc] = 0;
              stack.push([nr, nc]);
            }
          }
        }
      }
    }
  }
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
console.log(numIslands(grid));
