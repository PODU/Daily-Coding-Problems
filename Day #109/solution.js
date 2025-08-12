// Day 109: Swap adjacent bit pairs: ((x&0xAA)>>1)|((x&0x55)<<1). O(1).
function swapBits(x) {
  return (((x & 0xaa) >> 1) | ((x & 0x55) << 1)) & 0xff;
}

const toBin = (x) => x.toString(2).padStart(8, "0");
console.log(toBin(swapBits(0b10101010))); // 01010101
console.log(toBin(swapBits(0b11100010))); // 11010001
