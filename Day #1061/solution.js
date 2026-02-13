// Shortest path on a boolean grid (true=wall) via BFS in 4 directions.
// Time: O(M*N), Space: O(M*N). Returns null if unreachable.
"use strict";

function shortestPath(board, start, end) {
  const m = board.length, n = board[0].length;
  if (board[start[0]][start[1]] || board[end[0]][end[1]]) return null;
  const seen = Array.from({ length: m }, () => new Array(n).fill(false));
  const q = [[start[0], start[1], 0]];
  seen[start[0]][start[1]] = true;
  const dirs = [[-1, 0], [1, 0], [0, -1], [0, 1]];
  let head = 0;
  while (head < q.length) {
    const [r, c, steps] = q[head++];
    if (r === end[0] && c === end[1]) return steps;
    for (const [dr, dc] of dirs) {
      const nr = r + dr, nc = c + dc;
      if (nr < 0 || nr >= m || nc < 0 || nc >= n) continue;
      if (seen[nr][nc] || board[nr][nc]) continue;
      seen[nr][nc] = true;
      q.push([nr, nc, steps + 1]);
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
console.log(shortestPath(board, [3, 0], [0, 0]));
