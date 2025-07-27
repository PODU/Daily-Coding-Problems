// Single number where others appear 3x: ones/twos bit-counting state machine.
// O(N) time, O(1) space. Bitwise ops are 32-bit in JS, so negatives work.
function singleNumber(nums) {
  let ones = 0, twos = 0;
  for (const x of nums) {
    ones = (ones ^ x) & ~twos;
    twos = (twos ^ x) & ~ones;
  }
  return ones;
}

console.log(singleNumber([6, 1, 3, 3, 3, 6, 6]));
