// Day 462: Smallest sparse number (no adjacent set bits) >= N.
// Scan bits low->high, lift each "11" pair upward. Time O(log N), Space O(log N).
function nextSparse(n) {
  if (n <= 0n) return n;
  const bits = [];
  let t = n;
  while (t > 0n) { bits.push(Number(t & 1n)); t >>= 1n; }
  bits.push(0, 0);
  const size = bits.length;
  let lastFinal = 0;
  for (let i = 1; i < size - 1; i++) {
    if (bits[i] === 1 && bits[i - 1] === 1 && bits[i + 1] === 0) {
      bits[i + 1] = 1;
      for (let j = i; j >= lastFinal; j--) bits[j] = 0;
      lastFinal = i + 1;
    }
  }
  let ans = 0n;
  for (let i = 0; i < size; i++) if (bits[i]) ans |= (1n << BigInt(i));
  return ans;
}

console.log(nextSparse(22n).toString()); // 32
console.log(nextSparse(21n).toString()); // 21
