// Day 971: Rotate N x N matrix 90 degrees clockwise in place.
// Approach: transpose then reverse each row. Time O(N^2), Space O(1).

function rotate(m) {
  const n = m.length;
  for (let i = 0; i < n; i++)
    for (let j = i + 1; j < n; j++) {
      const t = m[i][j]; m[i][j] = m[j][i]; m[j][i] = t;
    }
  for (const row of m) row.reverse();
}

const m = [[1, 2, 3], [4, 5, 6], [7, 8, 9]];
rotate(m);
for (const row of m) console.log(row.join(' '));
// 7 4 1 / 8 5 2 / 9 6 3
