// Busiest period: sort events by timestamp, sweep occupancy, track max-occupancy interval [t_i, t_{i+1}). O(n log n) time, O(n) space.

function busiestPeriod(events) {
  const sorted = [...events].sort((a, b) => a.timestamp - b.timestamp);
  let current = 0, maxOcc = -1, best = [0, 0];
  for (let i = 0; i < sorted.length; i++) {
    if (sorted[i].type === "enter") current += sorted[i].count;
    else current -= sorted[i].count;
    const nextTs = i + 1 < sorted.length ? sorted[i + 1].timestamp : sorted[i].timestamp;
    if (current > maxOcc) {
      maxOcc = current;
      best = [sorted[i].timestamp, nextTs];
    }
  }
  return best;
}

const events = [
  { timestamp: 1526579928, count: 3, type: "enter" },
  { timestamp: 1526580000, count: 2, type: "enter" },
  { timestamp: 1526580382, count: 2, type: "exit" },
  { timestamp: 1526580500, count: 1, type: "enter" },
  { timestamp: 1526580700, count: 4, type: "exit" },
];

const [start, end] = busiestPeriod(events);
console.log(`(${start}, ${end})`);
