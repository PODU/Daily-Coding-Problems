// Day 217: Smallest sparse number (no two adjacent set bits) >= N.
// Approach: scan bits low->high; on adjacent 11 with a 0 above, carry up and clear lower bits.
// Time O(bits) ~ O(log N), much faster than O(N log N). Space O(log N).
function nextSparse(n) {
  if (n <= 0) return 0;
  const bits = [];
  let x = n;
  while (x) { bits.push(x & 1); x = Math.floor(x / 2); }
  bits.push(0, 0);
  let lastFinal = 0;
  for (let i = 1; i < bits.length - 1; i++) {
    if (bits[i] === 1 && bits[i - 1] === 1 && bits[i + 1] === 0) {
      bits[i + 1] = 1;
      for (let j = i; j >= lastFinal; j--) bits[j] = 0;
      lastFinal = i + 1;
    }
  }
  let res = 0;
  for (let i = bits.length - 1; i >= 0; i--) res = res * 2 + bits[i];
  return res;
}

console.log(nextSparse(22)); // -> 32
console.log(nextSparse(21)); // -> 21
