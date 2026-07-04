// Merge overlapping intervals: sort by start, then sweep merging when the next
// start <= current end. Time: O(n log n), Space: O(n).

function merge(intervals) {
  const sorted = [...intervals].sort((a, b) => a[0] - b[0]);
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

const iv = [[1, 3], [5, 8], [4, 10], [20, 25]];
const r = merge(iv);
console.log("[" + r.map(([a, b]) => `(${a}, ${b})`).join(", ") + "]");
