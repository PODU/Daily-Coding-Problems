// Day 410: Round floats so rounded sum is preserved with minimal total abs error.
// Floor all, then round up the R = round(sum)-sum(floors) elements with largest fractions. O(n log n) time, O(n) space.
function roundToSum(x) {
  const n = x.length;
  const y = x.map((v) => Math.floor(v));
  const floorSum = y.reduce((a, b) => a + b, 0);
  const total = x.reduce((a, b) => a + b, 0);
  const target = Math.round(total);
  const R = target - floorSum;
  const idx = [...Array(n).keys()].sort(
    (a, b) => (x[b] - Math.floor(x[b])) - (x[a] - Math.floor(x[a]))
  );
  for (let i = 0; i < Math.min(R, n); i++) y[idx[i]] += 1;
  return y;
}

const x = [1.3, 2.3, 4.4];
const y = roundToSum(x);
console.log("[" + y.join(", ") + "]");
