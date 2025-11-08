// Find min and max using ~3*ceil(N/2) comparisons (pairwise method).
// Time: O(N) with <2N comparisons. Space: O(1).
'use strict';

function minMax(a) {
  let cmps = 0;
  const n = a.length;
  let mn, mx, i;
  if (n % 2 === 1) { mn = mx = a[0]; i = 1; }      // odd: seed with first
  else {                                            // even: seed with first pair
    cmps++;
    if (a[0] < a[1]) { mn = a[0]; mx = a[1]; }
    else { mn = a[1]; mx = a[0]; }
    i = 2;
  }
  for (; i + 1 < n; i += 2) {
    let lo, hi;
    cmps++;
    if (a[i] < a[i + 1]) { lo = a[i]; hi = a[i + 1]; }
    else { lo = a[i + 1]; hi = a[i]; }
    cmps++; if (lo < mn) mn = lo;
    cmps++; if (hi > mx) mx = hi;
  }
  return { mn, mx, cmps };
}

const arr = [7, 2, 9, 4, 1, 8, 3];
const r = minMax(arr);
console.log(`min=${r.mn} max=${r.mx}`);
