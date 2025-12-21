// Day 775: Minimum meeting rooms = max overlapping intervals.
// Sort starts and ends, two-pointer sweep. O(n log n) time, O(n) space.
function minRooms(intervals) {
  const n = intervals.length;
  const starts = intervals.map(x => x[0]).sort((a, b) => a - b);
  const ends = intervals.map(x => x[1]).sort((a, b) => a - b);
  let rooms = 0, best = 0, i = 0, j = 0;
  while (i < n) {
    if (starts[i] < ends[j]) { rooms++; i++; best = Math.max(best, rooms); }
    else { rooms--; j++; }
  }
  return best;
}

console.log(minRooms([[30, 75], [0, 50], [60, 150]])); // 2
