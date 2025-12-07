// Day 708: Fixed point (a[i]==i) in a sorted distinct array via binary search.
// Since values are distinct integers, a[i]-i is monotonic. Time O(log n).
function fixedPoint(a) {
  let lo = 0, hi = a.length - 1;
  while (lo <= hi) {
    const mid = (lo + hi) >> 1;
    if (a[mid] === mid) return mid;
    else if (a[mid] < mid) lo = mid + 1;
    else hi = mid - 1;
  }
  return false;
}

function report(a) {
  const r = fixedPoint(a);
  console.log(r === false ? "False" : r);
}

report([-6, 0, 2, 40]);
report([1, 5, 7, 8]);
