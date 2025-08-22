// Day 151: Flood fill via BFS. Replace target pixel's connected same-colored
// region with new color. Time O(R*C), Space O(R*C).
'use strict';

function floodFill(m, r, c, color) {
  const R = m.length, C = m[0].length;
  const target = m[r][c];
  if (target === color) return m;
  const q = [[r, c]];
  m[r][c] = color;
  const dirs = [[-1, 0], [1, 0], [0, -1], [0, 1]];
  while (q.length) {
    const [x, y] = q.shift();
    for (const [dx, dy] of dirs) {
      const nx = x + dx, ny = y + dy;
      if (nx >= 0 && nx < R && ny >= 0 && ny < C && m[nx][ny] === target) {
        m[nx][ny] = color;
        q.push([nx, ny]);
      }
    }
  }
  return m;
}

const matrix = [
  ['B', 'B', 'W'],
  ['W', 'W', 'W'],
  ['W', 'W', 'W'],
  ['B', 'B', 'B'],
];
floodFill(matrix, 2, 2, 'G');
console.log(matrix.map((row) => row.join(' ')).join('\n'));
