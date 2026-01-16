// Greedy: sort intervals by end; keep interval if start >= last kept end.
// Answer = total - kept. Time O(n log n), Space O(n).
function eraseOverlapIntervals(intervals) {
  intervals = intervals.slice().sort((a, b) => a[1] - b[1]);
  let kept = 0, lastEnd = -Infinity;
  for (const [s, e] of intervals) {
    if (s >= lastEnd) { kept++; lastEnd = e; }
  }
  return intervals.length - kept;
}

const intervals = [[7, 9], [2, 4], [5, 8]];
console.log(eraseOverlapIntervals(intervals));
