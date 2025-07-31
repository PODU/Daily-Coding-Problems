// Day 56: Graph k-colorability via backtracking.
// Time: O(k^V) worst case, Space: O(V).
function kColorable(g, k) {
  const n = g.length;
  const color = new Array(n).fill(0);

  function canColor(v) {
    if (v === n) return true;
    for (let c = 1; c <= k; c++) {
      let ok = true;
      for (let u = 0; u < n; u++)
        if (g[v][u] && color[u] === c) { ok = false; break; }
      if (!ok) continue;
      color[v] = c;
      if (canColor(v + 1)) return true;
      color[v] = 0;
    }
    return false;
  }

  return canColor(0);
}

// Triangle graph: needs 3 colors.
const g = [
  [0, 1, 1],
  [1, 0, 1],
  [1, 1, 0],
];
console.log(kColorable(g, 2)); // false
console.log(kColorable(g, 3)); // true
