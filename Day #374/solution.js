// Day 374: Lowest index i with arr[i]==i in a sorted, distinct-int array.
// f(i)=arr[i]-i is non-decreasing, so binary-search the leftmost i with
// f(i)>=0 and check equality. Time O(log n), Space O(1).
'use strict';

function fixedPoint(arr) {
  let lo = 0, hi = arr.length - 1, ans = -1;
  while (lo <= hi) {
    const mid = (lo + hi) >> 1;
    if (arr[mid] >= mid) { ans = mid; hi = mid - 1; }
    else lo = mid + 1;
  }
  return ans !== -1 && arr[ans] === ans ? ans : null;
}

console.log(fixedPoint([-5, -3, 2, 3])); // 2
