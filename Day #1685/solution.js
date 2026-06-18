// Day 1685: Merge ranked preference lists -> topological sort (Kahn's BFS, FIFO,
// first-seen tie-break). Each adjacent pair in a list is an edge. Time O(V+E).
function interleave(lists) {
  const order = [];
  const seen = new Set();
  const adj = new Map();
  const edges = new Set();
  const indeg = new Map();

  for (const lst of lists) {
    for (const x of lst) {
      if (!seen.has(x)) {
        seen.add(x); order.push(x); adj.set(x, []); indeg.set(x, 0);
      }
    }
    for (let i = 0; i + 1 < lst.length; i++) {
      const a = lst[i], b = lst[i + 1], key = a + "->" + b;
      if (!edges.has(key)) { edges.add(key); adj.get(a).push(b); indeg.set(b, indeg.get(b) + 1); }
    }
  }
  const q = order.filter(x => indeg.get(x) === 0);
  let head = 0;
  const res = [];
  while (head < q.length) {
    const u = q[head++];
    res.push(u);
    for (const v of adj.get(u)) {
      indeg.set(v, indeg.get(v) - 1);
      if (indeg.get(v) === 0) q.push(v);
    }
  }
  return res;
}

console.log(JSON.stringify(interleave([[1,7,3],[2,1,6,7,9],[3,9,5]])));
// [2,1,6,7,3,9,5]
