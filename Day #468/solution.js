// Rotate NxN matrix 90 deg clockwise in place: transpose then reverse each row.
// Time: O(n^2), Space: O(1).
function rotate(m) {
  const n = m.length;
  for (let i = 0; i < n; i++)
    for (let j = i + 1; j < n; j++) {
      const t = m[i][j];
      m[i][j] = m[j][i];
      m[j][i] = t;
    }
  for (let i = 0; i < n; i++) m[i].reverse();
}

const m = [[1, 2, 3], [4, 5, 6], [7, 8, 9]];
rotate(m);
const n = m.length;
const lines = [];
for (let i = 0; i < n; i++) {
  const prefix = i === 0 ? "[[" : " [";
  const body = m[i].join(", ");
  const suffix = i < n - 1 ? "]" : "]]";
  lines.push(prefix + body + suffix);
}
console.log(lines.join(",\n"));
