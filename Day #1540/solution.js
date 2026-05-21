// Largest product of any three integers.
// Sort once; answer is max of top-3 product and (two smallest * largest).
// Time O(n log n), Space O(1).
function largestProductOfThree(a) {
  a = [...a].sort((x, y) => x - y);
  const n = a.length;
  return Math.max(a[n - 1] * a[n - 2] * a[n - 3], a[0] * a[1] * a[n - 1]);
}

console.log(largestProductOfThree([-10, -10, 5, 2]));
