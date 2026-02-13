// Day 1063: Find a peak in a rotated sorted array of distinct values.
// Approach: binary search; if a[mid] < a[mid+1] go right else left. Time O(log n), Space O(1).

function findPeak(a) {
  let lo = 0;
  let hi = a.length - 1;
  while (lo < hi) {
    const mid = (lo + hi) >> 1;
    if (a[mid] < a[mid + 1]) lo = mid + 1;
    else hi = mid;
  }
  return lo; // index of the peak
}

const a = [3, 4, 5, 1, 2];
const idx = findPeak(a);
console.log(a[idx]); // 5
