// Day 1667: Course ordering via topological sort (Kahn's algorithm).
// Time O(V+E + V log V), Space O(V+E). Returns null if a cycle exists.
function courseOrder(prereqs) {
  const indeg = new Map();
  const adj = new Map();
  for (const [course, deps] of Object.entries(prereqs)) {
    if (!indeg.has(course)) indeg.set(course, 0);
    for (const d of deps) if (!indeg.has(d)) indeg.set(d, 0);
  }
  for (const [course, deps] of Object.entries(prereqs)) {
    for (const d of deps) {
      if (!adj.has(d)) adj.set(d, []);
      adj.get(d).push(course);
      indeg.set(course, indeg.get(course) + 1);
    }
  }
  // ready set kept sorted for deterministic lexicographic output
  let ready = [...indeg.entries()].filter(([, d]) => d === 0).map(([c]) => c).sort();
  const order = [];
  while (ready.length) {
    const c = ready.shift();
    order.push(c);
    for (const nxt of adj.get(c) || []) {
      indeg.set(nxt, indeg.get(nxt) - 1);
      if (indeg.get(nxt) === 0) {
        ready.push(nxt);
        ready.sort();
      }
    }
  }
  return order.length === indeg.size ? order : null;
}

const g = { CSC300: ["CSC100", "CSC200"], CSC200: ["CSC100"], CSC100: [] };
console.log(courseOrder(g)); // [ 'CSC100', 'CSC200', 'CSC300' ]
