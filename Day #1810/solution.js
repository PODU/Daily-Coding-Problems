// 3-sum decision: does any triple sum to k?
// Sort, then for each i two-pointer scan. Time: O(n^2). Space: O(1).
function threeSum(arr, k) {
  const a = [...arr].sort((x, y) => x - y);
  const n = a.length;
  for (let i = 0; i < n - 2; i++) {
    let lo = i + 1, hi = n - 1;
    while (lo < hi) {
      const s = a[i] + a[lo] + a[hi];
      if (s === k) return true;
      else if (s < k) lo++;
      else hi--;
    }
  }
  return false;
}

console.log(threeSum([20, 303, 3, 4, 25], 49)); // true
