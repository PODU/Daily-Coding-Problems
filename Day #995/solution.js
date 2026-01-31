// Day 995: Product of array except self, without division.
// Left pass stores prefix products; right pass multiplies by suffix products.
// O(n) time, O(1) extra space (besides output).

function products(nums) {
  const n = nums.length;
  const res = new Array(n).fill(1);
  let left = 1;
  for (let i = 0; i < n; i++) {
    res[i] = left;
    left *= nums[i];
  }
  let right = 1;
  for (let i = n - 1; i >= 0; i--) {
    res[i] *= right;
    right *= nums[i];
  }
  return res;
}

console.log(JSON.stringify(products([1, 2, 3, 4, 5]))); // [120,60,40,30,24]
console.log(JSON.stringify(products([3, 2, 1]))); // [2,3,6]
