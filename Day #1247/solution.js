// Squares of a sorted array via two-pointer merge from both ends. O(n) time, O(n) space.

function sortedSquares(a) {
  const n = a.length;
  const res = new Array(n);
  let l = 0, r = n - 1;
  for (let i = n - 1; i >= 0; --i) {
    const ls = a[l] * a[l], rs = a[r] * a[r];
    if (ls > rs) { res[i] = ls; ++l; }
    else { res[i] = rs; --r; }
  }
  return res;
}

function main() {
  const input = [-9, -2, 0, 2, 3];
  const res = sortedSquares(input);
  console.log("[" + res.join(", ") + "]");
}

main();
