// Day 1065: Reverse a directed graph (reverse every edge).
// Approach: iterate all edges u->v, add v->u to reversed adjacency. Time O(V+E), Space O(V+E).

function reverseGraph(g) {
  const r = {};
  for (const u of Object.keys(g)) r[u] = []; // keep isolated nodes
  for (const [u, neighbors] of Object.entries(g)) {
    for (const v of neighbors) {
      if (!r[v]) r[v] = [];
      r[v].push(u);
    }
  }
  return r;
}

// A -> B -> C
const g = { A: ["B"], B: ["C"], C: [] };
const r = reverseGraph(g);
// Reversed: B -> A, C -> B  ("A <- B <- C")
console.log("A <- B <- C");
for (const u of Object.keys(r).sort()) {
  for (const v of r[u]) console.log(`${u} -> ${v}`);
}
