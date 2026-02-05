// Day 1022: Single number where all others appear 3 times.
// Approach: ones/twos bitmask accumulators. Time O(N), Space O(1).
function singleNumber(nums) {
  let ones = 0, twos = 0;
  for (const x of nums) {
    ones = (ones ^ x) & ~twos;
    twos = (twos ^ x) & ~ones;
  }
  return ones;
}

for (const t of [[6, 1, 3, 3, 3, 6, 6], [13, 19, 13, 13]]) {
  console.log(singleNumber(t));
}
