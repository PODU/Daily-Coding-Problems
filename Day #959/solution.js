// Day 959: total number of set bits over all integers in [1, N].
// Per-bit counting of full cycles plus remainder. Time O(log N), Space O(1).
// Uses BigInt for safety with large N.

function countSetBits(n) {
  n = BigInt(n);
  let total = 0n;
  for (let i = 0n; (1n << i) <= n; i++) {
    const cycle = 1n << (i + 1n);
    const half = cycle >> 1n;
    total += ((n + 1n) / cycle) * half;
    const rem = (n + 1n) % cycle;
    const extra = rem - half;
    if (extra > 0n) total += extra;
  }
  return total;
}

console.log(countSetBits(5).toString()); // 7
