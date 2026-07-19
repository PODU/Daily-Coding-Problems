// Day 1843: Max of two numbers with no branching/comparison; uses sign bit of the difference.
// max(a,b) = a - ((a-b) & ((a-b) >> 63)) with BigInt for safe 64-bit shift. O(1).

function maxNoBranch(a, b) {
  const A = BigInt(a),
    B = BigInt(b);
  const d = A - B;
  // arithmetic right shift of a BigInt: 0n if d>=0, -1n if d<0
  const sign = d >> 63n;
  return Number(A - (d & sign));
}

function main() {
  console.log(maxNoBranch(5, 9)); // 9
  console.log(maxNoBranch(12, 4)); // 12
  console.log(maxNoBranch(-3, -7)); // -3
}

main();
