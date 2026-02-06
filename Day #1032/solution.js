// Day 1032: Check if a matrix is Toeplitz.
// Compare each element to its top-left neighbor: m[i][j]===m[i-1][j-1]. O(rows*cols) time, O(1) space.
function isToeplitz(m) {
  for (let i = 1; i < m.length; i++)
    for (let j = 1; j < m[i].length; j++)
      if (m[i][j] !== m[i - 1][j - 1]) return false;
  return true;
}

const matrix = [
  [1, 2, 3, 4, 8],
  [5, 1, 2, 3, 4],
  [4, 5, 1, 2, 3],
  [7, 4, 5, 1, 2],
];
console.log(isToeplitz(matrix) ? "True" : "False");
