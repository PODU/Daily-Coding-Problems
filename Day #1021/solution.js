// Day 1021: Swap even and odd bits of an 8-bit integer.
// Approach: ((n & 0xAA) >> 1) | ((n & 0x55) << 1).  Time O(1), Space O(1).
const swapBits = (n) => (((n & 0xaa) >> 1) | ((n & 0x55) << 1)) & 0xff;

for (const b of ["10101010", "11100010"]) {
  console.log(swapBits(parseInt(b, 2)).toString(2).padStart(8, "0"));
}
