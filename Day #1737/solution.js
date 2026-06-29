// XOR all -> a^b; partition by a set bit, XOR each group to recover the two singletons. O(n) time, O(1) space.
"use strict";

function twoUnique(nums) {
  let xorAll = 0;
  for (const x of nums) xorAll ^= x;
  const bit = xorAll & -xorAll; // lowest set bit
  let a = 0, b = 0;
  for (const x of nums) {
    if (x & bit) a ^= x;
    else b ^= x;
  }
  return a < b ? [a, b] : [b, a];
}

function main() {
  const nums = [2, 4, 6, 8, 10, 2, 6, 10];
  const [a, b] = twoUnique(nums);
  console.log(`${a} and ${b}`);
}

main();
