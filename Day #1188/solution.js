// Count inversions via merge sort: count cross-pairs while merging. Time O(N log N), Space O(N).
'use strict';

function countInversions(arr) {
  function sortCount(a) {
    if (a.length <= 1) return [a, 0];
    const mid = a.length >> 1;
    const [left, lInv] = sortCount(a.slice(0, mid));
    const [right, rInv] = sortCount(a.slice(mid));
    const merged = [];
    let i = 0, j = 0, inv = lInv + rInv;
    while (i < left.length && j < right.length) {
      if (left[i] <= right[j]) merged.push(left[i++]);
      else { merged.push(right[j++]); inv += left.length - i; }
    }
    while (i < left.length) merged.push(left[i++]);
    while (j < right.length) merged.push(right[j++]);
    return [merged, inv];
  }
  return sortCount(arr)[1];
}

function main() {
  const arr = [2, 4, 1, 3, 5];
  console.log(countInversions(arr));
}

main();
