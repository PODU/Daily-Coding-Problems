// Day 1429: Find a peak element (greater than both neighbors) in O(log N).
// Approach: binary search; if a[mid] < a[mid+1] a peak lies right, else left/at mid.
// Time: O(log n), Space: O(1).

function findPeak(a) {
  let lo = 0, hi = a.length - 1;
  while (lo < hi) {
    const mid = (lo + hi) >> 1;
    if (a[mid] < a[mid + 1]) lo = mid + 1;
    else hi = mid;
  }
  return lo;
}

const a = [1, 3, 5, 7, 6, 4, 2];
console.log(a[findPeak(a)]); // 7
