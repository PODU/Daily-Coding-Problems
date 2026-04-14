// Reservoir sampling (size 1): i-th element (1-indexed) replaces pick with prob 1/i. O(1) space.
// Demo uses a portable 64-bit LCG (BigInt) seeded with 1 so output is deterministic -> 7.

const MASK = (1n << 64n) - 1n;
const A = 6364136223846793005n;
const C = 1442695040888963407n;

function reservoirPick(stream, seed) {
  let state = seed & MASK;
  let pick = null;
  for (let i = 1; i <= stream.length; i++) {
    state = (state * A + C) & MASK;        // advance LCG (mod 2^64)
    if (state % BigInt(i) === 0n)          // replace with probability 1/i
      pick = stream[i - 1];
  }
  return pick;
}

const stream = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
console.log("Selected: " + reservoirPick(stream, 1n)); // fixed seed = 1
