// Day 310: Total set bits over 1..N. Per-bit counting. O(log N).
function countBits(N) {
  let total = 0n;
  const n = BigInt(N);
  for (let i = 0n; (1n << i) <= n; i++) {
    const blk = 1n << (i + 1n);
    const full = ((n + 1n) / blk) * (1n << i);
    let rem = (n + 1n) % blk - (1n << i);
    if (rem < 0n) rem = 0n;
    total += full + rem;
  }
  return total;
}
console.log(countBits(5).toString()); // 7
