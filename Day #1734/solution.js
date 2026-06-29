// Binary search for fixed point (A[i]==i) in sorted distinct array; A[i]-i non-decreasing.
// Time: O(log n), Space: O(1).
function fixedPoint(a) {
  let lo = 0, hi = a.length - 1, ans = -1;
  while (lo <= hi) {
    const mid = (lo + hi) >> 1;
    if (a[mid] === mid) { ans = mid; hi = mid - 1; }
    else if (a[mid] < mid) lo = mid + 1;
    else hi = mid - 1;
  }
  return ans === -1 ? false : ans;
}

function show(r) { return r === false ? "False" : r; }
console.log(show(fixedPoint([-6, 0, 2, 40])));
console.log(show(fixedPoint([1, 5, 7, 8])));
