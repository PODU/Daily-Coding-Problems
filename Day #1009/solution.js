// Word Search: DFS backtracking from each cell, marking visited in-place.
// Time: O(M*N*4^L), Space: O(L) recursion depth.
function exists(board, word) {
  const rows = board.length, cols = board[0].length;
  function dfs(i, j, k) {
    if (k === word.length) return true;
    if (i < 0 || j < 0 || i >= rows || j >= cols || board[i][j] !== word[k])
      return false;
    const tmp = board[i][j];
    board[i][j] = '#';
    const found = dfs(i+1, j, k+1) || dfs(i-1, j, k+1) ||
                  dfs(i, j+1, k+1) || dfs(i, j-1, k+1);
    board[i][j] = tmp;
    return found;
  }
  for (let i = 0; i < rows; i++)
    for (let j = 0; j < cols; j++)
      if (dfs(i, j, 0)) return true;
  return false;
}

function main() {
  const board = [['A','B','C','E'],['S','F','C','S'],['A','D','E','E']];
  for (const w of ["ABCCED", "SEE", "ABCB"]) {
    const copy = board.map(r => r.slice());
    console.log(`${w}: ${exists(copy, w)}`);
  }
}

main();
