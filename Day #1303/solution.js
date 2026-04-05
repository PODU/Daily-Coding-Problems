// Day 1303: Next larger integer with the same number of set bits (snoob).
// Bit-hack: add lowest set bit, then re-insert the moved bits at the bottom. O(1) time.
function nextSameBits(n) {
  n = BigInt(n);
  const c = n & -n;                 // lowest set bit
  const r = n + c;                  // ripple carry
  const ones = ((n ^ r) >> 2n) / c; // moved bits, shifted down
  return r | ones;
}

console.log(nextSameBits(6).toString()); // 9
