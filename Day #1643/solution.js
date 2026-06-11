// Reverse the 32 bits of a 32-bit integer by shifting result left and pulling
// the lowest bit of the input each iteration. Time O(1) (32 iters), Space O(1).

function reverseBits(n) {
  let result = 0;
  for (let i = 0; i < 32; i++) {
    result = ((result << 1) | (n & 1)) >>> 0;
    n >>>= 1;
  }
  return result >>> 0;
}

function groupNibbles(n) {
  const bits = (n >>> 0).toString(2).padStart(32, '0');
  const parts = [];
  for (let i = 0; i < 32; i += 4) parts.push(bits.slice(i, i + 4));
  return parts.join(' ');
}

const value = 0xF0F0F0F0;
console.log(groupNibbles(reverseBits(value)));
