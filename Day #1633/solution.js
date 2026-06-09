// Branchless select: mask = -b (all-ones if b=1 else 0); result = (x & mask) | (y & ~mask). O(1) time/space.
'use strict';

function select(b, x, y) {
  const mask = -b | 0; // 0xFFFFFFFF if b=1, 0 if b=0 (32-bit)
  return (x & mask) | (y & ~mask);
}

function main() {
  console.log(select(1, 42, 99));
  console.log(select(0, 42, 99));
}

main();
