// Day 102: Contiguous subarray summing to K via prefix sums + hashmap. For each
// prefix p look for p-K seen earlier; earliest-ending match. O(n) time.
function subarraySum(nums, k) {
  const first = new Map([[0, -1]]);
  let prefix = 0;
  for (let j = 0; j < nums.length; j++) {
    prefix += nums[j];
    if (first.has(prefix - k)) {
      return nums.slice(first.get(prefix - k) + 1, j + 1);
    }
    if (!first.has(prefix)) first.set(prefix, j);
  }
  return null;
}

console.log(subarraySum([1, 2, 3, 4, 5], 9)); // [ 2, 3, 4 ]
