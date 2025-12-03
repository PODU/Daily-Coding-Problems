// 8-puzzle solver via A* with Manhattan-distance heuristic (admissible -> optimal solution).
// State = 9-char string (blank='0'); explore by sliding a tile into the blank.
const GOAL = "123456780";

function manhattan(s) {
  let d = 0;
  for (let i = 0; i < 9; i++) {
    const v = s.charCodeAt(i) - 48;
    if (v === 0) continue;
    const gr = ((v - 1) / 3) | 0, gc = (v - 1) % 3, r = (i / 3) | 0, c = i % 3;
    d += Math.abs(gr - r) + Math.abs(gc - c);
  }
  return d;
}

function neighbors(s) {
  const z = s.indexOf("0");
  const zr = (z / 3) | 0, zc = z % 3;
  const res = [];
  for (const [dr, dc] of [[-1, 0], [1, 0], [0, -1], [0, 1]]) {
    const nr = zr + dr, nc = zc + dc;
    if (nr >= 0 && nr < 3 && nc >= 0 && nc < 3) {
      const nz = nr * 3 + nc;
      const a = s.split("");
      [a[z], a[nz]] = [a[nz], a[z]];
      res.push(a.join(""));
    }
  }
  return res;
}

class MinHeap {
  constructor() { this.a = []; }
  get size() { return this.a.length; }
  push(item) { // item = [priority, state]
    const a = this.a; a.push(item); let i = a.length - 1;
    while (i > 0) { const p = (i - 1) >> 1; if (a[p][0] <= a[i][0]) break; [a[p], a[i]] = [a[i], a[p]]; i = p; }
  }
  pop() {
    const a = this.a; const top = a[0]; const last = a.pop();
    if (a.length) {
      a[0] = last; let i = 0;
      for (;;) {
        let l = 2 * i + 1, r = 2 * i + 2, s = i;
        if (l < a.length && a[l][0] < a[s][0]) s = l;
        if (r < a.length && a[r][0] < a[s][0]) s = r;
        if (s === i) break;
        [a[s], a[i]] = [a[i], a[s]]; i = s;
      }
    }
    return top;
  }
}

function solve(start) {
  const g = new Map([[start, 0]]);
  const parent = new Map([[start, null]]);
  const pq = new MinHeap();
  pq.push([manhattan(start), start]);
  while (pq.size) {
    const [f, cur] = pq.pop();
    if (cur === GOAL) break;
    if (f > g.get(cur) + manhattan(cur)) continue; // stale entry
    for (const nb of neighbors(cur)) {
      const ng = g.get(cur) + 1;
      if (!g.has(nb) || ng < g.get(nb)) {
        g.set(nb, ng);
        parent.set(nb, cur);
        pq.push([ng + manhattan(nb), nb]);
      }
    }
  }
  const path = [];
  let cur = GOAL;
  while (cur !== null) { path.push(cur); cur = parent.get(cur); }
  path.reverse();
  return path;
}

const start = "123406758";
const path = solve(start);
const moves = path.length - 1;
console.log(`Solved in ${moves} moves`);
console.log("Goal board:");
const gb = path[path.length - 1];
for (let i = 0; i < 9; i += 3) console.log(gb.slice(i, i + 3).split("").join(" "));
