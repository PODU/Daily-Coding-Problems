// Zigzag: place char i at (zigzag-row, column i) in a k x n grid; print rows. O(n*k).
function zigzag(s, k) {
  const n = s.length;
  const grid = Array.from({ length: k }, () => Array(n).fill(' '));
  let row = 0, dir = k === 1 ? 0 : 1;
  for (let i = 0; i < n; i++) {
    grid[row][i] = s[i];
    if (row === 0) dir = 1;
    else if (row === k - 1) dir = -1;
    row += dir;
  }
  for (const r of grid) console.log(r.join('').replace(/\s+$/, ''));
}

zigzag("thisisazigzag", 4);
