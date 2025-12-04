// Day 691: Bitwise AND of all integers in [M, N].
// Approach: result is the common binary prefix of M and N. Shift both right until
// equal, then shift back. Time O(log N), Space O(1).
function rangeBitwiseAnd(m, n) {
  let shift = 0;
  while (m < n) { m >>= 1; n >>= 1; shift++; }
  return m << shift;
}

console.log(rangeBitwiseAnd(5, 7));    // 4
console.log(rangeBitwiseAnd(12, 15));  // 12
