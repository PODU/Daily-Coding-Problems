// Island perimeter: +4 per land cell, -2 per adjacent right/down land pair. O(R*C) time, O(1) space.
function islandPerimeter(g) {
  const R = g.length, C = R ? g[0].length : 0;
  let per = 0;
  for (let r = 0; r < R; r++)
    for (let c = 0; c < C; c++)
      if (g[r][c] === 1) {
        per += 4;
        if (c + 1 < C && g[r][c + 1] === 1) per -= 2;
        if (r + 1 < R && g[r + 1][c] === 1) per -= 2;
      }
  return per;
}

const grid = [[0, 1, 1, 0], [1, 1, 1, 0], [0, 1, 1, 0], [0, 0, 1, 0]];
console.log(islandPerimeter(grid));
