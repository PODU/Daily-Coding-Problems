// Flood fill via BFS from start pixel, recoloring 4-connected same-color region.
// Time: O(rows*cols), Space: O(rows*cols).
function floodFill(g, sr, sc, color) {
  const rows = g.length, cols = g[0].length;
  const start = g[sr][sc];
  if (start === color) return;
  const q = [[sr, sc]];
  g[sr][sc] = color;
  const dirs = [[-1, 0], [1, 0], [0, -1], [0, 1]];
  while (q.length) {
    const [r, c] = q.shift();
    for (const [dr, dc] of dirs) {
      const nr = r + dr, nc = c + dc;
      if (nr >= 0 && nr < rows && nc >= 0 && nc < cols && g[nr][nc] === start) {
        g[nr][nc] = color;
        q.push([nr, nc]);
      }
    }
  }
}

const g = [
  ['B', 'B', 'W'],
  ['W', 'W', 'W'],
  ['W', 'W', 'W'],
  ['B', 'B', 'B'],
];
floodFill(g, 2, 2, 'G');
for (const row of g) console.log(row.join(' '));
