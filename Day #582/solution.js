// Greedy interval stabbing: sort by right endpoint, pick right end when uncovered. Time O(n log n).
function stab(intervals) {
  const sorted = intervals.slice().sort((a, b) => a[1] - b[1]);
  const points = [];
  let last = -Infinity;
  for (const [start, end] of sorted) {
    if (start > last) {
      last = end;
      points.push(end);
    }
  }
  return points;
}

const intervals = [[1, 4], [4, 5], [7, 9], [9, 12]];
console.log("[" + stab(intervals).join(", ") + "]");
