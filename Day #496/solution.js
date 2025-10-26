// Day 496: Total set bits across 1..N.
// For each bit position, count how many numbers in [0,N] have it set using the
// periodic pattern. O(log N) time, O(1) space.
function countSetBits(n) {
  let total = 0;
  for (let bit = 1; bit <= n; bit *= 2) {
    const full = n + 1;     // count of integers in [0, n]
    const cycle = bit * 2;  // period for this bit
    total += Math.floor(full / cycle) * bit;
    const rem = full % cycle;
    total += Math.max(0, rem - bit);
  }
  return total;
}

console.log(countSetBits(5));  // 7  (1+1+2+1+2)
console.log(countSetBits(16)); // 33
