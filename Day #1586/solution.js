// 8-puzzle solver via A* search with Manhattan-distance heuristic (admissible => optimal).
// Time: O(b^d * log) over states explored; Space: O(states) for visited/frontier.
'use strict';

const GOAL = '123456780';

function manhattan(b) {
  let d = 0;
  for (let i = 0; i < 9; i++) {
    const v = b.charCodeAt(i) - 48;
    if (v === 0) continue;
    const gi = v - 1;
    d += Math.abs(Math.floor(i / 3) - Math.floor(gi / 3)) + Math.abs((i % 3) - (gi % 3));
  }
  return d;
}

// Minimal binary heap keyed by f.
class Heap {
  constructor() { this.a = []; }
  push(x) { const a = this.a; a.push(x); let i = a.length - 1; while (i > 0) { const p = (i - 1) >> 1; if (a[p][0] <= a[i][0]) break;[a[p], a[i]] = [a[i], a[p]]; i = p; } }
  pop() { const a = this.a; const top = a[0]; const last = a.pop(); if (a.length) { a[0] = last; let i = 0; for (;;) { let l = 2 * i + 1, r = l + 1, s = i; if (l < a.length && a[l][0] < a[s][0]) s = l; if (r < a.length && a[r][0] < a[s][0]) s = r; if (s === i) break;[a[s], a[i]] = [a[i], a[s]]; i = s; } } return top; }
  get size() { return this.a.length; }
}

function solve(flat) {
  const start = flat.join('');
  const g = new Map([[start, 0]]);
  const parent = new Map([[start, start]]);
  const moved = new Map();
  const heap = new Heap();
  heap.push([manhattan(start), 0, start]);
  const dirs = [[-1, 0], [1, 0], [0, -1], [0, 1]];
  let found = false;
  while (heap.size) {
    const [, gc, cur] = heap.pop();
    if (cur === GOAL) { found = true; break; }
    if (gc > g.get(cur)) continue;
    const z = cur.indexOf('0');
    const zr = Math.floor(z / 3), zc = z % 3;
    for (const [dr, dc] of dirs) {
      const nr = zr + dr, nc = zc + dc;
      if (nr < 0 || nr > 2 || nc < 0 || nc > 2) continue;
      const nz = nr * 3 + nc;
      const arr = cur.split('');
      const tile = arr[nz];
      arr[z] = tile; arr[nz] = '0';
      const nb = arr.join('');
      const ng = gc + 1;
      if (ng < (g.has(nb) ? g.get(nb) : Infinity)) {
        g.set(nb, ng);
        parent.set(nb, cur);
        moved.set(nb, Number(tile));
        heap.push([ng + manhattan(nb), ng, nb]);
      }
    }
  }
  const tiles = [];
  if (found) {
    let cur = GOAL;
    while (cur !== start) { tiles.push(moved.get(cur)); cur = parent.get(cur); }
    tiles.reverse();
  }
  return tiles;
}

function main() {
  const start = [[1, 2, 3], [4, 5, 6], [0, 7, 8]];
  const flat = start.flat();
  const tiles = solve(flat);
  console.log(`Solved in ${tiles.length} moves`);
  console.log('Tiles slid: ' + tiles.join(', '));
}

main();
