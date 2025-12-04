// Day 693: Swap adjacent (even/odd) bits of an unsigned 8-bit integer.
// Approach: one-liner masks. Odd bits 0x55 shift left, even bits 0xAA shift right.
// Time O(1), Space O(1).
function swapBits(n) {
  return (((n & 0xAA) >> 1) | ((n & 0x55) << 1)) & 0xFF;
}

const bin8 = (n) => n.toString(2).padStart(8, "0");

console.log(bin8(swapBits(0b10101010))); // 01010101
console.log(bin8(swapBits(0b11100010))); // 11010001
