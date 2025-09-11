// Zigzag print: place each char at advancing column, row bounces 0..k-1..0.
// Build k row buffers, fill columns; rtrim each row. Time O(n*k), Space O(n*k).
function zigzag(s, k) {
  const n = s.length;
  const grid = Array.from({ length: k }, () => new Array(n).fill(' '));
  let row = 0, dir = 1;
  for (let col = 0; col < n; col++) {
    grid[row][col] = s[col];
    if (k > 1) {
      if (row === 0) dir = 1;
      else if (row === k - 1) dir = -1;
      row += dir;
    }
  }
  return grid.map(r => r.join('').replace(/\s+$/, ''));
}

for (const r of zigzag("thisisazigzag", 4)) console.log(r);
