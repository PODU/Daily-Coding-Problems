// Topological sort of courses (Kahn's algorithm with cycle detection).
// Lexicographic tie-break by sorting the ready frontier for deterministic order.
// Time: O((V+E) log V), Space: O(V+E).

function courseOrder(prereqs) {
  const adj = new Map();
  const indeg = new Map();
  for (const c of Object.keys(prereqs)) { indeg.set(c, 0); adj.set(c, []); }
  for (const [course, pres] of Object.entries(prereqs)) {
    for (const p of pres) {
      if (!adj.has(p)) adj.set(p, []);
      adj.get(p).push(course);
      indeg.set(course, (indeg.get(course) || 0) + 1);
      if (!indeg.has(p)) indeg.set(p, 0);
    }
  }
  let ready = [...indeg.entries()].filter(([, d]) => d === 0).map(([c]) => c).sort();
  const order = [];
  while (ready.length) {
    ready.sort();
    const c = ready.shift();
    order.push(c);
    for (const nx of adj.get(c) || []) {
      indeg.set(nx, indeg.get(nx) - 1);
      if (indeg.get(nx) === 0) ready.push(nx);
    }
  }
  return order.length === indeg.size ? order : null;
}

const prereqs = { CSC300: ['CSC100', 'CSC200'], CSC200: ['CSC100'], CSC100: [] };
console.log(courseOrder(prereqs)); // [ 'CSC100', 'CSC200', 'CSC300' ]
