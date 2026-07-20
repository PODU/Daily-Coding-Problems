// Day 1845: Index of nearest larger value (by array distance) via outward expansion.
// Time O(N) per query, Space O(1). Returns null if none exists.

function nearestLarger(a, i) {
  const n = a.length;
  for (let d = 1; d < n; d++) {
    if (i - d >= 0 && a[i - d] > a[i]) return i - d;
    if (i + d < n && a[i + d] > a[i]) return i + d;
  }
  return null;
}

function main() {
  console.log(nearestLarger([4, 1, 3, 5, 6], 0)); // 3
}

main();
