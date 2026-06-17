// Day 1682: Graph k-colorability via backtracking with pruning.
// Time O(k^V) worst case, Space O(V).
function canColor(adj, k) {
  const n = adj.length;
  const color = new Array(n).fill(-1);
  function solve(v) {
    if (v === n) return true;
    for (let c = 0; c < k; c++) {
      let ok = true;
      for (let u = 0; u < n; u++)
        if (adj[v][u] && color[u] === c) { ok = false; break; }
      if (ok) {
        color[v] = c;
        if (solve(v + 1)) return true;
        color[v] = -1;
      }
    }
    return false;
  }
  return solve(0);
}

const tri = [[0, 1, 1], [1, 0, 1], [1, 1, 0]];
console.log(canColor(tri, 2)); // false
console.log(canColor(tri, 3)); // true
