// Nearest larger element's index by index-distance: expand outward from i. O(n) per query, O(1) space.

function nearestLarger(a, i) {
  const n = a.length;
  for (let d = 1; d < n; d++) {
    if (i - d >= 0 && a[i - d] > a[i]) return i - d;
    if (i + d < n && a[i + d] > a[i]) return i + d;
  }
  return null;
}

const a = [4, 1, 3, 5, 6];
console.log(nearestLarger(a, 0)); // 3
