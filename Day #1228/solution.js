// Merge overlapping intervals: sort by start, sweep merging when start <= last end.
// Time: O(n log n), Space: O(n).
function merge(intervals) {
  const sorted = intervals.slice().sort((a, b) => a[0] - b[0]);
  const res = [];
  for (const [s, e] of sorted) {
    if (res.length && s <= res[res.length - 1][1]) {
      res[res.length - 1][1] = Math.max(res[res.length - 1][1], e);
    } else {
      res.push([s, e]);
    }
  }
  return res;
}

const data = [[1, 3], [5, 8], [4, 10], [20, 25]];
const out = merge(data);
console.log("[" + out.map(([s, e]) => `(${s}, ${e})`).join(", ") + "]");
