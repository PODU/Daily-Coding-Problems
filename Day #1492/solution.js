// Minimum meeting rooms for overlapping intervals.
// Sort starts and ends, two-pointer sweep. Time O(n log n), Space O(n).

function minRooms(intervals) {
  const n = intervals.length;
  const starts = intervals.map((iv) => iv[0]).sort((a, b) => a - b);
  const ends = intervals.map((iv) => iv[1]).sort((a, b) => a - b);
  let rooms = 0;
  let maxRooms = 0;
  let j = 0;
  for (let i = 0; i < n; i++) {
    while (j < n && ends[j] <= starts[i]) {
      rooms--;
      j++;
    }
    rooms++;
    maxRooms = Math.max(maxRooms, rooms);
  }
  return maxRooms;
}

const intervals = [[30, 75], [0, 50], [60, 150]];
console.log(minRooms(intervals)); // expected: 2
