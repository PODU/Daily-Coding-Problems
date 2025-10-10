// Min meeting rooms: sort starts and ends, two-pointer sweep, half-open [start,end).
// Time O(n log n), Space O(n).

function minRooms(intervals) {
  const n = intervals.length;
  const starts = intervals.map(p => p[0]).sort((a, b) => a - b);
  const ends = intervals.map(p => p[1]).sort((a, b) => a - b);
  let rooms = 0, maxRooms = 0, j = 0;
  for (let i = 0; i < n; i++) {
    while (j < n && ends[j] <= starts[i]) { rooms--; j++; }
    rooms++;
    maxRooms = Math.max(maxRooms, rooms);
  }
  return maxRooms;
}

const intervals = [[30, 75], [0, 50], [60, 150]];
console.log(minRooms(intervals));
