// Day 118: Two-pointer merge from both ends into result back-to-front. O(n) time, O(n) space.
function sortedSquares(a) {
  const n = a.length;
  const res = new Array(n);
  let lo = 0, hi = n - 1;
  for (let k = n - 1; k >= 0; k--) {
    const sl = a[lo] * a[lo], sh = a[hi] * a[hi];
    if (sl > sh) { res[k] = sl; lo++; } else { res[k] = sh; hi--; }
  }
  return res;
}

console.log("[" + sortedSquares([-9, -2, 0, 2, 3]).join(", ") + "]"); // [0, 4, 4, 9, 81]
