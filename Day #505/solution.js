// Day 505: Rotate array right by k in-place via three reversals.
// Time O(n), Space O(1).

function reverse(a, lo, hi) {
  while (lo < hi) {
    const t = a[lo];
    a[lo] = a[hi];
    a[hi] = t;
    lo++;
    hi--;
  }
}

function rotateRight(a, k) {
  const n = a.length;
  if (n === 0) return;
  k %= n;
  reverse(a, 0, n - 1);
  reverse(a, 0, k - 1);
  reverse(a, k, n - 1);
}

function main() {
  const a = [1, 2, 3, 4, 5, 6, 7];
  rotateRight(a, 3);
  console.log("[" + a.join(", ") + "]"); // [5, 6, 7, 1, 2, 3, 4]
}

main();
