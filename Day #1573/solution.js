// Find a peak in a rotated array (ends lowest) via binary search. O(log N) time, O(1) space.

function findPeak(a) {
  let lo = 0, hi = a.length - 1;
  while (lo < hi) {
    const mid = (lo + hi) >> 1;
    if (a[mid] < a[mid + 1]) lo = mid + 1;
    else hi = mid;
  }
  return a[lo];
}

const arr = [5, 7, 9, 3, 1];
console.log(findPeak(arr));
