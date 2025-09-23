// 8-puzzle solver: A* search with Manhattan-distance heuristic; blank=0.
// Time ~O(b^d) bounded by states explored; Space O(states stored).
'use strict';

function manhattan(b) {
  let d = 0;
  for (let i = 0; i < 9; i++) {
    const v = b[i];
    if (v === 0) continue;
    const gr = Math.floor((v - 1) / 3), gc = (v - 1) % 3;
    const r = Math.floor(i / 3), c = i % 3;
    d += Math.abs(gr - r) + Math.abs(gc - c);
  }
  return d;
}

function solve(start, goal) {
  const sKey = start.join(','), gKey = goal.join(',');
  const moves = [[-3, "Up"], [3, "Down"], [-1, "Left"], [1, "Right"]];
  const g = new Map([[sKey, 0]]);
  const parent = new Map();
  // simple array-based priority queue (small search space)
  const pq = [[manhattan(start), 0, start]];
  const pop = () => {
    let bi = 0;
    for (let i = 1; i < pq.length; i++) if (pq[i][0] < pq[bi][0]) bi = i;
    return pq.splice(bi, 1)[0];
  };
  while (pq.length) {
    const [f, gc, cur] = pop();
    const ck = cur.join(',');
    if (gc > g.get(ck)) continue;
    if (ck === gKey) break;
    const blank = cur.indexOf(0);
    const r = Math.floor(blank / 3), c = blank % 3;
    for (const [delta, name] of moves) {
      if (name === "Up" && r === 0) continue;
      if (name === "Down" && r === 2) continue;
      if (name === "Left" && c === 0) continue;
      if (name === "Right" && c === 2) continue;
      const nb = blank + delta;
      const nx = cur.slice();
      [nx[blank], nx[nb]] = [nx[nb], nx[blank]];
      const nk = nx.join(',');
      const ng = gc + 1;
      if (!g.has(nk) || ng < g.get(nk)) {
        g.set(nk, ng);
        parent.set(nk, [cur, name]);
        pq.push([ng + manhattan(nx), ng, nx]);
      }
    }
  }
  const path = [];
  let curKey = gKey;
  while (curKey !== sKey) {
    const [prev, name] = parent.get(curKey);
    path.push(name);
    curKey = prev.join(',');
  }
  path.reverse();
  return path;
}

const start = [[1, 2, 3], [4, 5, 6], [7, 0, 8]].flat();
const goal = [[1, 2, 3], [4, 5, 6], [7, 8, 0]].flat();
const path = solve(start, goal);
console.log(`Solved in ${path.length} move(s): ${path.join(", ")}`);
