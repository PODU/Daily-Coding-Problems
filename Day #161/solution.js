// Day 161: Reverse the 32 bits of an integer by shifting bits out of the input
// into the result. Time O(32), Space O(1).
'use strict';

function reverseBits(n) {
  let res = 0;
  for (let i = 0; i < 32; i++) {
    res = (res << 1) | (n & 1);
    n >>>= 1;
  }
  return res >>> 0; // treat as unsigned
}

function toGroups(n) {
  const bits = (n >>> 0).toString(2).padStart(32, '0');
  return bits.match(/.{4}/g).join(' ');
}

const n = 0xf0f0f0f0; // 1111 0000 ... repeated
console.log(toGroups(reverseBits(n)));
