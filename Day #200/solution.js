// Day 200: Minimum points stabbing all intervals.
// Greedy: sort by right endpoint; pick the right end whenever current interval is unstabbed.
// Time: O(n log n), Space: O(1).
function stab(intervals) {
  const iv = [...intervals].sort((a, b) => a[1] - b[1]);
  const pts = [];
  let last = -Infinity;
  for (const [lo, hi] of iv) {
    if (lo > last) { last = hi; pts.push(last); }
  }
  return pts;
}

console.log(stab([[1, 4], [4, 5], [7, 9], [9, 12]])); // [4, 9]
