// Product of array except self without division: prefix-product then suffix-product.
// Two passes, output array only. Time O(n), Space O(1) extra (besides output).

function productExceptSelf(nums) {
  const n = nums.length;
  const res = new Array(n).fill(1);
  for (let i = 1; i < n; i++) res[i] = res[i - 1] * nums[i - 1];
  let suffix = 1;
  for (let i = n - 1; i >= 0; i--) {
    res[i] *= suffix;
    suffix *= nums[i];
  }
  return res;
}

const nums = [1, 2, 3, 4, 5];
const res = productExceptSelf(nums);
console.log("[" + res.join(", ") + "]");
