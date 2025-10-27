// Validate American-style crossword grid: every white cell in a horizontal AND vertical run
// of length >=3, single connected component of white cells, 180-degree rotational symmetry.
// Time: O(N^2), Space: O(N^2).

function isCrossword(g) {
  const n = g.length;
  if (n === 0) return false;

  // Rule 4: rotational symmetry.
  for (let i = 0; i < n; i++)
    for (let j = 0; j < n; j++)
      if (g[i][j] !== g[n - 1 - i][n - 1 - j]) return false;

  // Rules 1 & 2: runs of length >= 3 in both directions.
  for (let i = 0; i < n; i++) {
    for (let j = 0; j < n; j++) {
      if (g[i][j] !== ".") continue;
      let l = j; while (l > 0 && g[i][l - 1] === ".") l--;
      let r = j; while (r < n - 1 && g[i][r + 1] === ".") r++;
      if (r - l + 1 < 3) return false;
      let t = i; while (t > 0 && g[t - 1][j] === ".") t--;
      let b = i; while (b < n - 1 && g[b + 1][j] === ".") b++;
      if (b - t + 1 < 3) return false;
    }
  }

  // Rule 3: connectivity via BFS.
  const whites = [];
  for (let i = 0; i < n; i++)
    for (let j = 0; j < n; j++)
      if (g[i][j] === ".") whites.push([i, j]);
  if (whites.length === 0) return true;
  const seen = new Set();
  const key = (x, y) => x * n + y;
  const queue = [whites[0]];
  seen.add(key(whites[0][0], whites[0][1]));
  const dirs = [[0, 1], [0, -1], [1, 0], [-1, 0]];
  while (queue.length) {
    const [x, y] = queue.shift();
    for (const [dx, dy] of dirs) {
      const nx = x + dx, ny = y + dy;
      if (nx >= 0 && nx < n && ny >= 0 && ny < n && g[nx][ny] === "." && !seen.has(key(nx, ny))) {
        seen.add(key(nx, ny));
        queue.push([nx, ny]);
      }
    }
  }
  return seen.size === whites.length;
}

function main() {
  const valid = [".....", ".....", ".....", ".....", "....."];
  const invalid = ["..#..", ".....", "#...#", ".....", "..#.."];
  console.log(isCrossword(valid) ? "true" : "false");
  console.log(isCrossword(invalid) ? "true" : "false");
}

main();
