// Day 1436: Length of longest subarray with all distinct elements.
// Approach: Sliding window with last-seen index map; shrink left on repeat.
// Time: O(n), Space: O(n).

function longestDistinct(arr) {
  const last = new Map();
  let best = 0, left = 0;
  for (let right = 0; right < arr.length; right++) {
    const v = arr[right];
    if (last.has(v) && last.get(v) >= left) left = last.get(v) + 1;
    last.set(v, right);
    best = Math.max(best, right - left + 1);
  }
  return best;
}

console.log(longestDistinct([5, 1, 3, 5, 2, 3, 4, 1])); // 5
