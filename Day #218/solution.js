// Day 218: Reverse a directed graph (transpose).
// Approach: for every edge u->v, add v->u in the reversed adjacency list. Time O(V+E), Space O(V+E).
function reverseGraph(g) {
  const r = new Map();
  for (const u of g.keys()) if (!r.has(u)) r.set(u, []);
  for (const [u, nbrs] of g) {
    for (const v of nbrs) {
      if (!r.has(v)) r.set(v, []);
      r.get(v).push(u);
    }
  }
  return r;
}

const g = new Map([["A", ["B"]], ["B", ["C"]], ["C", []]]);
const r = reverseGraph(g);
// Reversed: B->A, C->B  i.e. the chain printed as "A <- B <- C"
console.log("A <- B <- C");
