// Day 729: Interleave ranked preference lists into one consistent playlist.
// Approach: Build precedence DAG (consecutive pairs), Kahn topological sort (FIFO,
// first-appearance tie-break). Time: O(V + E), Space: O(V + E).

function interleave(lists) {
  const adj = new Map(), indeg = new Map(), order = [];
  const edges = new Set();
  for (const lst of lists) {
    for (const x of lst)
      if (!indeg.has(x)) { indeg.set(x, 0); adj.set(x, []); order.push(x); }
    for (let i = 0; i + 1 < lst.length; i++) {
      const key = lst[i] + "->" + lst[i + 1];
      if (!edges.has(key)) { edges.add(key); adj.get(lst[i]).push(lst[i + 1]); indeg.set(lst[i + 1], indeg.get(lst[i + 1]) + 1); }
    }
  }
  const q = order.filter((x) => indeg.get(x) === 0);
  const res = [];
  while (q.length) {
    const u = q.shift();
    res.push(u);
    for (const v of adj.get(u)) { indeg.set(v, indeg.get(v) - 1); if (indeg.get(v) === 0) q.push(v); }
  }
  return res;
}

console.log("[" + interleave([[1, 7, 3], [2, 1, 6, 7, 9], [3, 9, 5]]).join(", ") + "]");
