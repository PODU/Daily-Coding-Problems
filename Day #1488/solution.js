// Day 1488: Topological sort of courses via Kahn's algorithm (BFS on in-degrees).
// Returns a valid ordering, or null if a cycle exists. Time O(V+E), Space O(V+E).

function topoSort(prereqs) {
  // prereqs[course] = list of its prerequisites.
  const indeg = new Map();
  const adj = new Map();
  for (const [course, ps] of Object.entries(prereqs)) {
    if (!indeg.has(course)) indeg.set(course, 0);
    for (const p of ps) if (!indeg.has(p)) indeg.set(p, 0);
  }
  for (const [course, ps] of Object.entries(prereqs)) {
    for (const p of ps) {
      if (!adj.has(p)) adj.set(p, []);
      adj.get(p).push(course); // p before course
      indeg.set(course, indeg.get(course) + 1);
    }
  }

  // Lexicographic min-extraction for deterministic output.
  let ready = [...indeg.keys()].filter((c) => indeg.get(c) === 0);
  const order = [];
  while (ready.length) {
    ready.sort();
    const c = ready.shift();
    order.push(c);
    for (const nxt of adj.get(c) || []) {
      indeg.set(nxt, indeg.get(nxt) - 1);
      if (indeg.get(nxt) === 0) ready.push(nxt);
    }
  }
  return order.length === indeg.size ? order : null;
}

const prereqs = { CSC300: ["CSC100", "CSC200"], CSC200: ["CSC100"], CSC100: [] };
const res = topoSort(prereqs);
console.log(res === null ? "null" : res);
