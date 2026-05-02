// Fixed point (arr[i]==i) in sorted distinct array via binary search.
// Time O(log n), Space O(1).
'use strict';

// Returns index i where arr[i]===i, or false if none.
function fixedPoint(arr) {
  let lo = 0, hi = arr.length - 1;
  while (lo <= hi) {
    const mid = (lo + hi) >> 1;
    if (arr[mid] === mid) return mid;
    else if (arr[mid] < mid) lo = mid + 1;
    else hi = mid - 1;
  }
  return false;
}

function run(arr) {
  const r = fixedPoint(arr);
  console.log(r === false ? 'False' : String(r));
}

run([-6, 0, 2, 40]);
run([1, 5, 7, 8]);
