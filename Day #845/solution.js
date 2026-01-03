// Day 845: rotate a list left by k in place using the 3-reversal trick.
// reverse(0,k-1), reverse(k,n-1), reverse(0,n-1). O(n) time, O(1) extra space.
function reverse(a, i, j) {
  while (i < j) { const t = a[i]; a[i++] = a[j]; a[j--] = t; }
}

function rotateLeft(a, k) {
  const n = a.length;
  if (n === 0) return a;
  k = ((k % n) + n) % n;
  reverse(a, 0, k - 1);
  reverse(a, k, n - 1);
  reverse(a, 0, n - 1);
  return a;
}

console.log(rotateLeft([1, 2, 3, 4, 5, 6], 2)); // [ 3, 4, 5, 6, 1, 2 ]
