// Day 1124 - Minimum points to stab all intervals
// Greedy: sort by right endpoint, place a point at the end of each not-yet-
// stabbed interval. Time: O(n log n), Space: O(n).

function stab(intervals) {
  const sorted = [...intervals].sort((a, b) => a[1] - b[1]);
  const points = [];
  let last = -Infinity;
  for (const [s, e] of sorted)
    if (s > last) { last = e; points.push(e); }
  return points;
}

console.log(stab([[1, 4], [4, 5], [7, 9], [9, 12]])); // [4, 9]
