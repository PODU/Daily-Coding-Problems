// Day 1113 - Validate an American-style crossword grid ('#' black, '.' white)
// Approach: 180-degree symmetry, every white cell in across & down run >= 3,
// and white connectivity. Time: O(N^2), Space: O(N^2).

function isCrossword(grid) {
  const n = grid.length;
  if (n === 0 || grid.some(r => r.length !== n)) return false;

  for (let i = 0; i < n; i++)
    for (let j = 0; j < n; j++)
      if ((grid[i][j] === '#') !== (grid[n-1-i][n-1-j] === '#')) return false;

  for (let i = 0; i < n; i++)
    for (let j = 0; j < n; j++)
      if (grid[i][j] === '.') {
        let len = 1, k = j - 1;
        while (k >= 0 && grid[i][k] === '.') { len++; k--; }
        k = j + 1;
        while (k < n && grid[i][k] === '.') { len++; k++; }
        if (len < 3) return false;
        len = 1; k = i - 1;
        while (k >= 0 && grid[k][j] === '.') { len++; k--; }
        k = i + 1;
        while (k < n && grid[k][j] === '.') { len++; k++; }
        if (len < 3) return false;
      }

  const whites = [];
  for (let i = 0; i < n; i++)
    for (let j = 0; j < n; j++)
      if (grid[i][j] === '.') whites.push([i, j]);
  if (whites.length === 0) return true;

  const seen = Array.from({ length: n }, () => new Array(n).fill(false));
  const st = [whites[0]];
  seen[whites[0][0]][whites[0][1]] = true;
  let cnt = 1;
  const dirs = [[1,0],[-1,0],[0,1],[0,-1]];
  while (st.length) {
    const [i, j] = st.pop();
    for (const [di, dj] of dirs) {
      const ni = i + di, nj = j + dj;
      if (ni>=0&&ni<n&&nj>=0&&nj<n&&grid[ni][nj]==='.'&&!seen[ni][nj]) {
        seen[ni][nj] = true; cnt++; st.push([ni, nj]);
      }
    }
  }
  return cnt === whites.length;
}

const valid = ["...##", ".....", ".....", ".....", "##..."];
const invalid = [".....", ".....", ".....", ".....", "....#"];
console.log("Grid 1 valid:", isCrossword(valid) ? "True" : "False");
console.log("Grid 2 valid:", isCrossword(invalid) ? "True" : "False");
