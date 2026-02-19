// Day 1100: Search sorted array in O(log N) using only addition/comparison
// (no *, /, or bit-shift). Binary lifting with powers of two built by doubling.
// Time: O(log N). Space: O(log N).
function contains(a, x) {
  const n = a.length;
  if (n === 0) return false;
  const pows = [];
  for (let p = 1; p <= n; p += p) pows.push(p);
  let pos = -1;
  for (let i = pows.length - 1; i >= 0; i--) {
    const p = pows[i];
    if (pos + p < n && a[pos + p] <= x) pos += p;
  }
  return pos >= 0 && a[pos] === x;
}

const a = [1, 3, 5, 7, 9, 11];
console.log(contains(a, 7)); // true
console.log(contains(a, 8)); // false
