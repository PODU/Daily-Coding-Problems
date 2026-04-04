// Day 1300: Find a contiguous subarray summing to K (handles negatives).
// Prefix-sum hashmap: for each prefix p, look for p-K seen earlier. O(N) time, O(N) space.
function subarraySum(a, K) {
  const firstIndex = new Map([[0, -1]]);
  let prefix = 0;
  for (let j = 0; j < a.length; j++) {
    prefix += a[j];
    if (firstIndex.has(prefix - K)) {
      return a.slice(firstIndex.get(prefix - K) + 1, j + 1);
    }
    if (!firstIndex.has(prefix)) firstIndex.set(prefix, j);
  }
  return [];
}

console.log(subarraySum([1, 2, 3, 4, 5], 9)); // [2, 3, 4]
