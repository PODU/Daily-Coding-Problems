// Day 950: busiest period - interval with the most people inside the building.
// Sort events by timestamp, sweep maintaining running count. Time O(n log n), Space O(n).

function busiest(events) {
  const ev = [...events].sort((a, b) => a.timestamp - b.timestamp);
  let cur = 0;
  let best = -Infinity;
  let ans = [0, 0];
  for (let i = 0; i < ev.length; i++) {
    cur += ev[i].type === "enter" ? ev[i].count : -ev[i].count;
    const nextTs = i + 1 < ev.length ? ev[i + 1].timestamp : ev[i].timestamp;
    if (cur > best) {
      best = cur;
      ans = [ev[i].timestamp, nextTs];
    }
  }
  return ans;
}

const events = [
  { timestamp: 1526579928, count: 3, type: "enter" },
  { timestamp: 1526579943, count: 4, type: "enter" },
  { timestamp: 1526580382, count: 2, type: "exit" },
  { timestamp: 1526581000, count: 5, type: "exit" },
];
const [s, e] = busiest(events);
console.log(`(${s}, ${e})`); // (1526579943, 1526580382)
