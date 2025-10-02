// Round floats to ints keeping sum == round(total) with minimal total abs diff.
// Floor all; round up (T-F) elements with largest fractional parts (cheapest to push up). O(N log N) time, O(N) space.
"use strict";

function roundPreservingSum(x) {
  const n = x.length;
  const y = x.map((v) => Math.floor(v));
  const floorSum = y.reduce((a, b) => a + b, 0);
  const total = x.reduce((a, b) => a + b, 0);
  const target = Math.round(total);
  let need = target - floorSum;

  const idx = Array.from({ length: n }, (_, i) => i);
  idx.sort((a, b) => (x[b] - Math.floor(x[b])) - (x[a] - Math.floor(x[a])));

  for (const i of idx) {
    if (need <= 0) break;
    if (x[i] - Math.floor(x[i]) > 0) { y[i] += 1; need -= 1; }
  }
  return y;
}

function main() {
  const x = [1.3, 2.3, 4.4];
  const y = roundPreservingSum(x);
  console.log("[" + y.join(", ") + "]");
}

main();
