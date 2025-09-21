// Day 311: Find a peak (greater than both neighbors) when ends are the lowest.
// Binary search toward the rising side. O(log N).
function findPeak(a) {
  let lo = 0, hi = a.length - 1;
  while (lo < hi) {
    const mid = (lo + hi) >> 1;
    if (a[mid] < a[mid + 1]) lo = mid + 1; else hi = mid;
  }
  return lo;
}
const a = [1, 3, 5, 7, 6, 4, 2];
const p = findPeak(a);
console.log(`index ${p} value ${a[p]}`); // index 3 value 7
