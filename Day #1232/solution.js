// Count regions carved by slashes via 4-triangle split + Union-Find.
// Time O(n*m a(n*m)), Space O(n*m).
function regions(grid) {
  const rows = grid.length, cols = rows ? grid[0].length : 0;
  const p = Array.from({ length: 4 * rows * cols }, (_, i) => i);
  let count = p.length;
  const find = (x) => { while (p[x] !== x) { p[x] = p[p[x]]; x = p[x]; } return x; };
  const unite = (a, b) => { const ra = find(a), rb = find(b); if (ra !== rb) { p[ra] = rb; count--; } };
  const idx = (r, c, k) => 4 * (r * cols + c) + k;
  for (let r = 0; r < rows; r++)
    for (let c = 0; c < cols; c++) {
      const ch = grid[r][c];
      const t = idx(r, c, 0), ri = idx(r, c, 1), b = idx(r, c, 2), le = idx(r, c, 3);
      if (ch === '/') { unite(t, le); unite(ri, b); }
      else if (ch === '\\') { unite(t, ri); unite(le, b); }
      else { unite(t, ri); unite(ri, b); unite(b, le); }
      if (c + 1 < cols) unite(ri, idx(r, c + 1, 3));
      if (r + 1 < rows) unite(b, idx(r + 1, c, 0));
    }
  return count;
}

const grid = ["\\    /", " \\  / ", "  \\/  "];
console.log(regions(grid));
