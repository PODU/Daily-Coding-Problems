// Reorganize string: count freqs, if max > (n+1)/2 impossible; sort chars by freq desc (tie by char),
// fill even indices first then odd. Time O(n log k), Space O(n).
function reorganize(s) {
  const n = s.length;
  const cnt = {};
  for (const c of s) cnt[c] = (cnt[c] || 0) + 1;
  for (const c in cnt) if (cnt[c] > Math.floor((n + 1) / 2)) return null;
  const chars = Object.keys(cnt).sort((a, b) => {
    if (cnt[a] !== cnt[b]) return cnt[b] - cnt[a]; // freq desc
    return a < b ? -1 : a > b ? 1 : 0;             // tie by char asc
  });
  const res = new Array(n).fill('');
  let idx = 0;
  for (const c of chars) {
    for (let k = 0; k < cnt[c]; k++) {
      res[idx] = c;
      idx += 2;
      if (idx >= n) idx = 1;
    }
  }
  return res.join('');
}

const r1 = reorganize("aaabbc");
console.log(r1 === null ? "None" : r1);
const r2 = reorganize("aaab");
console.log(r2 === null ? "None" : r2);
