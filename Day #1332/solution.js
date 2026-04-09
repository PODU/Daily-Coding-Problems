// Day 1332: Round each x_i up or down so sum(Y)=round(sum X) while minimizing sum|x_i-y_i|.
// Floor everything, then round up the k elements with the largest fractional parts (k=target-sum of floors). O(n log n).

function roundPreserveSum(x) {
  const n = x.length;
  const target = Math.round(x.reduce((s, v) => s + v, 0));
  const floors = x.map(Math.floor);
  const base = floors.reduce((s, v) => s + v, 0);
  const k = target - base;
  const idx = [...Array(n).keys()].sort((a, b) => (x[b] - floors[b]) - (x[a] - floors[a]));
  const y = floors.slice();
  for (let i = 0; i < k; i++) y[idx[i]]++;
  return y;
}

console.log(roundPreserveSum([1.3, 2.3, 4.4])); // [ 1, 2, 5 ]
