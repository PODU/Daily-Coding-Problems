// Shortest path on a grid with walls using BFS (4-directional).
// Time: O(M*N), Space: O(M*N).
function shortestPath(board, start, end) {
  const m = board.length, n = board[0].length;
  const visited = Array.from({ length: m }, () => new Array(n).fill(false));
  const dirs = [[-1, 0], [1, 0], [0, -1], [0, 1]];
  const q = [[start[0], start[1], 0]];
  visited[start[0]][start[1]] = true;
  let head = 0;
  while (head < q.length) {
    const [r, c, d] = q[head++];
    if (r === end[0] && c === end[1]) return d;
    for (const [dr, dc] of dirs) {
      const nr = r + dr, nc = c + dc;
      if (nr >= 0 && nr < m && nc >= 0 && nc < n && !visited[nr][nc] && !board[nr][nc]) {
        visited[nr][nc] = true;
        q.push([nr, nc, d + 1]);
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
console.log(res === null ? 'None' : res);
