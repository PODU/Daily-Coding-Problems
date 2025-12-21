// Day 778: Interleave ranked preference lists -> topological sort (Kahn's).
// Consecutive items in each list become edges. FIFO queue + first-seen order.
// O(V + E).
function interleave(lists) {
  const order = [], seen = new Set();
  const adj = new Map(), indeg = new Map();
  for (const l of lists) {
    for (const x of l) if (!seen.has(x)) { seen.add(x); order.push(x); if (!indeg.has(x)) indeg.set(x, 0); }
    for (let i = 0; i + 1 < l.length; i++) {
      if (!adj.has(l[i])) adj.set(l[i], []);
      adj.get(l[i]).push(l[i + 1]);
      indeg.set(l[i + 1], (indeg.get(l[i + 1]) || 0) + 1);
    }
  }
  const q = order.filter(x => indeg.get(x) === 0);
  const res = [];
  while (q.length) {
    const u = q.shift();
    res.push(u);
    for (const v of (adj.get(u) || [])) {
      indeg.set(v, indeg.get(v) - 1);
      if (indeg.get(v) === 0) q.push(v);
    }
  }
  return res;
}

console.log(interleave([[1, 7, 3], [2, 1, 6, 7, 9], [3, 9, 5]]));
// [2, 1, 6, 7, 3, 9, 5]
