// Count total set bits in 1..N using per-bit cycle formula. O(log N) time, O(1) space.
function countSetBits(n) {
  let total = 0;
  for (let p = 2; p <= 2 * n; p <<= 1) {
    const full = Math.floor((n + 1) / p) * (p / 2);
    const rem = Math.max(0, ((n + 1) % p) - p / 2);
    total += full + rem;
  }
  return total;
}

console.log(countSetBits(5));
