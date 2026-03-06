// 8-puzzle solver: A* search with Manhattan-distance heuristic (admissible), reconstruct path.
// Time: O(b^d) bounded by states, Space: O(states).
'use strict';

function manhattan(b) {
  let d = 0;
  for (let i = 0; i < 9; i++) {
    const v = b[i];
    if (v === 0) continue;
    const gr = Math.floor((v - 1) / 3), gc = (v - 1) % 3;
    d += Math.abs(Math.floor(i / 3) - gr) + Math.abs((i % 3) - gc);
  }
  return d;
}

// Minimal binary min-heap keyed by f.
class Heap {
  constructor() { this.a = []; }
  push(x) {
    const a = this.a;
    a.push(x);
    let i = a.length - 1;
    while (i > 0) {
      const p = (i - 1) >> 1;
      if (a[p][0] <= a[i][0]) break;
      [a[p], a[i]] = [a[i], a[p]];
      i = p;
    }
  }
  pop() {
    const a = this.a;
    const top = a[0];
    const last = a.pop();
    if (a.length) {
      a[0] = last;
      let i = 0;
      for (;;) {
        let l = 2 * i + 1, r = 2 * i + 2, s = i;
        if (l < a.length && a[l][0] < a[s][0]) s = l;
        if (r < a.length && a[r][0] < a[s][0]) s = r;
        if (s === i) break;
        [a[s], a[i]] = [a[i], a[s]];
        i = s;
      }
    }
    return top;
  }
  get size() { return this.a.length; }
}

function solve(start, goal) {
  const key = (b) => b.join(',');
  const goalKey = key(goal);
  const pq = new Heap();
  const best = new Map();
  pq.push([manhattan(start), 0, start]);
  best.set(key(start), 0);
  const dirs = [[-1, 0], [1, 0], [0, -1], [0, 1]];
  while (pq.size) {
    const [f, g, b] = pq.pop();
    const bk = key(b);
    if (bk === goalKey) return g;
    if (g > best.get(bk)) continue;
    const z = b.indexOf(0);
    const r = Math.floor(z / 3), c = z % 3;
    for (const [dr, dc] of dirs) {
      const nr = r + dr, nc = c + dc;
      if (nr < 0 || nr > 2 || nc < 0 || nc > 2) continue;
      const j = nr * 3 + nc;
      const nb = b.slice();
      [nb[z], nb[j]] = [nb[j], nb[z]];
      const ng = g + 1;
      const nk = key(nb);
      if (!best.has(nk) || ng < best.get(nk)) {
        best.set(nk, ng);
        pq.push([ng + manhattan(nb), ng, nb]);
      }
    }
  }
  return -1;
}

const start = [1, 2, 3, 4, 0, 6, 7, 5, 8];
const goal = [1, 2, 3, 4, 5, 6, 7, 8, 0];
const moves = solve(start, goal);
console.log(`Solved in ${moves} moves`);
for (let r = 0; r < 3; r++) {
  const cells = [];
  for (let c = 0; c < 3; c++) {
    const v = goal[r * 3 + c];
    cells.push(v === 0 ? '_' : String(v));
  }
  console.log(cells.join(' '));
}
