// Reservoir sampling (reservoir size 1): for the i-th element replace kept with prob 1/i.
// Distribution is uniform over the stream. Time O(n), Space O(1). Seeded RNG -> deterministic.
'use strict';

// Mulberry32: tiny deterministic seeded PRNG.
function mulberry32(seed) {
  let a = seed >>> 0;
  return function () {
    a |= 0;
    a = (a + 0x6d2b79f5) | 0;
    let t = Math.imul(a ^ (a >>> 15), 1 | a);
    t = (t + Math.imul(t ^ (t >>> 7), 61 | t)) ^ t;
    return ((t ^ (t >>> 14)) >>> 0) / 4294967296;
  };
}

function reservoirSample(stream, seed) {
  const rng = mulberry32(seed);
  let kept = null;
  for (let i = 0; i < stream.length; i++) {
    // for the (i+1)-th element keep with prob 1/(i+1)
    if (Math.floor(rng() * (i + 1)) === 0) kept = stream[i];
  }
  return kept;
}

function main() {
  const stream = [];
  for (let v = 1; v <= 10; v++) stream.push(v);
  const selected = reservoirSample(stream, 42);
  console.log(`Selected: ${selected}`);
}

main();
