// Day 768: Find min and max together using ~3N/2 comparisons (< 2*(N-2)).
// Process elements in pairs: compare the pair, then smaller vs min, larger vs max.
function minMax(a) {
  const n = a.length;
  let mn, mx, i;
  if (n % 2 === 0) { mn = Math.min(a[0], a[1]); mx = Math.max(a[0], a[1]); i = 2; }
  else { mn = mx = a[0]; i = 1; }
  for (; i < n; i += 2) {
    let lo = a[i], hi = a[i + 1];
    if (lo > hi) [lo, hi] = [hi, lo];
    if (lo < mn) mn = lo;
    if (hi > mx) mx = hi;
  }
  return [mn, mx];
}

const [mn, mx] = minMax([3, 5, 1, 2, 4, 8, 7]);
console.log(`min=${mn} max=${mx}`); // min=1 max=8
