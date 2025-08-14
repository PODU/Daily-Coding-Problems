// Day 119: Min points to stab all intervals. Greedy: sort by start desc, pick start
// of each not-yet-stabbed interval (mirror of the classic sort-by-end greedy). O(n log n).
function minCover(intervals) {
  const iv = intervals.slice().sort((a, b) => b[0] - a[0]);
  const pts = [];
  let last = null;
  for (const [s, e] of iv) {
    if (last === null || last > e) { last = s; pts.push(s); }
  }
  return pts.sort((a, b) => a - b);
}

const r = minCover([[0, 3], [2, 6], [3, 4], [6, 9]]);
console.log("{" + r.join(", ") + "}"); // {3, 6}
