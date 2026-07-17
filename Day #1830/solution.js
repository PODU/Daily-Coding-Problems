// Word search: DFS backtracking from each cell. O(R*C*4^L) time, O(L) space.
function exists(board, word) {
  const R = board.length, C = board[0].length;
  function dfs(i, r, c) {
    if (i === word.length) return true;
    if (r < 0 || r >= R || c < 0 || c >= C || board[r][c] !== word[i]) return false;
    const saved = board[r][c];
    board[r][c] = "#";
    const found = dfs(i + 1, r + 1, c) || dfs(i + 1, r - 1, c) ||
                  dfs(i + 1, r, c + 1) || dfs(i + 1, r, c - 1);
    board[r][c] = saved;
    return found;
  }
  for (let r = 0; r < R; r++)
    for (let c = 0; c < C; c++)
      if (dfs(0, r, c)) return true;
  return false;
}

const board = [
  ["A", "B", "C", "E"],
  ["S", "F", "C", "S"],
  ["A", "D", "E", "E"],
];
for (const w of ["ABCCED", "SEE", "ABCB"])
  console.log(`exists(board, "${w}") returns ${exists(board, w) ? "true" : "false"}`);
