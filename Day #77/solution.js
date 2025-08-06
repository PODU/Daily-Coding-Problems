// Day 77: Merge overlapping intervals. Sort by start, then sweep merging.
// Time O(n log n), Space O(n).
function mergeIntervals(intervals) {
  const iv = intervals.slice().sort((a, b) => a[0] - b[0]);
  const res = [];
  for (const [start, end] of iv) {
    if (res.length && start <= res[res.length - 1][1]) {
      res[res.length - 1][1] = Math.max(res[res.length - 1][1], end);
    } else {
      res.push([start, end]);
    }
  }
  return res;
}

const inp = [[1, 3], [5, 8], [4, 10], [20, 25]];
const out = mergeIntervals(inp);
console.log("[" + out.map(([a, b]) => `(${a}, ${b})`).join(", ") + "]");
// [(1, 3), (4, 10), (20, 25)]
