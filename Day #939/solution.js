// Day 939: Print an N x M matrix in clockwise spiral order.
// Shrink four boundaries layer by layer. Time O(N*M), Space O(1) extra.
function spiral(m) {
  if (m.length === 0) return;
  let top = 0, bottom = m.length - 1, left = 0, right = m[0].length - 1;
  while (top <= bottom && left <= right) {
    for (let c = left; c <= right; c++) console.log(m[top][c]);
    top++;
    for (let r = top; r <= bottom; r++) console.log(m[r][right]);
    right--;
    if (top <= bottom) {
      for (let c = right; c >= left; c--) console.log(m[bottom][c]);
      bottom--;
    }
    if (left <= right) {
      for (let r = bottom; r >= top; r--) console.log(m[r][left]);
      left++;
    }
  }
}

const matrix = [
  [1, 2, 3, 4, 5],
  [6, 7, 8, 9, 10],
  [11, 12, 13, 14, 15],
  [16, 17, 18, 19, 20],
];
spiral(matrix); // 1 2 3 4 5 10 15 20 19 18 17 16 11 6 7 8 9 14 13 12
