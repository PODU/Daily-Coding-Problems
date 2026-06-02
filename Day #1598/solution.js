// Round floats preserving sum: floor all, then round up the d elements with
// largest fractional parts (d = round(sum) - sum(floors)). Time O(n log n).
"use strict";

function roundPreserve(x) {
  const n = x.length;
  const y = x.map((v) => Math.floor(v));
  const floorSum = y.reduce((a, b) => a + b, 0);
  const sum = x.reduce((a, b) => a + b, 0);
  const target = Math.round(sum);
  const d = target - floorSum;
  const order = [...Array(n).keys()].sort(
    (a, b) => (x[b] - Math.floor(x[b])) - (x[a] - Math.floor(x[a]))
  );
  for (let i = 0; i < d; i++) y[order[i]] += 1;
  return y;
}

const x = [1.3, 2.3, 4.4];
const y = roundPreserve(x);
console.log("[" + y.join(", ") + "]");
let diff = 0;
for (let i = 0; i < x.length; i++) diff += Math.abs(x[i] - y[i]);
console.log("abs diff = " + diff.toFixed(1));
