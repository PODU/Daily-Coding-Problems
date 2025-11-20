// Day 636: Minimum in a rotated sorted array (no duplicates).
// Approach: binary search comparing mid with right endpoint.
// Time: O(log N), Space: O(1).
function findMin(a) {
  let lo = 0, hi = a.length - 1;
  while (lo < hi) {
    const mid = (lo + hi) >> 1;
    if (a[mid] > a[hi]) lo = mid + 1;
    else hi = mid;
  }
  return a[lo];
}

console.log(findMin([5, 7, 10, 3, 4])); // 3
