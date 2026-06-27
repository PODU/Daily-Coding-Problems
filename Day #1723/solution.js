// Day 1723: Search element in rotated sorted array.
// Modified binary search: one half is always sorted; decide which side to recurse.
// Time: O(log n), Space: O(1). Returns index, or null if absent.
'use strict';

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

function main() {
  const a = [13, 18, 25, 2, 8, 10];
  console.log(search(a, 8));    // 4
  console.log(search(a, 99));   // null (not found)
}

main();
