// Expand outward from index i, returning nearest j (by |j-i|) with a[j] > a[i]; null if none.
// Time: O(n) per query, Space: O(1).
"use strict";

function nearestLarger(a, i) {
  const n = a.length;
  for (let d = 1; d < n; d++) {
    const l = i - d, r = i + d;
    if (l >= 0 && a[l] > a[i]) return l;
    if (r < n && a[r] > a[i]) return r;
  }
  return null;
}

function main() {
  const a = [4, 1, 3, 5, 6];
  console.log(nearestLarger(a, 0));
}

main();
