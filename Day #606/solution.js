// Count all open knight's tours: DFS backtracking from every start square,
// counting Hamiltonian paths. Time O(8^(N*N)) worst, Space O(N*N). N=5 -> 1728.
const MOVES = [[1, 2], [1, -2], [-1, 2], [-1, -2],
               [2, 1], [2, -1], [-2, 1], [-2, -1]];

function knightTours(n) {
  if (n === 0) return 0;
  const totalCells = n * n;
  const board = Array.from({ length: n }, () => new Array(n).fill(false));

  function dfs(x, y, visited) {
    if (visited === totalCells) return 1;
    let count = 0;
    for (const [dx, dy] of MOVES) {
      const nx = x + dx, ny = y + dy;
      if (nx >= 0 && nx < n && ny >= 0 && ny < n && !board[nx][ny]) {
        board[nx][ny] = true;
        count += dfs(nx, ny, visited + 1);
        board[nx][ny] = false;
      }
    }
    return count;
  }

  let total = 0;
  for (let i = 0; i < n; i++) {
    for (let j = 0; j < n; j++) {
      board[i][j] = true;
      total += dfs(i, j, 1);
      board[i][j] = false;
    }
  }
  return total;
}

console.log(knightTours(5));
