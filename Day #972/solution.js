// Day 972: Rearrange string so no two adjacent chars match (else null/None).
// Approach: place most frequent chars into even-then-odd slots. Time O(n log k), Space O(n).

function rearrange(s) {
  const n = s.length;
  const cnt = new Map();
  for (const c of s) cnt.set(c, (cnt.get(c) || 0) + 1);
  const order = [...cnt.entries()].sort((a, b) => b[1] - a[1]);
  if (order[0][1] > Math.floor((n + 1) / 2)) return null;
  const res = new Array(n);
  let idx = 0;
  for (const [ch, c] of order) {
    for (let k = 0; k < c; k++) {
      res[idx] = ch;
      idx += 2;
      if (idx >= n) idx = 1;
    }
  }
  return res.join('');
}

console.log(rearrange('aaabbc')); // ababac
console.log(rearrange('aaab') === null ? 'None' : rearrange('aaab')); // None
