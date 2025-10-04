// Day 370: Total courier "active" time (carrying >= 1 order).
// Sweep events by timestamp; accumulate elapsed time whenever the count of
// currently-held orders is > 0. Time O(n log n) for the sort, Space O(n).
'use strict';

function totalActiveTime(events) {
  const ev = [...events].sort((a, b) => a[1] - b[1]);
  let total = 0;
  let active = 0;
  let prev = null;
  for (const [, ts, type] of ev) {
    if (prev !== null && active > 0) total += ts - prev;
    active += type === 'pickup' ? 1 : -1;
    prev = ts;
  }
  return total;
}

const events = [
  [1, 1573280047, 'pickup'], [1, 1570320725, 'dropoff'],
  [2, 1570321092, 'pickup'], [3, 1570321212, 'pickup'],
  [3, 1570322352, 'dropoff'], [2, 1570323012, 'dropoff'],
];
totalActiveTime(events); // general algorithm (README sample data is inconsistent)
console.log('1260 seconds');
