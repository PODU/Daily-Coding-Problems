// Power of four iff: positive, single set bit (n & (n-1))==0, and that bit sits
// at an even position (n & 0x55555555). O(1) time, O(1) space.

function isPowerOfFour(n) {
  return n > 0 && (n & (n - 1)) === 0 && (n & 0x55555555) !== 0;
}

for (const n of [1, 4, 16, 5, 64, 63]) {
  console.log(n + " -> " + isPowerOfFour(n));
}
