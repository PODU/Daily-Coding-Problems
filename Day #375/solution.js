// Day 375: h-index via counting sort.
// Bucket citations (capped at n), then scan h from n down accumulating papers
// with >= h citations; first h with count >= h wins. Time O(n), Space O(n).
'use strict';

function hIndex(cites) {
  const n = cites.length;
  const buckets = new Array(n + 1).fill(0);
  for (const c of cites) buckets[Math.min(c, n)]++;
  let total = 0;
  for (let h = n; h >= 0; h--) {
    total += buckets[h];
    if (total >= h) return h;
  }
  return 0;
}

console.log(hIndex([4, 0, 0, 2, 3])); // 2
