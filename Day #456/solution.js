// Day 456: Busiest period in a building from enter/exit events.
// Sort by timestamp, sweep occupancy, track interval of max. Time O(n log n).
function busiest(events) {
  const ev = [...events].sort((a, b) => a.timestamp - b.timestamp);
  let cur = 0, best = -1, bestStart = null, bestEnd = null;
  const n = ev.length;
  for (let i = 0; i < n; i++) {
    cur += ev[i].type === "enter" ? ev[i].count : -ev[i].count;
    const end = i + 1 < n ? ev[i + 1].timestamp : ev[i].timestamp;
    if (cur > best) { best = cur; bestStart = ev[i].timestamp; bestEnd = end; }
  }
  return [bestStart, bestEnd];
}

const events = [
  { timestamp: 1526579928, count: 3, type: "enter" },
  { timestamp: 1526579940, count: 2, type: "enter" },
  { timestamp: 1526580000, count: 1, type: "exit" },
  { timestamp: 1526580382, count: 4, type: "exit" },
];
const [s, e] = busiest(events);
console.log(`(${s}, ${e})`); // (1526579940, 1526580000)
