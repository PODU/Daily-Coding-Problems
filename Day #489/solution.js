// Day 489: Longest subarray with all distinct elements.
// Sliding window + last-seen map; move left past previous occurrence. Time O(n), Space O(n).
function longestDistinctSubarray(a) {
  const last = new Map();
  let left = 0;
  let best = 0;
  for (let right = 0; right < a.length; right++) {
    const prev = last.get(a[right]);
    if (prev !== undefined && prev >= left) left = prev + 1;
    last.set(a[right], right);
    best = Math.max(best, right - left + 1);
  }
  return best;
}

console.log(longestDistinctSubarray([5, 1, 3, 5, 2, 3, 4, 1])); // 5
