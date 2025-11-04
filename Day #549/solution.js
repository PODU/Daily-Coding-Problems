// Single number among triples: sum each bit position mod 3 to rebuild the unique value. O(N) time, O(1) space.
'use strict';

function singleNumber(nums) {
  let result = 0;
  for (let b = 0; b < 32; b++) {
    let cnt = 0;
    for (const x of nums) cnt += (x >> b) & 1;
    if (cnt % 3) result |= (1 << b);
  }
  return result;
}

console.log(singleNumber([6, 1, 3, 3, 3, 6, 6]));
console.log(singleNumber([13, 19, 13, 13]));
