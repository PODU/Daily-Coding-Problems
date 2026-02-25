// Day 1125 - Contiguous sublist summing to K
// Prefix sums + hash map (handles negatives) to find a range with sum == K in
// one pass. Time: O(n), Space: O(n).

function subarraySum(nums, k) {
  const seen = new Map([[0, -1]]); // prefix sum -> earliest index
  let total = 0;
  for (let j = 0; j < nums.length; j++) {
    total += nums[j];
    if (seen.has(total - k)) {
      const i = seen.get(total - k);
      return nums.slice(i + 1, j + 1);
    }
    if (!seen.has(total)) seen.set(total, j);
  }
  return null;
}

console.log(subarraySum([1, 2, 3, 4, 5], 9)); // [2, 3, 4]
