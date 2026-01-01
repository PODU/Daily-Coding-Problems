// Longest subarray with all distinct elements.
// Sliding window with last-seen index map. Time: O(N), Space: O(N).

function longestDistinct(a) {
  const last = new Map();
  let best = 0, start = 0;
  for (let i = 0; i < a.length; i++) {
    if (last.has(a[i]) && last.get(a[i]) >= start) start = last.get(a[i]) + 1;
    last.set(a[i], i);
    best = Math.max(best, i - start + 1);
  }
  return best;
}

console.log(longestDistinct([5, 1, 3, 5, 2, 3, 4, 1]));
