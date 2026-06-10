// Minimum stabbing points: greedy sort intervals by right endpoint; add right
// endpoint when current interval is unstabbed. Time O(n log n), Space O(n).
"use strict";

function minStabbingPoints(intervals) {
  const sorted = intervals.slice().sort((a, b) => a[1] - b[1]);
  const points = [];
  let last = -Infinity;
  for (const [a, b] of sorted) {
    if (a > last) {
      points.push(b);
      last = b;
    }
  }
  return points;
}

const intervals = [[1, 4], [4, 5], [7, 9], [9, 12]];
console.log("[" + minStabbingPoints(intervals).join(", ") + "]");
