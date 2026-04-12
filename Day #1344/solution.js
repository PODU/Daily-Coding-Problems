// Bipartite 2-coloring via BFS over all components (sorted iteration for determinism).
// Time: O(V+E), Space: O(V+E).
"use strict";

function solve(students) {
  const color = new Map();
  const nodes = [...students.keys()].sort((a, b) => a - b);
  for (const start of nodes) {
    if (color.has(start)) continue;
    color.set(start, 0);
    const q = [start];
    while (q.length) {
      const u = q.shift();
      const nbrs = [...(students.get(u) || [])].sort((a, b) => a - b);
      for (const v of nbrs) {
        if (!color.has(v)) {
          color.set(v, color.get(u) ^ 1);
          q.push(v);
        } else if (color.get(v) === color.get(u)) {
          return false;
        }
      }
    }
  }
  const t0 = [], t1 = [];
  for (const [k, c] of color) (c === 0 ? t0 : t1).push(k);
  t0.sort((a, b) => a - b);
  t1.sort((a, b) => a - b);
  return [t0, t1];
}

function group(g) {
  return "{" + g.join(", ") + "}";
}

function main() {
  const s1 = new Map([[0, [3]], [1, [2]], [2, [1, 4]], [3, [0, 4, 5]], [4, [2, 3]], [5, [3]]]);
  const r1 = solve(s1);
  console.log(r1 ? `${group(r1[0])} and ${group(r1[1])}` : "False");

  const s2 = new Map([[0, [3]], [1, [2]], [2, [1, 3, 4]], [3, [0, 2, 4, 5]], [4, [2, 3]], [5, [3]]]);
  const r2 = solve(s2);
  console.log(r2 ? `${group(r2[0])} and ${group(r2[1])}` : "False");
}

main();
