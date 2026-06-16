// Day 1671: Min columns to remove so each column is non-decreasing top->bottom.
// Count columns containing any out-of-order adjacent pair. Time O(N*M), Space O(1).
function minDeletions(g) {
  if (g.length === 0) return 0;
  const rows = g.length, cols = g[0].length;
  let del = 0;
  for (let j = 0; j < cols; j++)
    for (let i = 0; i + 1 < rows; i++)
      if (g[i][j] > g[i + 1][j]) { del++; break; }
  return del;
}

console.log(minDeletions(["cba", "daf", "ghi"])); // 1
