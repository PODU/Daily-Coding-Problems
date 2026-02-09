// Search in rotated sorted array via modified binary search.
// Time: O(log n), Space: O(1).
function searchRotated(a, target) {
  let lo = 0, hi = a.length - 1;
  while (lo <= hi) {
    const mid = (lo + hi) >> 1;
    if (a[mid] === target) return mid;
    if (a[lo] <= a[mid]) {
      if (a[lo] <= target && target < a[mid]) hi = mid - 1;
      else lo = mid + 1;
    } else {
      if (a[mid] < target && target <= a[hi]) lo = mid + 1;
      else hi = mid - 1;
    }
  }
  return null;
}

const arr = [13, 18, 25, 2, 8, 10];
console.log(searchRotated(arr, 8)); // 4
