// Day 1280: Swap adjacent bit pairs of an 8-bit unsigned integer.
// One-liner: shift odd bits up, even bits down. Time O(1), Space O(1).
const swapBits = (n) => ((n & 0xaa) >> 1) | ((n & 0x55) << 1);

for (const s of ["10101010", "11100010"]) {
  const n = parseInt(s, 2);
  console.log(swapBits(n).toString(2).padStart(8, "0"));
}
