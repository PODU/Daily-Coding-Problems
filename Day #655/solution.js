// Reverse 32 bits: iterate 32 times, shift result left, OR in LSB of input, shift input right.
// Time O(32)=O(1), space O(1). Uses >>> 0 to keep values unsigned 32-bit.

function reverseBits(x) {
  x = x >>> 0;
  let result = 0;
  for (let i = 0; i < 32; i++) {
    result = ((result << 1) | (x & 1)) >>> 0;
    x = x >>> 1;
  }
  return result >>> 0;
}

function main() {
  const out = reverseBits(0xF0F0F0F0);
  const bits = out.toString(2).padStart(32, '0');
  const groups = [];
  for (let i = 0; i < 32; i += 4) groups.push(bits.slice(i, i + 4));
  console.log(groups.join(' '));
}

main();
