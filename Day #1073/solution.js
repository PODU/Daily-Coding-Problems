// Job scheduler: schedule f after n ms using setTimeout. O(1) schedule, event loop keeps program alive until done.
function schedule(f, delayMs) {
  return setTimeout(f, delayMs);
}

console.log("Scheduling job...");
schedule(() => {
  console.log("Job executed!");
}, 100);
// Node.js event loop waits for the timer naturally — no extra sleep needed.
