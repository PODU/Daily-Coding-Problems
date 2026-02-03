// Bitwise AND of all ints in [M,N] = common binary prefix of M and N.
// Shift both right until equal, then shift back. Time: O(log N), Space: O(1).

function rangeAnd(m, n) {
  let shift = 0;
  while (m < n) { m = Math.floor(m / 2); n = Math.floor(n / 2); shift++; }
  return m * Math.pow(2, shift);
}

console.log(`AND(5, 7) = ${rangeAnd(5, 7)}`);      // 4
console.log(`AND(12, 15) = ${rangeAnd(12, 15)}`);  // 12
