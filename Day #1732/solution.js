// Bitwise AND of range [M,N] = common binary prefix; shift both right until equal, then back. O(log N) time, O(1) space.
function rangeBitwiseAnd(m, n) {
  let shift = 0;
  while (m < n) { m >>= 1; n >>= 1; shift++; }
  return m << shift;
}

console.log(rangeBitwiseAnd(5, 7));
