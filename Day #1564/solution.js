// Staircase count from top-right in O(N+M): smaller = count(<low); larger = N*M - count(<high). Time O(N+M), Space O(1).
function countLess(M, x) {
  // Number of elements strictly less than x in a row/col sorted matrix.
  const n = M.length, m = M[0].length;
  let cnt = 0, r = 0, c = m - 1;
  while (r < n && c >= 0) {
    if (M[r][c] < x) {
      cnt += c + 1;
      r++;
    } else {
      c--;
    }
  }
  return cnt;
}

function main() {
  const M = [
    [1, 3, 7, 10, 15, 20],
    [2, 6, 9, 14, 22, 25],
    [3, 8, 10, 15, 25, 30],
    [10, 11, 12, 23, 30, 35],
    [20, 25, 30, 35, 40, 45],
  ];
  const low = M[1][1];   // 6
  const high = M[3][3];  // 23
  const total = M.length * M[0].length;
  const smaller = countLess(M, low);          // elements < 6
  const larger = total - countLess(M, high);  // elements >= 23
  console.log(smaller + larger);
}

main();
