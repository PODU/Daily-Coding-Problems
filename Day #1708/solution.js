// Power of four: N>0 && single set bit (N&(N-1))==0 && bit at even position (N & 0x55555555). O(1).
function isPowerOfFour(n) {
  return n > 0 && (n & (n - 1)) === 0 && (n & 0x55555555) !== 0;
}

for (const t of [16, 8, 1, 64, 5]) {
  console.log(`${t} -> ${isPowerOfFour(t)}`);
}
