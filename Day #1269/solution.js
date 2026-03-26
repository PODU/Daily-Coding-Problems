// Day 1269: Rotate N x N matrix 90 degrees clockwise, in place.
// Transpose then reverse each row. O(n^2) time, O(1) extra space.
function rotate(m) {
  const n = m.length;
  for (let i = 0; i < n; i++)
    for (let j = i + 1; j < n; j++) { const t = m[i][j]; m[i][j] = m[j][i]; m[j][i] = t; }
  for (const row of m) row.reverse();
}

const matrix = [[1, 2, 3], [4, 5, 6], [7, 8, 9]];
rotate(matrix);
console.log(JSON.stringify(matrix));
