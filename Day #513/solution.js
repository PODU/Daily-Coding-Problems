// Contiguous subarray summing to K via prefix-sum hash map. O(n) time, O(n) space.
function subarraySum(a, K) {
  const seen = new Map([[0, -1]]);
  let pre = 0;
  for (let i = 0; i < a.length; i++) {
    pre += a[i];
    if (seen.has(pre - K)) {
      return a.slice(seen.get(pre - K) + 1, i + 1);
    }
    if (!seen.has(pre)) seen.set(pre, i);
  }
  return [];
}

console.log(subarraySum([1, 2, 3, 4, 5], 9));
