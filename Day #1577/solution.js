// Day 1577: Smallest sparse number (no adjacent 1 bits) >= N.
// Repeatedly fix the lowest pair of adjacent ones by clearing low bits and carrying.
// Time: O((log N)^2); Space: O(1). Uses BigInt for safe bit ops.
"use strict";

function smallestSparse(n) {
  n = BigInt(n);
  while (n & (n >> 1n)) {
    let i = 0n;
    while (!(((n >> i) & 1n) && ((n >> (i + 1n)) & 1n))) i++;
    const mask = (1n << (i + 2n)) - 1n;
    n = (n & ~mask) + (1n << (i + 2n));
  }
  return n;
}

console.log(smallestSparse(21).toString()); // 21
