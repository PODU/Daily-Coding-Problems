// Word search L-to-R / U-to-D only: scan each row and column for target substring.
// Time O(R*C*L), Space O(max(R,C)).

function findWord(matrix, target) {
  const R = matrix.length;
  const C = R ? matrix[0].length : 0;
  for (let r = 0; r < R; r++) {
    if (matrix[r].join("").includes(target)) return true;
  }
  for (let c = 0; c < C; c++) {
    let col = "";
    for (let r = 0; r < R; r++) col += matrix[r][c];
    if (col.includes(target)) return true;
  }
  return false;
}

const matrix = [
  ['F', 'A', 'C', 'I'],
  ['O', 'B', 'Q', 'P'],
  ['A', 'N', 'O', 'B'],
  ['M', 'A', 'S', 'S'],
];
console.log(findWord(matrix, "FOAM"));
