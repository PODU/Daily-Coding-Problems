// Day 1851: Zigzag string across k lines.
// Place char i at row=zigzag(i), col=i in a grid; print rows. O(n*k) build. Space O(n*k).

function zigzag(s, k) {
  const n = s.length;
  const grid = Array.from({ length: k }, () => new Array(n).fill(' '));
  const period = k <= 1 ? 1 : 2 * (k - 1);
  for (let i = 0; i < n; i++) {
    const pos = k <= 1 ? 0 : i % period;
    const row = pos < k ? pos : period - pos;
    grid[row][i] = s[i];
  }
  return grid.map((r) => r.join('').replace(/\s+$/, ''));
}

for (const line of zigzag('thisisazigzag', 4)) console.log(line);
