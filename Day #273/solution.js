// Day 273: Fixed point (arr[i]==i) in sorted distinct array via binary search.
// Time O(log N), Space O(1). Returns index or false.
function fixedPoint(a) {
  let lo = 0, hi = a.length - 1;
  while (lo <= hi) {
    const mid = lo + Math.floor((hi - lo) / 2);
    if (a[mid] === mid) return mid;
    else if (a[mid] < mid) lo = mid + 1;
    else hi = mid - 1;
  }
  return false;
}

console.log(fixedPoint([-6, 0, 2, 40])); // 2
console.log(fixedPoint([1, 5, 7, 8]));   // false
