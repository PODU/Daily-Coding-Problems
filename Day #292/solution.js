// Bipartite check (2-coloring) via BFS over enemy graph; split into two teams or False.
// Time O(V+E), Space O(V).
function twoColor(g, n) {
  const color = new Array(n).fill(-1);
  for (let s = 0; s < n; s++) {
    if (color[s] !== -1) continue;
    color[s] = 0;
    const q = [s];
    while (q.length) {
      const u = q.shift();
      for (const v of g[u]) {
        if (color[v] === -1) { color[v] = color[u] ^ 1; q.push(v); }
        else if (color[v] === color[u]) return null;
      }
    }
  }
  return color;
}

function solve(g, n) {
  const color = twoColor(g, n);
  if (color === null) { console.log("False"); return; }
  const a = [], b = [];
  for (let i = 0; i < n; i++) (color[i] === 0 ? a : b).push(i);
  const fmt = (s) => "{" + s.join(", ") + "}";
  console.log(`${fmt(a)} and ${fmt(b)}`);
}

const g1 = { 0: [3], 1: [2], 2: [1, 4], 3: [0, 4, 5], 4: [2, 3], 5: [3] };
const g2 = { 0: [3], 1: [2], 2: [1, 3, 4], 3: [0, 2, 4, 5], 4: [2, 3], 5: [3] };
solve(g1, 6);
solve(g2, 6);
