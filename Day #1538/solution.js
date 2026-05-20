// Minimum rooms for overlapping intervals: sort starts & ends, sweep with two pointers.
// Time O(n log n), Space O(n).
function minRooms(intervals) {
  const n = intervals.length;
  const starts = intervals.map((x) => x[0]).sort((a, b) => a - b);
  const ends = intervals.map((x) => x[1]).sort((a, b) => a - b);
  let rooms = 0, max = 0, j = 0;
  for (let i = 0; i < n; i++) {
    while (j < n && ends[j] <= starts[i]) { rooms--; j++; }
    rooms++;
    max = Math.max(max, rooms);
  }
  return max;
}

console.log(minRooms([[30, 75], [0, 50], [60, 150]]));
