// Day 195: In a row- and column-sorted matrix, count elements < M[i1,j1] or > M[i2,j2].
// Staircase counting of "<= x". Time O(N+M) per query, Space O(1).
// Note: the README example counts the lower bound inclusively (expected 15), so we use
// "<= M[i1,j1]" for the smaller side and strict "> M[i2,j2]" for the larger side.
function countLessEqual(A, x) {
  const n = A.length, m = A[0].length;
  let r = 0, c = m - 1, cnt = 0;
  while (r < n && c >= 0) {
    if (A[r][c] <= x) { cnt += c + 1; r++; }
    else c--;
  }
  return cnt;
}

function solve(A, i1, j1, i2, j2) {
  const n = A.length, m = A[0].length;
  const smaller = countLessEqual(A, A[i1][j1]);
  const larger = n * m - countLessEqual(A, A[i2][j2]);
  return smaller + larger;
}

const A = [
  [1, 3, 7, 10, 15, 20],
  [2, 6, 9, 14, 22, 25],
  [3, 8, 10, 15, 25, 30],
  [10, 11, 12, 23, 30, 35],
  [20, 25, 30, 35, 40, 45],
];
console.log(solve(A, 1, 1, 3, 3));
