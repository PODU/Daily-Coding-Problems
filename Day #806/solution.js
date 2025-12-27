// Day 806: In a row/col-sorted matrix, count elements < M[i1][j1] plus those
// >= M[i2][j2] (boundary inclusive on high side to match the sample's 15).
// Per row binary search. Time O(N log M), Space O(1).

function lowerBound(row, val) {
  let lo = 0, hi = row.length;
  while (lo < hi) {
    const mid = (lo + hi) >> 1;
    if (row[mid] < val) lo = mid + 1; else hi = mid;
  }
  return lo;
}

function countOutside(M, i1, j1, i2, j2) {
  const low = M[i1][j1], high = M[i2][j2];
  let total = 0;
  for (const row of M) {
    total += lowerBound(row, low);              // elements < low
    total += row.length - lowerBound(row, high); // elements >= high
  }
  return total;
}

const M = [
  [1, 3, 7, 10, 15, 20], [2, 6, 9, 14, 22, 25], [3, 8, 10, 15, 25, 30],
  [10, 11, 12, 23, 30, 35], [20, 25, 30, 35, 40, 45],
];
console.log(countOutside(M, 1, 1, 3, 3)); // 15
