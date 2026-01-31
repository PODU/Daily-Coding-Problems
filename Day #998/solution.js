// Day 998: Graph k-colorability (adjacency matrix).
// Backtracking: try each color per vertex, skipping colors used by neighbors.
// O(k^V) worst case, O(V) extra space.

function canColor(graph, k) {
  const n = graph.length;
  const colors = new Array(n).fill(0);
  const ok = (v, c) => {
    for (let u = 0; u < n; u++) if (graph[v][u] && colors[u] === c) return false;
    return true;
  };
  const solve = (v) => {
    if (v === n) return true;
    for (let c = 1; c <= k; c++) {
      if (ok(v, c)) {
        colors[v] = c;
        if (solve(v + 1)) return true;
        colors[v] = 0;
      }
    }
    return false;
  };
  return solve(0);
}

const triangle = [[0, 1, 1], [1, 0, 1], [1, 1, 0]];
console.log(canColor(triangle, 2)); // false
console.log(canColor(triangle, 3)); // true
