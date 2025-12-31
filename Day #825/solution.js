// Sorted squares via two-pointer merge from both ends, filling result from the back.
// Time: O(n), Space: O(n).

function sortedSquares(nums) {
  const n = nums.length;
  const res = new Array(n);
  let lo = 0, hi = n - 1;
  for (let k = n - 1; k >= 0; k--) {
    const l = nums[lo] * nums[lo];
    const r = nums[hi] * nums[hi];
    if (l > r) { res[k] = l; lo++; }
    else { res[k] = r; hi--; }
  }
  return res;
}

console.log("[" + sortedSquares([-9, -2, 0, 2, 3]).join(", ") + "]");
