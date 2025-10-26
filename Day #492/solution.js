// Day 492: Graph m-colorability via backtracking.
// Assign each vertex a color 1..k, ensuring no adjacent pair (adjacency matrix) matches.
// Time: O(k^V) worst case, Space: O(V) for the color assignment + recursion stack.
function canColor(graph, k) {
  const n = graph.length;
  const colors = new Array(n).fill(0);

  function isSafe(v, c) {
    for (let u = 0; u < n; u++)
      if (graph[v][u] && colors[u] === c) return false;
    return true;
  }

  function solve(v) {
    if (v === n) return true;
    for (let c = 1; c <= k; c++) {
      if (isSafe(v, c)) {
        colors[v] = c;
        if (solve(v + 1)) return true;
        colors[v] = 0;
      }
    }
    return false;
  }

  return solve(0);
}

// Triangle K3: every pair adjacent.
const graph = [
  [0, 1, 1],
  [1, 0, 1],
  [1, 1, 0],
];
console.log("k=2 colorable: " + canColor(graph, 2));
console.log("k=3 colorable: " + canColor(graph, 3));
