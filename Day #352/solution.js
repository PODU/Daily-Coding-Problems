// Validate crossword: rotational symmetry, all white runs (H & V) length>=3, white cells connected. O(N^2).

function valid(grid) {
  const n = grid.length;
  // 1. rotational symmetry
  for (let i = 0; i < n; i++)
    for (let j = 0; j < n; j++)
      if ((grid[i][j] === '#') !== (grid[n - 1 - i][n - 1 - j] === '#')) return false;
  // 2a. horizontal runs >= 3
  for (let i = 0; i < n; i++) {
    let run = 0;
    for (let j = 0; j <= n; j++) {
      if (j < n && grid[i][j] === '.') run++;
      else { if (run > 0 && run < 3) return false; run = 0; }
    }
  }
  // 2b. vertical runs >= 3
  for (let j = 0; j < n; j++) {
    let run = 0;
    for (let i = 0; i <= n; i++) {
      if (i < n && grid[i][j] === '.') run++;
      else { if (run > 0 && run < 3) return false; run = 0; }
    }
  }
  // 3. connectivity
  const whites = [];
  for (let i = 0; i < n; i++)
    for (let j = 0; j < n; j++)
      if (grid[i][j] === '.') whites.push([i, j]);
  if (whites.length === 0) return true;
  const seen = new Set();
  const key = (i, j) => i * n + j;
  const q = [whites[0]];
  seen.add(key(whites[0][0], whites[0][1]));
  while (q.length) {
    const [i, j] = q.pop();
    for (const [di, dj] of [[1, 0], [-1, 0], [0, 1], [0, -1]]) {
      const ni = i + di, nj = j + dj;
      if (ni >= 0 && ni < n && nj >= 0 && nj < n && grid[ni][nj] === '.' && !seen.has(key(ni, nj))) {
        seen.add(key(ni, nj));
        q.push([ni, nj]);
      }
    }
  }
  return seen.size === whites.length;
}

function main() {
  const gridA = Array(5).fill(".....");
  const gridB = ["#....", ".....", ".....", ".....", "....."];
  console.log(String(valid(gridA)));
  console.log(String(valid(gridB)));
}

main();
