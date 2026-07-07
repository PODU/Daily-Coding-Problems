// Swap even/odd bits of 8-bit int: ((n&0xAA)>>1)|((n&0x55)<<1), masked to 8 bits.
// Time: O(1), Space: O(1).
function swapBits(binStr) {
  const n = parseInt(binStr, 2);
  const r = (((n & 0xAA) >> 1) | ((n & 0x55) << 1)) & 0xFF;
  return r.toString(2).padStart(8, "0");
}

console.log(swapBits("10101010"));
console.log(swapBits("11100010"));
