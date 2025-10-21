// Nearest larger: expand outward from i, return first index with greater value.
// Time: O(n), Space: O(1).
function nearestLarger(a, i) {
  const n = a.length;
  for (let d = 1; d < n; d++) {
    if (i - d >= 0 && a[i - d] > a[i]) return i - d;
    if (i + d < n && a[i + d] > a[i]) return i + d;
  }
  return null;
}

const a = [4, 1, 3, 5, 6];
const r = nearestLarger(a, 0);
console.log(r === null ? "null" : r);
