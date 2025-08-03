// Spiral order print via boundary shrinking (top/bottom/left/right). Time O(N*M), Space O(1) extra.
function spiral(m) {
  let top = 0, bottom = m.length - 1, left = 0, right = m[0].length - 1;
  const out = [];
  while (top <= bottom && left <= right) {
    for (let j = left; j <= right; j++) out.push(m[top][j]);
    top++;
    for (let i = top; i <= bottom; i++) out.push(m[i][right]);
    right--;
    if (top <= bottom) {
      for (let j = right; j >= left; j--) out.push(m[bottom][j]);
      bottom--;
    }
    if (left <= right) {
      for (let i = bottom; i >= top; i--) out.push(m[i][left]);
      left++;
    }
  }
  return out;
}

const matrix = [
  [1, 2, 3, 4, 5],
  [6, 7, 8, 9, 10],
  [11, 12, 13, 14, 15],
  [16, 17, 18, 19, 20],
];
for (const v of spiral(matrix)) console.log(v);
