// Single number among triples: bitwise ones/twos accumulators isolate the unique value.
// Time: O(n). Space: O(1).
'use strict';

function singleNumber(nums) {
  let ones = 0, twos = 0;
  for (const x of nums) {
    ones = (ones ^ x) & ~twos;
    twos = (twos ^ x) & ~ones;
  }
  return ones; // bitwise ops in JS are 32-bit signed, negatives handled correctly
}

function main() {
  const a = [6, 1, 3, 3, 3, 6, 6];
  const b = [13, 19, 13, 13];
  console.log(singleNumber(a)); // 1
  console.log(singleNumber(b)); // 19
}

main();
