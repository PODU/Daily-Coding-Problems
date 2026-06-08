// Day 1628: Rotate N x N matrix 90 degrees clockwise in place.
// Transpose then reverse each row. Time O(N^2), Space O(1).
function rotate(m) {
  const n = m.length;
  for (let i = 0; i < n; i++)
    for (let j = i + 1; j < n; j++) {
      [m[i][j], m[j][i]] = [m[j][i], m[i][j]];
    }
  for (const row of m) row.reverse();
  return m;
}

const mat = [[1, 2, 3], [4, 5, 6], [7, 8, 9]];
rotate(mat);
for (const row of mat) console.log(row.join(" "));
