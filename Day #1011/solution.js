// Branchless max: 64-bit BigInt diff avoids overflow; sign bit via arithmetic
// right shift selects the larger. No if/branch/compare. Time O(1), Space O(1).
function maxNoBranch(a, b) {
  const A = BigInt(a), B = BigInt(b);
  const diff = A - B;                 // no overflow with BigInt
  const sign = diff >> 63n;           // -1n if diff<0 else 0n
  return Number(A - (diff & sign));   // a>=b -> a ; a<b -> b
}

function main() {
  console.log(`max(3, 7) = ${maxNoBranch(3, 7)}`);
  console.log(`max(10, -5) = ${maxNoBranch(10, -5)}`);
}

main();
