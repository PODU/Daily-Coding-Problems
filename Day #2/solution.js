// Product of array except self via prefix and suffix passes (no division).
// Time: O(n), Space: O(1) extra (excluding output).
function productExceptSelf(nums) {
  const n = nums.length;
  const res = new Array(n).fill(1);
  let pre = 1;
  for (let i = 0; i < n; i++) { res[i] = pre; pre *= nums[i]; }
  let suf = 1;
  for (let i = n - 1; i >= 0; i--) { res[i] *= suf; suf *= nums[i]; }
  return res;
}

console.log(productExceptSelf([1, 2, 3, 4, 5]));
