// Day 1149: Number of islands - 4-directional flood fill.
// Iterative DFS marks visited land. Time O(R*C), Space O(R*C).
function numIslands(grid) {
  if (!grid.length) return 0;
  const g = grid.map((row) => row.slice());
  const rows = g.length, cols = g[0].length;
  let count = 0;
  const dirs = [[1, 0], [-1, 0], [0, 1], [0, -1]];
  for (let r = 0; r < rows; r++) {
    for (let c = 0; c < cols; c++) {
      if (g[r][c] === 1) {
        count++;
        const stack = [[r, c]];
        g[r][c] = 0;
        while (stack.length) {
          const [x, y] = stack.pop();
          for (const [dx, dy] of dirs) {
            const nx = x + dx, ny = y + dy;
            if (nx >= 0 && ny >= 0 && nx < rows && ny < cols && g[nx][ny] === 1) {
              g[nx][ny] = 0;
              stack.push([nx, ny]);
            }
          }
        }
      }
    }
  }
  return count;
}

const grid = [
  [1, 0, 0, 0, 0], [0, 0, 1, 1, 0], [0, 1, 1, 0, 0],
  [0, 0, 0, 0, 0], [1, 1, 0, 0, 1], [1, 1, 0, 0, 1]];
console.log(numIslands(grid)); // 4
