// BFS shortest path on a boolean grid (false=walkable, true=wall).
// Time O(M*N), Space O(M*N).
function minSteps(board, start, end) {
  const M = board.length, N = board[0].length;
  if (board[start[0]][start[1]] || board[end[0]][end[1]]) return null;
  const visited = Array.from({ length: M }, () => new Array(N).fill(false));
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
        if (nr >= 0 && nr < M && nc >= 0 && nc < N && !visited[nr][nc] && !board[nr][nc]) {
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

const t = true, f = false;
const board = [
  [f, f, f, f],
  [t, t, f, t],
  [f, f, f, f],
  [f, f, f, f],
];
console.log(minSteps(board, [3, 0], [0, 0]));
