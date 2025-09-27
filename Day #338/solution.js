// Next higher integer with same number of set bits (snoob bit-twiddle).
// O(1) time, O(1) space.
'use strict';

function nextSameBits(n) {
  const smallest = n & (-n);
  const ripple = n + smallest;
  let ones = n ^ ripple;
  ones = (ones >>> 2) / smallest;
  return ripple | ones;
}

console.log(nextSameBits(6)); // 9
