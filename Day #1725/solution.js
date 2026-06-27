// Min intervals to remove for non-overlapping (touching allowed).
// Greedy: sort by end, keep intervals whose start >= last kept end. O(n log n) time, O(1) extra space.

function minRemovals(intervals) {
  intervals = intervals.slice().sort((a, b) => a[1] - b[1]);
  let kept = 0;
  let lastEnd = -Infinity;
  for (const [start, end] of intervals) {
    if (start >= lastEnd) {
      kept++;
      lastEnd = end;
    }
  }
  return intervals.length - kept;
}

const intervals = [[7, 9], [2, 4], [5, 8]];
console.log(minRemovals(intervals));
