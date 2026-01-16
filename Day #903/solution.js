// Sort, then fix arr[i] and two-pointer scan for the remaining pair summing to k.
// Time O(n^2), Space O(1) extra.
function threeSum(arr, k) {
  arr = arr.slice().sort((a, b) => a - b);
  const n = arr.length;
  for (let i = 0; i < n - 2; i++) {
    let lo = i + 1, hi = n - 1;
    while (lo < hi) {
      const s = arr[i] + arr[lo] + arr[hi];
      if (s === k) return true;
      if (s < k) lo++; else hi--;
    }
  }
  return false;
}

const arr = [20, 303, 3, 4, 25];
console.log(threeSum(arr, 49) ? "true" : "false");
