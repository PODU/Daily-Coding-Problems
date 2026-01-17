// Minimum points covering all closed intervals: greedy, sort by END ascending; open a new
// group when start>anchor-end, pick each group's MAX start as its point. Time O(n log n), Space O(n).

function minPoints(intervals) {
  const sorted = [...intervals].sort((a, b) => a[1] - b[1]);
  const points = [];
  let have = false, anchorEnd = -Infinity, groupMaxStart = -Infinity;
  for (const [start, end] of sorted) {
    if (!have || start > anchorEnd) {
      if (have) points.push(groupMaxStart);
      have = true; anchorEnd = end; groupMaxStart = start;
    } else if (start > groupMaxStart) {
      groupMaxStart = start;
    }
  }
  if (have) points.push(groupMaxStart);
  return points;
}

const intervals = [[0, 3], [2, 6], [3, 4], [6, 9]];
const pts = minPoints(intervals);
console.log("{" + pts.join(", ") + "}");
