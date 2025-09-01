// Day 197: Rotate array right by k in-place.
// Triple-reversal: reverse whole, reverse first k, reverse rest. O(n) time, O(1) space.
function reverse(a, i, j) {
  while (i < j) { [a[i], a[j]] = [a[j], a[i]]; i++; j--; }
}

function rotateRight(a, k) {
  const n = a.length;
  if (n === 0) return;
  k %= n;
  reverse(a, 0, n - 1);
  reverse(a, 0, k - 1);
  reverse(a, k, n - 1);
}

const a = [1, 2, 3, 4, 5];
rotateRight(a, 2);
console.log(a); // [4, 5, 1, 2, 3]
