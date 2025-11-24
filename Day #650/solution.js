// Per-row binary search: count elements < A[i1][j1] (lowerBound) plus elements >= A[i2][j2] (M - lowerBound).
// Upper bound taken inclusive (>=) to match reference example. Time O(N log M), space O(1).
function lowerBound(row, key) {
  let lo = 0, hi = row.length;
  while (lo < hi) {
    const mid = (lo + hi) >> 1;
    if (row[mid] < key) lo = mid + 1; else hi = mid;
  }
  return lo;
}

function main() {
  const A = [
    [1, 3, 7, 10, 15, 20],
    [2, 6, 9, 14, 22, 25],
    [3, 8, 10, 15, 25, 30],
    [10, 11, 12, 23, 30, 35],
    [20, 25, 30, 35, 40, 45],
  ];
  const i1 = 1, j1 = 1, i2 = 3, j2 = 3;
  const pivot1 = A[i1][j1]; // 6
  const pivot2 = A[i2][j2]; // 23
  const M = A[0].length;
  let countSmaller = 0, countUpper = 0;
  for (const row of A) {
    countSmaller += lowerBound(row, pivot1);
    countUpper += M - lowerBound(row, pivot2);
  }
  console.log(countSmaller + countUpper);
}

main();
