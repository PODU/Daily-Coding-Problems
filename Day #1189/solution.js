// 3-sum existence: sort, fix each i, two-pointer scan remaining pair. Time O(N^2), Space O(1).
'use strict';

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

function main() {
  const arr = [20, 303, 3, 4, 25];
  console.log(threeSum(arr, 49));
}

main();
