// Day 987: Nearest larger number index.
// Expand outward from i checking i-d and i+d; first larger wins. O(n) per query, O(1) space.
// Follow-up: with O(n^2) preprocessing (or sparse tables) each query can be answered in O(1).

// Returns index of nearest larger element, or null if none.
function nearestLarger(a, i) {
  const n = a.length;
  for (let d = 1; d < n; d++) {
    const l = i - d, r = i + d;
    if (l >= 0 && a[l] > a[i]) return l; // prefer left on ties
    if (r < n && a[r] > a[i]) return r;
  }
  return null;
}

const a = [4, 1, 3, 5, 6];
const idx = nearestLarger(a, 0);
console.log(idx === null ? "null" : idx); // expected 3
