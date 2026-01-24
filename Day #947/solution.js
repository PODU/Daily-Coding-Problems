// Day 947: smallest sparse number (no two adjacent set bits) >= N.
// Repeatedly fix the lowest adjacent-1 pair by rounding up. Time O(log N), Space O(1).
// Uses BigInt to stay safe for large N.

function smallestSparse(n) {
  n = BigInt(n);
  while ((n & (n >> 1n)) !== 0n) {
    const m = n & (n >> 1n);
    let p = 0n;
    let t = m;
    while ((t & 1n) === 0n) { t >>= 1n; p++; } // trailing zeros
    const step = 1n << (p + 1n);
    n = (n + step) & ~(step - 1n);
  }
  return n;
}

console.log(smallestSparse(21).toString()); // 21
console.log(smallestSparse(22).toString()); // 32
