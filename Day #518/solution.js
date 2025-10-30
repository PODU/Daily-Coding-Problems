// 3-sum decision: sort, fix one element, two-pointer scan the rest. O(n^2) time, O(1) extra.
function threeSum(a, k) {
  a = [...a].sort((x, y) => x - y);
  const n = a.length;
  for (let i = 0; i < n - 2; i++) {
    let lo = i + 1, hi = n - 1;
    while (lo < hi) {
      const s = a[i] + a[lo] + a[hi];
      if (s === k) return true;
      if (s < k) lo++; else hi--;
    }
  }
  return false;
}

console.log(threeSum([20, 303, 3, 4, 25], 49));
