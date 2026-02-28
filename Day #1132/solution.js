// Left-rotate array in place by k using 3 reversals. O(n) time, O(1) space.

function reverseRange(a, i, j) {
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
  reverseRange(a, 0, k - 1);
  reverseRange(a, k, n - 1);
  reverseRange(a, 0, n - 1);
}

const a = [1, 2, 3, 4, 5, 6];
rotateLeft(a, 2);
console.log("[" + a.join(", ") + "]");
