// Square a sorted array and return sorted. Two pointers from both ends pick larger
// absolute value into the back of the result. O(N) time, O(N) space.
function sortedSquares(a) {
  const n = a.length;
  let l = 0, r = n - 1;
  const res = new Array(n);
  for (let k = n - 1; k >= 0; k--) {
    const lv = a[l] * a[l], rv = a[r] * a[r];
    if (lv > rv) { res[k] = lv; l++; }
    else { res[k] = rv; r--; }
  }
  return res;
}

console.log("[" + sortedSquares([-9, -2, 0, 2, 3]).join(", ") + "]"); // [0, 4, 4, 9, 81]
