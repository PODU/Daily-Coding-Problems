// Total set bits from 1..N. Per-bit counting: O(log N) time, O(1) space.
// For each bit i: full cycles contribute 2^i ones each, plus remainder.
function countSetBits(n) {
  let total = 0n;
  n = BigInt(n);
  for (let i = 0n; (1n << i) <= n; i++) {
    const block = 1n << (i + 1n);
    let ones = ((n + 1n) / block) * (1n << i);
    const rem = ((n + 1n) % block) - (1n << i);
    if (rem > 0n) ones += rem;
    total += ones;
  }
  return total;
}

console.log("N=5  -> " + countSetBits(5));   // 7
console.log("N=16 -> " + countSetBits(16));  // 33
