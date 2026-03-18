// Graph k-colorability via backtracking: assign colors 1..k to vertices in order,
// skipping conflicts. Time O(k^n) worst case, Space O(n).
function isKColorable(g, k) {
  const n = g.length;
  const color = new Array(n).fill(0);

  function safe(v, c) {
    for (let i = 0; i < n; i++)
      if (g[v][i] && color[i] === c) return false;
    return true;
  }

  function colorize(v) {
    if (v === n) return true;
    for (let c = 1; c <= k; c++) {
      if (safe(v, c)) {
        color[v] = c;
        if (colorize(v + 1)) return true;
        color[v] = 0;
      }
    }
    return false;
  }

  return colorize(0);
}

const g = [[0, 1, 1], [1, 0, 1], [1, 1, 0]];
console.log(isKColorable(g, 2));
console.log(isKColorable(g, 3));
