// Day 92: Topological sort (Kahn's algorithm) over a prerequisite graph.
// Returns a valid course order or null if a cycle exists. O(V+E).
function courseOrder(prereqs) {
  const indeg = new Map(), adj = new Map();
  for (const c of Object.keys(prereqs)) indeg.set(c, 0);
  for (const [c, pres] of Object.entries(prereqs))
    for (const p of pres) {
      if (!adj.has(p)) adj.set(p, []);
      adj.get(p).push(c);
      indeg.set(c, indeg.get(c) + 1);
    }
  const ready = [...indeg].filter(([, d]) => d === 0).map(([c]) => c).sort();
  const order = [];
  while (ready.length) {
    ready.sort();
    const c = ready.shift();
    order.push(c);
    for (const n of (adj.get(c) || [])) {
      indeg.set(n, indeg.get(n) - 1);
      if (indeg.get(n) === 0) ready.push(n);
    }
  }
  return order.length === Object.keys(prereqs).length ? order : null;
}

const g = { CSC300: ["CSC100", "CSC200"], CSC200: ["CSC100"], CSC100: [] };
console.log(courseOrder(g)); // [ 'CSC100', 'CSC200', 'CSC300' ]
