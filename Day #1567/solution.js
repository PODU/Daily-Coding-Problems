// Busiest period: sort events by timestamp, sweep current occupancy, track max interval.
// Time O(n log n), space O(n).
function busiestPeriod(events) {
  // events: [timestamp, count, isEnter]
  const sorted = events.slice().sort((a, b) => a[0] - b[0]);
  let cur = 0, best = -1, bestStart = 0, bestEnd = 0;
  for (let i = 0; i < sorted.length; i++) {
    const [ts, count, isEnter] = sorted[i];
    cur += isEnter ? count : -count;
    if (cur > best && i + 1 < sorted.length) {
      best = cur;
      bestStart = ts;
      bestEnd = sorted[i + 1][0];
    }
  }
  return [bestStart, bestEnd];
}

const events = [
  [1, 3, true],
  [4, 2, true],
  [6, 5, false],
];
const [start, end] = busiestPeriod(events);
console.log(`(${start}, ${end})`);
