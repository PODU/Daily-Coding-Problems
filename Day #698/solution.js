// Day 698: Count regions a grid of '/'/'\\'/' ' is divided into.
// Approach: split each cell into 4 triangles (top,right,bottom,left) and union by
// the slash type plus across neighbors (Union-Find). Time O(R*C*a), Space O(R*C).
function regions(grid) {
  const R = grid.length;
  const C = Math.max(...grid.map((s) => s.length));
  grid = grid.map((s) => s.padEnd(C, " "));
  const p = Array.from({ length: R * C * 4 }, (_, i) => i);
  const find = (x) => { while (p[x] !== x) { p[x] = p[p[x]]; x = p[x]; } return x; };
  const uni = (a, b) => { p[find(a)] = find(b); };

  for (let r = 0; r < R; r++)
    for (let c = 0; c < C; c++) {
      const base = (r * C + c) * 4; // 0=T,1=R,2=B,3=L
      const ch = grid[r][c];
      if (ch === "/") { uni(base, base + 3); uni(base + 1, base + 2); }
      else if (ch === "\\") { uni(base, base + 1); uni(base + 2, base + 3); }
      else { uni(base, base + 1); uni(base + 1, base + 2); uni(base + 2, base + 3); }
      if (c + 1 < C) uni(base + 1, (r * C + c + 1) * 4 + 3);
      if (r + 1 < R) uni(base + 2, ((r + 1) * C + c) * 4);
    }
  let cnt = 0;
  for (let i = 0; i < p.length; i++) if (find(i) === i) cnt++;
  return cnt;
}

console.log(regions(["\\    /", " \\  /", "  \\/"])); // 3
