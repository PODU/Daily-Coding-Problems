// Day 98: Word search via DFS backtracking from each cell, marking visited cells
// in place. O(M*N*4^L) time, O(L) recursion space.
function exists(board, word) {
  const rows = board.length, cols = board[0].length;
  function dfs(r, c, i) {
    if (i === word.length) return true;
    if (r < 0 || r >= rows || c < 0 || c >= cols || board[r][c] !== word[i])
      return false;
    const saved = board[r][c];
    board[r][c] = '#';
    const found = dfs(r + 1, c, i + 1) || dfs(r - 1, c, i + 1)
               || dfs(r, c + 1, i + 1) || dfs(r, c - 1, i + 1);
    board[r][c] = saved;
    return found;
  }
  for (let r = 0; r < rows; r++)
    for (let c = 0; c < cols; c++)
      if (dfs(r, c, 0)) return true;
  return false;
}

const board = [['A','B','C','E'],['S','F','C','S'],['A','D','E','E']];
console.log(exists(board, "ABCCED")); // true
console.log(exists(board, "SEE"));    // true
console.log(exists(board, "ABCB"));   // false
