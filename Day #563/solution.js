// Max of two numbers without if-else/branch/comparison via bit manipulation.
// Use BigInt 64-bit: d=a-b; mask = d>>63 (arithmetic); max = a - (d & mask). Time O(1), Space O(1).
function maxNoBranch(a, b) {
  const A = BigInt.asIntN(64, BigInt(a));
  const B = BigInt.asIntN(64, BigInt(b));
  const d = BigInt.asIntN(64, A - B);
  const mask = BigInt.asIntN(64, d >> 63n); // all 1s if d<0, else 0
  return BigInt.asIntN(64, A - (d & mask));
}

console.log(maxNoBranch(3, 7).toString());
console.log(maxNoBranch(10, -4).toString());
