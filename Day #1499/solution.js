// Day 1499: Job scheduler that calls f after n ms using setTimeout.
// Node's event loop keeps the process alive until the timer fires.
// Time O(1) to schedule, Space O(1).
function schedule(f, n) {
  setTimeout(f, n);
}

schedule(() => {
  console.log("Job executed after delay");
}, 100);
