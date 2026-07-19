// Day 1839: Minimum of a rotated sorted array (no duplicates) via binary search.
// Time O(log N), Space O(1).

function findMin(a) {
  let lo = 0,
    hi = a.length - 1;
  while (lo < hi) {
    const mid = (lo + hi) >> 1;
    if (a[mid] > a[hi]) lo = mid + 1;
    else hi = mid;
  }
  return a[lo];
}

function main() {
  console.log(findMin([5, 7, 10, 3, 4]));
}

main();
