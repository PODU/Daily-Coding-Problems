// Smallest sparse number (no adjacent 1 bits) >= N. Scan bits low->high; at the
// top of each adjacent-ones run, carry up and clear below. Time O(log N).
'use strict';

function nextSparse(x) {
  if (x === 0) return 0;
  const b = [];
  for (let t = x; t > 0; t = Math.floor(t / 2)) b.push(t & 1);
  b.push(0, 0); // padding for carries
  const n = b.length;
  for (let i = 1; i < n - 1; i++) {
    if (b[i] === 1 && b[i - 1] === 1 && b[i + 1] === 0) {
      b[i + 1] = 1;
      for (let j = i; j >= 0; j--) b[j] = 0;
    }
  }
  let ans = 0;
  for (let i = 0; i < n; i++) if (b[i]) ans += Math.pow(2, i);
  return ans;
}

console.log(nextSparse(22)); // 32
