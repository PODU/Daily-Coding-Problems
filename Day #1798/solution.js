// Grid shortest path via BFS over walkable cells. Time O(M*N), Space O(M*N).
// Walls are booleans (true=wall). Returns null if no path.
function shortestPath(grid, start, end) {
  const m = grid.length, n = grid[0].length;
  if (grid[start[0]][start[1]] || grid[end[0]][end[1]]) return null;
  const visited = Array.from({ length: m }, () => Array(n).fill(false));
  let q = [start];
  visited[start[0]][start[1]] = true;
  let steps = 0;
  const dirs = [[-1, 0], [1, 0], [0, -1], [0, 1]];
  while (q.length) {
    const next = [];
    for (const [r, c] of q) {
      if (r === end[0] && c === end[1]) return steps;
      for (const [dr, dc] of dirs) {
        const nr = r + dr, nc = c + dc;
        if (nr >= 0 && nr < m && nc >= 0 && nc < n && !visited[nr][nc] && !grid[nr][nc]) {
          visited[nr][nc] = true;
          next.push([nr, nc]);
        }
      }
    }
    q = next;
    steps++;
  }
  return null;
}

const F = false, T = true;
const grid = [
  [F, F, F, F],
  [T, T, F, T],
  [F, F, F, F],
  [F, F, F, F],
];
console.log(shortestPath(grid, [3, 0], [0, 0])); // 7
