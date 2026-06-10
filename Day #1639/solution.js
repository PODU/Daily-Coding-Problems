// Count inversions using modified merge sort (count cross-pairs during merge).
// Time: O(N log N), Space: O(N).

function mergeCount(a, tmp, lo, hi) {
  if (hi - lo <= 1) return 0;
  const mid = lo + Math.floor((hi - lo) / 2);
  let inv = mergeCount(a, tmp, lo, mid) + mergeCount(a, tmp, mid, hi);
  let i = lo, j = mid, k = lo;
  while (i < mid && j < hi) {
    if (a[i] <= a[j]) tmp[k++] = a[i++];
    else { tmp[k++] = a[j++]; inv += mid - i; }
  }
  while (i < mid) tmp[k++] = a[i++];
  while (j < hi) tmp[k++] = a[j++];
  for (let t = lo; t < hi; t++) a[t] = tmp[t];
  return inv;
}

function countInversions(arr) {
  const a = arr.slice();
  const tmp = new Array(a.length).fill(0);
  return mergeCount(a, tmp, 0, a.length);
}

function main() {
  console.log("[2, 4, 1, 3, 5] has " + countInversions([2, 4, 1, 3, 5]) + " inversions");
  console.log("[5, 4, 3, 2, 1] has " + countInversions([5, 4, 3, 2, 1]) + " inversions");
}

main();
