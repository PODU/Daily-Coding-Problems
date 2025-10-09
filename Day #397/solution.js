// Greedy activity selection: sort by end time, pick job if start >= last end (intervals [start,end)).
// Time O(n log n), Space O(n).
function selectJobs(jobs) {
  const sorted = jobs.slice().sort((a, b) => a[1] - b[1]);
  const chosen = [];
  let lastEnd = -Infinity;
  for (const [start, end] of sorted) {
    if (start >= lastEnd) {
      chosen.push([start, end]);
      lastEnd = end;
    }
  }
  return chosen;
}

const jobs = [[0, 6], [1, 4], [3, 5], [3, 8], [4, 7], [5, 9], [6, 10], [8, 11]];
const chosen = selectJobs(jobs);
console.log("[" + chosen.map(([s, e]) => `(${s}, ${e})`).join(", ") + "]");
