// Floor all; round up the `deficit` elements with largest fractional parts to match round(sum). O(n log n) time, O(n) space.
function roundToSum(x) {
  const n = x.length;
  const y = x.map((v) => Math.floor(v));
  const floorSum = y.reduce((a, b) => a + b, 0);
  const total = x.reduce((a, b) => a + b, 0);
  const target = Math.round(total);
  const deficit = target - floorSum;
  const order = x
    .map((v, i) => i)
    .sort((a, b) => (x[b] - Math.floor(x[b])) - (x[a] - Math.floor(x[a])));
  for (let k = 0; k < deficit; k++) y[order[k]]++;
  return y;
}

function main() {
  const x = [1.3, 2.3, 4.4];
  const y = roundToSum(x);
  console.log("[" + y.join(", ") + "]");
}

main();
