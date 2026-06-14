// Smallest unsorted window. Scan L->R tracking max for end, R->L tracking min for start. O(n) time, O(1) space.
function unsortedWindow(a) {
  const n = a.length;
  let end = -1, start = -1;
  let mx = -Infinity, mn = Infinity;
  for (let i = 0; i < n; i++) {
    mx = Math.max(mx, a[i]);
    if (a[i] < mx) end = i;
  }
  for (let i = n - 1; i >= 0; i--) {
    mn = Math.min(mn, a[i]);
    if (a[i] > mn) start = i;
  }
  return [start, end];
}

const [s, e] = unsortedWindow([3, 7, 5, 6, 9]);
console.log(`(${s}, ${e})`);
