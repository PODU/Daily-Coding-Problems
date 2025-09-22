// Day 315: Toeplitz check - each cell equals its top-left neighbor. O(N*M) time, O(1) space.
function isToeplitz(m) {
  for (let i = 1; i < m.length; i++)
    for (let j = 1; j < m[i].length; j++)
      if (m[i][j] !== m[i - 1][j - 1]) return false;
  return true;
}
const m = [[1, 2, 3, 4, 8], [5, 1, 2, 3, 4], [4, 5, 1, 2, 3], [7, 4, 5, 1, 2]];
console.log(isToeplitz(m) ? "True" : "False"); // True
