// Region cutting by slashes: split each cell into 4 triangles, union-find.
// Time: O(N*M * alpha), Space: O(N*M).
function regions(g) {
  const n = g.length, m = g[0].length;
  const par = Array.from({ length: n * m * 4 }, (_, i) => i);
  function find(x) {
    while (par[x] !== x) { par[x] = par[par[x]]; x = par[x]; }
    return x;
  }
  function uni(a, b) { par[find(a)] = find(b); }

  for (let r = 0; r < n; r++) {
    for (let c = 0; c < m; c++) {
      const base = (r * m + c) * 4;
      const ch = g[r][c];
      if (ch === '/') { uni(base + 0, base + 3); uni(base + 1, base + 2); }
      else if (ch === '\\') { uni(base + 0, base + 1); uni(base + 2, base + 3); }
      else { uni(base + 0, base + 1); uni(base + 1, base + 2); uni(base + 2, base + 3); }
      if (c + 1 < m) uni(base + 1, ((r * m + c + 1) * 4) + 3);
      if (r + 1 < n) uni(base + 2, (((r + 1) * m + c) * 4) + 0);
    }
  }
  let cnt = 0;
  for (let i = 0; i < par.length; i++) if (find(i) === i) cnt++;
  return cnt;
}

const g = [
  "\\    /",
  " \\  / ",
  "  \\/  ",
];
console.log(regions(g));
