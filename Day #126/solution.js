// Day 126: Rotate list left by k in-place via 3 reversals.
// O(n) time, O(1) extra space, ~n swaps total.
function reverse(a, i, j) {
  while (i < j) {
    const t = a[i];
    a[i] = a[j];
    a[j] = t;
    i++;
    j--;
  }
}

function rotateLeft(a, k) {
  const n = a.length;
  if (n === 0) return;
  k %= n;
  reverse(a, 0, k - 1);
  reverse(a, k, n - 1);
  reverse(a, 0, n - 1);
}

const a = [1, 2, 3, 4, 5, 6];
rotateLeft(a, 2);
console.log("[" + a.join(", ") + "]");
