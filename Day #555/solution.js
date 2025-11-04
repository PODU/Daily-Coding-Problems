// Search in rotated sorted array via modified binary search. O(log n) time, O(1) space.
function search(a, target) {
  let lo = 0, hi = a.length - 1;
  while (lo <= hi) {
    const mid = (lo + hi) >> 1;
    if (a[mid] === target) return mid;
    if (a[lo] <= a[mid]) { // left half sorted
      if (a[lo] <= target && target < a[mid]) hi = mid - 1;
      else lo = mid + 1;
    } else { // right half sorted
      if (a[mid] < target && target <= a[hi]) lo = mid + 1;
      else hi = mid - 1;
    }
  }
  return null;
}

const arr = [13, 18, 25, 2, 8, 10];
console.log(search(arr, 8));
const miss = search(arr, 7);
console.log(miss === null ? "null" : miss);
