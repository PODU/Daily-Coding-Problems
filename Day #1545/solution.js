// Pyramid min cost (only lowering). For each index the max pyramid height is
// min of a left pass (rise +1 from edge) and a right pass. A pyramid of peak h
// retains h*h stones, so cost = sum(a) - max(h)^2. Time O(n), Space O(n).
function minCost(a) {
  const n = a.length;
  const left = new Array(n), right = new Array(n);
  left[0] = Math.min(a[0], 1);
  for (let i = 1; i < n; i++) left[i] = Math.min(a[i], left[i - 1] + 1);
  right[n - 1] = Math.min(a[n - 1], 1);
  for (let i = n - 2; i >= 0; i--) right[i] = Math.min(a[i], right[i + 1] + 1);
  let bestPeak = 0, sum = 0;
  for (let i = 0; i < n; i++) {
    bestPeak = Math.max(bestPeak, Math.min(left[i], right[i]));
    sum += a[i];
  }
  return sum - bestPeak * bestPeak;
}

console.log(minCost([1, 1, 3, 3, 2, 1]));
