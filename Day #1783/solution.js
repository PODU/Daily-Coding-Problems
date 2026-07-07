// Count regions split by '/','\\',' ' via Union-Find: each cell = 4 triangles (T,R,B,L).
// Union within cell per char and across neighbors. Time O(R*C*alpha), Space O(R*C).
"use strict";

class DSU {
  constructor(n) {
    this.p = Array.from({ length: n }, (_, i) => i);
  }
  find(x) {
    while (this.p[x] !== x) {
      this.p[x] = this.p[this.p[x]];
      x = this.p[x];
    }
    return x;
  }
  union(a, b) {
    this.p[this.find(a)] = this.find(b);
  }
}

function regions(grid) {
  const R = grid.length, C = grid[0].length;
  const dsu = new DSU(4 * R * C);
  const idx = (r, c, t) => 4 * (r * C + c) + t;
  for (let r = 0; r < R; r++) {
    for (let c = 0; c < C; c++) {
      const ch = grid[r][c];
      if (ch === " ") {
        dsu.union(idx(r, c, 0), idx(r, c, 1));
        dsu.union(idx(r, c, 1), idx(r, c, 2));
        dsu.union(idx(r, c, 2), idx(r, c, 3));
      } else if (ch === "/") {
        dsu.union(idx(r, c, 0), idx(r, c, 3));
        dsu.union(idx(r, c, 1), idx(r, c, 2));
      } else {
        dsu.union(idx(r, c, 0), idx(r, c, 1));
        dsu.union(idx(r, c, 2), idx(r, c, 3));
      }
      if (c + 1 < C) dsu.union(idx(r, c, 1), idx(r, c + 1, 3));
      if (r + 1 < R) dsu.union(idx(r, c, 2), idx(r + 1, c, 0));
    }
  }
  const roots = new Set();
  for (let i = 0; i < 4 * R * C; i++) roots.add(dsu.find(i));
  return roots.size;
}

function main() {
  const grid = ["\\    /", " \\  / ", "  \\/  "];
  console.log(regions(grid)); // 3
}

main();
