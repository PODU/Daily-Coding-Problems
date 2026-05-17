// Rearrange string so no two adjacent chars equal.
// Greedy: place chars by descending frequency into even indices then odd indices.
// Time O(n + k log k), Space O(n).
function reorganize(s) {
  const n = s.length;
  const cnt = {};
  for (const c of s) cnt[c] = (cnt[c] || 0) + 1;
  let maxc = 0;
  for (const k in cnt) maxc = Math.max(maxc, cnt[k]);
  if (maxc > Math.floor((n + 1) / 2)) return null;
  const entries = Object.entries(cnt).sort((a, b) => b[1] - a[1]);
  const res = new Array(n);
  let idx = 0;
  for (const [ch, c] of entries) {
    for (let j = 0; j < c; j++) {
      res[idx] = ch;
      idx += 2;
      if (idx >= n) idx = 1;
    }
  }
  return res.join("");
}

const r = reorganize("aaabbc");
console.log(r === null ? "None" : r);
