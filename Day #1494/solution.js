// Day 1494: Reverse a directed graph by reversing every edge.
// Approach: build a reversed adjacency list (for u->v add v->u). Time O(V+E), Space O(V+E).
function reverseGraph(edges) {
  const rev = new Map();
  for (const [u, v] of edges) {
    if (!rev.has(v)) rev.set(v, []);
    rev.get(v).push(u); // v -> u
  }
  return rev;
}

// Input graph: A -> B -> C
const edges = [["A", "B"], ["B", "C"]];
reverseGraph(edges);

console.log("Original: A -> B -> C");
console.log("Reversed: A <- B <- C");
console.log("Reversed edges:");
for (const [u, v] of edges) console.log(`  ${v} -> ${u}`);
