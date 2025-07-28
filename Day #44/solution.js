// Count Inversions via modified merge sort: count cross-pairs while merging.
// Time O(n log n), Space O(n).
function countInversions(src) {
  const a = src.slice();
  const tmp = new Array(a.length);

  function rec(lo, hi) {
    if (hi - lo <= 1) return 0;
    const mid = (lo + hi) >> 1;
    let inv = rec(lo, mid) + rec(mid, hi);
    let i = lo, j = mid, k = lo;
    while (i < mid && j < hi) {
      if (a[i] <= a[j]) tmp[k++] = a[i++];
      else { tmp[k++] = a[j++]; inv += mid - i; }
    }
    while (i < mid) tmp[k++] = a[i++];
    while (j < hi) tmp[k++] = a[j++];
    for (let x = lo; x < hi; x++) a[x] = tmp[x];
    return inv;
  }
  return rec(0, a.length);
}

console.log(countInversions([2, 4, 1, 3, 5]));
console.log(countInversions([5, 4, 3, 2, 1]));
