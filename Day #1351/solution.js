// Pairwise min/max: process pairs, compare smaller->min, larger->max -> ~3N/2 comparisons.
// Time: O(N) (~3N/2 comparisons), Space: O(1).
function minMax(a) {
  const n = a.length;
  let mn, mx, i;
  if (n % 2 === 0) {
    if (a[0] < a[1]) { mn = a[0]; mx = a[1]; } else { mn = a[1]; mx = a[0]; }
    i = 2;
  } else {
    mn = mx = a[0];
    i = 1;
  }
  for (; i + 1 <= n; i += 2) {
    let small, large;
    if (a[i] < a[i + 1]) { small = a[i]; large = a[i + 1]; }
    else { small = a[i + 1]; large = a[i]; }
    if (small < mn) mn = small;
    if (large > mx) mx = large;
  }
  return [mn, mx];
}

const a = [3, 5, 1, 2, 4, 8, 7];
const [mn, mx] = minMax(a);
console.log(`Min: ${mn}, Max: ${mx}`);
