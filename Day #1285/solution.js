// Day 1285: Print a string in zigzag form across k lines.
// Place char i at column i, row = triangle-wave of i. Time O(n*k) to render, Space O(n*k).
function zigzag(s, k) {
  const n = s.length;
  if (k <= 1) { console.log(s); return; }
  const period = 2 * (k - 1);
  const grid = Array.from({ length: k }, () => new Array(n).fill(" "));
  for (let i = 0; i < n; i++) {
    const pos = i % period;
    const row = pos < k ? pos : period - pos;
    grid[row][i] = s[i];
  }
  for (const row of grid) console.log(row.join("").replace(/\s+$/, ""));
}

zigzag("thisisazigzag", 4);
