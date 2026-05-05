// Day 1477: First N regular numbers (only prime factors 2, 3, 5).
// DP with three pointers merging *2, *3, *5 sequences. Time O(N), Space O(N).

function regularNumbers(n) {
  if (n <= 0) return [];
  const h = new Array(n).fill(1);
  let i2 = 0, i3 = 0, i5 = 0;
  for (let k = 1; k < n; ++k) {
    const nxt = Math.min(h[i2] * 2, h[i3] * 3, h[i5] * 5);
    h[k] = nxt;
    if (nxt === h[i2] * 2) ++i2;
    if (nxt === h[i3] * 3) ++i3;
    if (nxt === h[i5] * 5) ++i5;
  }
  return h;
}

console.log(regularNumbers(10)); // [1, 2, 3, 4, 5, 6, 8, 9, 10, 12]
