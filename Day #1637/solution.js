// Binary search without *, /, or bit-shift; midpoint via two-pointer walk (only ++/--).
// Time: O(log N), Space: O(1).

function midpoint(lo, hi) {
  let i = lo, j = hi;
  while (i < j) { i++; j--; }
  return j; // floor((lo+hi)/2) using only ++/--
}

function contains(arr, x) {
  let lo = 0, hi = arr.length - 1;
  while (lo <= hi) {
    const mid = midpoint(lo, hi);
    if (arr[mid] === x) return true;
    else if (arr[mid] < x) lo = mid + 1;
    else hi = mid - 1;
  }
  return false;
}

function main() {
  const arr = [1, 3, 5, 7, 9, 11, 13];
  console.log(contains(arr, 7) ? "true" : "false");
  console.log(contains(arr, 8) ? "true" : "false");
}

main();
