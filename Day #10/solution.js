// Job scheduler: use setTimeout to invoke f after n milliseconds.
// schedule: O(1); the event loop fires the callback at the due time.
function schedule(f, n) {
  return setTimeout(f, n);
}

schedule(() => console.log("Executed after delay!"), 100);
