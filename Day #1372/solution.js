// Shortest path on grid with walls via BFS. Time O(M*N), Space O(M*N).
// Returns null if unreachable.
function shortestPath(grid, start, end) {
  const m = grid.length, n = grid[0].length;
  if (grid[start[0]][start[1]] || grid[end[0]][end[1]]) return null;
  const dist = Array.from({ length: m }, () => new Array(n).fill(-1));
  dist[start[0]][start[1]] = 0;
  const q = [start];
  let head = 0;
  const dirs = [[-1, 0], [1, 0], [0, -1], [0, 1]];
  while (head < q.length) {
    const [r, c] = q[head++];
    if (r === end[0] && c === end[1]) return dist[r][c];
    for (const [dr, dc] of dirs) {
      const nr = r + dr, nc = c + dc;
      if (nr >= 0 && nr < m && nc >= 0 && nc < n && !grid[nr][nc] && dist[nr][nc] === -1) {
        dist[nr][nc] = dist[r][c] + 1;
        q.push([nr, nc]);
      }
    }
  }
  return null;
}

const f = false, t = true;
const board = [
  [f, f, f, f],
  [t, t, f, t],
  [f, f, f, f],
  [f, f, f, f],
];
const res = shortestPath(board, [3, 0], [0, 0]);
console.log(res === null ? "null" : res);
