// Bitwise AND of all integers in [M, N] = common binary prefix of M and N.
// Shift both right until equal, then shift back. Time O(log N), Space O(1).
'use strict';

function rangeAnd(m, n) {
  let shift = 0;
  while (m < n) { m = Math.floor(m / 2); n = Math.floor(n / 2); shift++; }
  return m * Math.pow(2, shift);
}

console.log(rangeAnd(5, 7)); // 4  (5 & 6 & 7)
