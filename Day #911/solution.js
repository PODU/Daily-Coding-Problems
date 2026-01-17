// Reservoir sampling (size 1): replace pick with prob 1/i for i-th element. O(n) time, O(1) space.
'use strict';

function reservoirSample(stream) {
  let pick = null;
  let i = 0;
  for (const x of stream) {
    i++; // 1-indexed
    if (Math.floor(Math.random() * i) === 0) pick = x; // prob 1/i
  }
  return pick;
}

function main() {
  const n = 10;
  const stream = Array.from({ length: n }, (_, k) => k); // 0..9

  console.log('Sampled element:', reservoirSample(stream));

  const trials = 100000;
  const freq = new Array(n).fill(0);
  for (let t = 0; t < trials; t++) freq[reservoirSample(stream)]++;
  console.log(`Approx frequencies over ${trials} trials (expect ~${(1 / n).toFixed(4)} each):`);
  for (let v = 0; v < n; v++) {
    console.log(`  ${v}: ${(freq[v] / trials).toFixed(4)}`);
  }
  console.log('Distribution is ~uniform.');
}

main();
