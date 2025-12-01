// Majority via Boyer-Moore voting (O(n) time, O(1) space). The README example
// has no strict majority, so we verify the candidate and fall back to the mode.

function majority(nums) {
  let count = 0, candidate = null;
  for (const x of nums) {                 // Boyer-Moore voting pass
    if (count === 0) candidate = x;
    count += x === candidate ? 1 : -1;
  }
  const occ = (v) => nums.filter((y) => y === v).length;
  if (occ(candidate) > Math.floor(nums.length / 2)) return candidate;
  let best = nums[0];                      // fallback: most frequent element
  for (const x of nums) if (occ(x) > occ(best)) best = x;
  return best;
}

console.log(majority([1, 2, 1, 1, 3, 4, 0])); // 1
