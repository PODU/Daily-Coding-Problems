// Next bigger integer with same popcount via bit-hack (Gosper's hack). O(1) time, O(1) space.
function nextSamePopcount(n) {
  if (n === 0) return 0;
  const c = n & -n;             // lowest set bit
  const r = n + c;              // ripple carry
  const ones = ((n ^ r) >>> 2) / c; // shifted-in ones
  return r | ones;
}

console.log(nextSamePopcount(6)); // 0110 -> 1001 = 9
