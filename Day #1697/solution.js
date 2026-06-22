// Validate American crossword grid: each white cell in horiz & vert runs of len>=3, connected, 180-deg symmetric.
// Time O(N^2), Space O(N^2).

function isValidCrossword(g) {
  const n = g.length;
  if (n === 0) return false;

  for (let i = 0; i < n; i++)
    for (let j = 0; j < n; j++)
      if (g[i][j] !== g[n - 1 - i][n - 1 - j]) return false;

  for (let i = 0; i < n; i++) {
    for (let j = 0; j < n; j++) {
      if (g[i][j] !== 0) continue;
      let l = j, r = j;
      while (l - 1 >= 0 && g[i][l - 1] === 0) l--;
      while (r + 1 < n && g[i][r + 1] === 0) r++;
      if (r - l + 1 < 3) return false;
      let t = i, b = i;
      while (t - 1 >= 0 && g[t - 1][j] === 0) t--;
      while (b + 1 < n && g[b + 1][j] === 0) b++;
      if (b - t + 1 < 3) return false;
    }
  }

  const cells = [];
  for (let i = 0; i < n; i++)
    for (let j = 0; j < n; j++)
      if (g[i][j] === 0) cells.push([i, j]);
  if (cells.length === 0) return false;
  const total = cells.length;

  const vis = Array.from({ length: n }, () => new Array(n).fill(false));
  const queue = [cells[0]];
  vis[cells[0][0]][cells[0][1]] = true;
  let seen = 0;
  const dirs = [[-1, 0], [1, 0], [0, -1], [0, 1]];
  while (queue.length) {
    const [r, c] = queue.shift();
    seen++;
    for (const [dr, dc] of dirs) {
      const nr = r + dr, nc = c + dc;
      if (nr >= 0 && nr < n && nc >= 0 && nc < n && !vis[nr][nc] && g[nr][nc] === 0) {
        vis[nr][nc] = true;
        queue.push([nr, nc]);
      }
    }
  }
  return seen === total;
}

function main() {
  const valid = Array.from({ length: 5 }, () => new Array(5).fill(0)); // all white
  console.log(isValidCrossword(valid) ? "true" : "false");

  const invalid = Array.from({ length: 5 }, () => new Array(5).fill(0));
  invalid[0][0] = 1; // breaks symmetry
  console.log(isValidCrossword(invalid) ? "true" : "false");
}

main();
