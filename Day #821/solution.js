// Fixed point in sorted distinct array via binary search (nums[i]-i non-decreasing).
// Time: O(log n), Space: O(1).

function fixedPoint(nums) {
  let lo = 0, hi = nums.length - 1;
  while (lo <= hi) {
    const mid = (lo + hi) >> 1;
    if (nums[mid] === mid) return mid;
    else if (nums[mid] < mid) lo = mid + 1;
    else hi = mid - 1;
  }
  return false;
}

function show(r) {
  return r === false ? "False" : String(r);
}
console.log(show(fixedPoint([-6, 0, 2, 40])));
console.log(show(fixedPoint([1, 5, 7, 8])));
