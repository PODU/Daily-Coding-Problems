// Day 84: Count islands (connected groups of 1s) via iterative DFS flood fill.
// Time O(rows*cols), Space O(rows*cols).
function numIslands(grid) {
  const g = grid.map((row) => row.slice());
  const rows = g.length, cols = g[0].length;
  let count = 0;
  for (let sr = 0; sr < rows; sr++) {
    for (let sc = 0; sc < cols; sc++) {
      if (g[sr][sc] === 1) {
        count++;
        const stack = [[sr, sc]];
        g[sr][sc] = 0;
        while (stack.length) {
          const [r, c] = stack.pop();
          for (const [dr, dc] of [[1, 0], [-1, 0], [0, 1], [0, -1]]) {
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
console.log(numIslands(grid)); // 4
