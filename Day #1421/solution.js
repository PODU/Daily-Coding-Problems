// Day 1421: product of all elements except self, without division.
// Approach: prefix products pass then suffix products pass. O(n) time, O(1) extra (besides output).

function productExceptSelf(nums) {
  const n = nums.length;
  const res = new Array(n).fill(1);
  let prefix = 1;
  for (let i = 0; i < n; i++) {
    res[i] = prefix;
    prefix *= nums[i];
  }
  let suffix = 1;
  for (let i = n - 1; i >= 0; i--) {
    res[i] *= suffix;
    suffix *= nums[i];
  }
  return res;
}

console.log(productExceptSelf([1, 2, 3, 4, 5])); // [120, 60, 40, 30, 24]
console.log(productExceptSelf([3, 2, 1]));       // [2, 3, 6]
