// Day 671: Debounce. Each call clears the pending timeout and sets a fresh N-ms one;
// f fires only after N ms of silence. Per-call O(1).
function debounce(f, n) {
  let t;
  return (...args) => {
    clearTimeout(t);
    t = setTimeout(() => f(...args), n);
  };
}

const g = debounce((s) => console.log("f called with:", s), 50);
for (const s of ["a", "b", "c", "d", "e"]) g(s); // rapid burst
// Node's event loop waits for the pending timer; prints once: f called with: e
