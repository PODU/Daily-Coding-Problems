// Day 1024: Reverse all 32 bits of a 32-bit integer.
// Approach: swap-mask reversal in O(log 32) = O(1) time, O(1) space.
function reverseBits(n) {
  n = ((n >>> 16) | (n << 16)) >>> 0;
  n = (((n & 0xff00ff00) >>> 8) | ((n & 0x00ff00ff) << 8)) >>> 0;
  n = (((n & 0xf0f0f0f0) >>> 4) | ((n & 0x0f0f0f0f) << 4)) >>> 0;
  n = (((n & 0xcccccccc) >>> 2) | ((n & 0x33333333) << 2)) >>> 0;
  n = (((n & 0xaaaaaaaa) >>> 1) | ((n & 0x55555555) << 1)) >>> 0;
  return n >>> 0;
}

function grouped(n) {
  const bits = (n >>> 0).toString(2).padStart(32, "0");
  const parts = [];
  for (let i = 0; i < 32; i += 4) parts.push(bits.slice(i, i + 4));
  return parts.join(" ");
}

const x = 0xf0f0f0f0;
console.log(grouped(reverseBits(x)));
