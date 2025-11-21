// Day 637: Merge overlapping intervals.
// Approach: sort by start, sweep merging while next.start <= cur.end.
// Time: O(n log n), Space: O(n).
function merge(intervals) {
  const iv = [...intervals].sort((a, b) => a[0] - b[0]);
  const res = [];
  for (const [s, e] of iv) {
    if (res.length && s <= res[res.length - 1][1]) {
      res[res.length - 1][1] = Math.max(res[res.length - 1][1], e);
    } else {
      res.push([s, e]);
    }
  }
  return res;
}

const r = merge([[1, 3], [5, 8], [4, 10], [20, 25]]);
console.log("[" + r.map(([s, e]) => `(${s}, ${e})`).join(", ") + "]");
