// Day 1201: Reverse a directed graph.
// Build reversed adjacency (for each edge u->v add v->u). Time O(V+E), Space O(V+E).
function reverseGraph(g) {
  const r = new Map();
  for (const u of g.keys()) if (!r.has(u)) r.set(u, []);
  for (const [u, vs] of g) for (const v of vs) {
    if (!r.has(v)) r.set(v, []);
    r.get(v).push(u);
  }
  return r;
}

function main() {
  const nodes = ["A", "B", "C"];
  const g = new Map();
  for (let i = 0; i + 1 < nodes.length; i++) {
    if (!g.has(nodes[i])) g.set(nodes[i], []);
    g.get(nodes[i]).push(nodes[i + 1]);
  }
  reverseGraph(g);
  console.log(nodes.join(" <- "));
}

main();
