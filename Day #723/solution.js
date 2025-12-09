// Day 723: Smallest set of points stabbing all closed intervals.
// Approach: Sort by right endpoint; greedily pick the end of each uncovered interval.
// Time: O(n log n), Space: O(n). (Any minimum-size set is valid; {3,9} here.)

function minStabbingPoints(intervals) {
  intervals = [...intervals].sort((a, b) => a[1] - b[1]);
  const points = [];
  let last = -Infinity;
  for (const [s, e] of intervals) {
    if (s > last) { last = e; points.push(e); }
  }
  return points;
}

const intervals = [[0, 3], [2, 6], [3, 4], [6, 9]];
console.log("{" + minStabbingPoints(intervals).join(", ") + "}");
