// Day 1446: Minimum set of points hitting every closed interval.
// Greedy: sort by right endpoint; whenever the current interval is unhit, pick
// its right endpoint. Time O(n log n), Space O(1). (Any minimal set is valid.)
function minStabbingSet(intervals) {
  const sorted = intervals.slice().sort((a, b) => a[1] - b[1]);
  const points = [];
  let last = -Infinity;
  for (const [l, r] of sorted) {
    if (l > last) { points.push(r); last = r; }
  }
  return points;
}

const intervals = [[0, 3], [2, 6], [3, 4], [6, 9]];
const pts = minStabbingSet(intervals);
console.log("{" + pts.join(", ") + "}"); // e.g. {3, 9}; {3, 6} also valid
